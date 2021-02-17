#[test]
fn select_window() {
    use crate::{Error, SelectWindow, SelectWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.8:
        // ```text
        // tmux select-window [-lnpT] [-t target-window]
        // (alias: selectw)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux select-window [-lnp] [-t target-window]
        // (alias: selectw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux select-window [-t target-window]
        // (alias: selectw)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("select-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("selectw");
        #[cfg(feature = "tmux_1_5")]
        s.push("-l");
        #[cfg(feature = "tmux_1_5")]
        s.push("-n");
        #[cfg(feature = "tmux_1_5")]
        s.push("-p");
        #[cfg(feature = "tmux_1_8")]
        s.push("-T");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_window = TargetWindow::Raw("1").to_string();

    let select_window = SelectWindow {
        #[cfg(feature = "tmux_1_5")]
        last: Some(true),
        #[cfg(feature = "tmux_1_5")]
        next: Some(true),
        #[cfg(feature = "tmux_1_5")]
        previous: Some(true),
        #[cfg(feature = "tmux_1_8")]
        switch: Some(true),
        #[cfg(feature = "tmux_0_8")]
        target_window: Some(&target_window),
    };
    tmux.select_window(Some(&select_window)).unwrap_err();

    let mut builder = SelectWindowBuilder::new();
    #[cfg(feature = "tmux_1_5")]
    builder.last();
    #[cfg(feature = "tmux_1_5")]
    builder.next();
    #[cfg(feature = "tmux_1_5")]
    builder.previous();
    #[cfg(feature = "tmux_1_8")]
    builder.switch();
    #[cfg(feature = "tmux_0_8")]
    builder.target_window(&target_window);
    let select_window = builder.build();
    tmux.select_window(Some(&select_window)).unwrap_err();
}
