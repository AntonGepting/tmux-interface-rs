#[test]
fn link_window() {
    use crate::{Error, LinkWindow, LinkWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.1:
        // ```text
        // tmux link-window [-adk] [-s src-window] [-t dst-window]
        // (alias: linkw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux link-window [-dk] [-s src-window] [-t dst-window]
        // (alias: linkw)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("link-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("linkw");
        #[cfg(feature = "tmux_2_1")]
        s.push("-a");
        #[cfg(feature = "tmux_0_8")]
        s.push("-d");
        #[cfg(feature = "tmux_0_8")]
        s.push("-k");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-s", "1"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "2"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let src_window = TargetWindow::Raw("1").to_string();
    let dst_window = TargetWindow::Raw("2").to_string();

    let link_window = LinkWindow {
        #[cfg(feature = "tmux_2_1")]
        add: Some(true),
        #[cfg(feature = "tmux_0_8")]
        detached: Some(true),
        #[cfg(feature = "tmux_0_8")]
        kill: Some(true),
        #[cfg(feature = "tmux_0_8")]
        src_window: Some(&src_window),
        #[cfg(feature = "tmux_0_8")]
        dst_window: Some(&dst_window),
    };
    tmux.link_window(Some(&link_window)).unwrap_err();

    let mut builder = LinkWindowBuilder::new();
    #[cfg(feature = "tmux_2_1")]
    builder.add();
    #[cfg(feature = "tmux_0_8")]
    builder.detached();
    #[cfg(feature = "tmux_0_8")]
    builder.kill();
    #[cfg(feature = "tmux_0_8")]
    builder.src_window(&src_window);
    #[cfg(feature = "tmux_0_8")]
    builder.dst_window(&dst_window);
    let link_window = builder.build();
    tmux.link_window(Some(&link_window)).unwrap_err();
}
