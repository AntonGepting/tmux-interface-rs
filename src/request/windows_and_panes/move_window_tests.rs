#[test]
fn move_window() {
    use crate::{Error, MoveWindow, MoveWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.1:
        // ```text
        // tmux move-window [-ardk] [-s src-window] [-t dst-window]
        // (alias: movew)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux move-window [-rdk] [-s src-window] [-t dst-window]
        // (alias: movew)
        // ```
        //
        // tmux ^1.3:
        // ```text
        // tmux move-window [-dk] [-s src-window] [-t dst-window]
        // (alias: movew)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux move-window [-d] [-s src-window] [-t dst-window]
        // (alias: movew)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("move-window");
        #[cfg(feature = "tmux_2_1")]
        s.push("-a");
        #[cfg(feature = "tmux_1_7")]
        s.push("-r");
        #[cfg(feature = "tmux_0_8")]
        s.push("-d");
        #[cfg(feature = "tmux_1_3")]
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

    let move_window = MoveWindow {
        #[cfg(feature = "tmux_2_1")]
        add: Some(true),
        #[cfg(feature = "tmux_1_7")]
        renumber: Some(true),
        #[cfg(feature = "tmux_0_8")]
        detached: Some(true),
        #[cfg(feature = "tmux_1_3")]
        kill: Some(true),
        #[cfg(feature = "tmux_0_8")]
        src_window: Some(&TargetWindow::Raw("1")),
        #[cfg(feature = "tmux_0_8")]
        dst_window: Some(&TargetWindow::Raw("2")),
    };
    tmux.move_window(Some(&move_window)).unwrap_err();

    let mut builder = MoveWindowBuilder::new();
    #[cfg(feature = "tmux_2_1")]
    builder.add();
    #[cfg(feature = "tmux_1_7")]
    builder.renumber();
    #[cfg(feature = "tmux_0_8")]
    builder.detached();
    #[cfg(feature = "tmux_1_3")]
    builder.kill();
    #[cfg(feature = "tmux_0_8")]
    builder.src_window(&TargetWindow::Raw("1"));
    #[cfg(feature = "tmux_0_8")]
    builder.dst_window(&TargetWindow::Raw("2"));
    builder.build();
    tmux.move_window(Some(&move_window)).unwrap_err();
}
