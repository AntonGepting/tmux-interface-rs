#[test]
fn set_window_option() {
    use crate::{Error, SetWindowOption, SetWindowOptionBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // (removed)
        // ```
        //
        // tmux ^2.6:
        // ```text
        // tmux set-window-option [-aFgoqu] [-t target-window] option value
        // (alias: setw)
        // ```
        //
        // tmux ^1.9:
        // ```text
        // tmux set-window-option [-agoqu] [-t target-window] option value
        // (alias: setw)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux set-window-option [-agqu] [-t target-window] option value
        // (alias: setw)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux set-window-option [-agu] [-t target-window] option value
        // (alias: setw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux set-window-option [-gu] [-t target-window] option value
        // (alias: setw)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("set-window-option");
        #[cfg(feature = "use_cmd_alias")]
        s.push("setw");
        #[cfg(feature = "tmux_1_0")]
        s.push("-a");
        #[cfg(feature = "tmux_2_6")]
        s.push("-F");
        #[cfg(feature = "tmux_0_8")]
        s.push("-g");
        #[cfg(feature = "tmux_1_9")]
        s.push("-o");
        #[cfg(feature = "tmux_1_7")]
        s.push("-q");
        #[cfg(feature = "tmux_0_8")]
        s.push("-u");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "1"]);
        s.push("2");
        s.push("3");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_window = TargetWindow::Raw("1").to_string();
    let set_window_option = SetWindowOption {
        #[cfg(feature = "tmux_1_0")]
        append: Some(true),
        #[cfg(feature = "tmux_2_6")]
        format: Some(true),
        #[cfg(feature = "tmux_0_8")]
        global: Some(true),
        #[cfg(feature = "tmux_1_9")]
        not_overwrite: Some(true),
        #[cfg(feature = "tmux_1_7")]
        quiet: Some(true),
        #[cfg(feature = "tmux_0_8")]
        unset: Some(true),
        #[cfg(feature = "tmux_0_8")]
        target_window: Some(&target_window),
    };
    tmux.set_window_option(Some(&set_window_option), "2", "3")
        .unwrap_err();

    let mut builder = SetWindowOptionBuilder::new();
    #[cfg(feature = "tmux_1_0")]
    builder.append();
    #[cfg(feature = "tmux_2_6")]
    builder.format();
    #[cfg(feature = "tmux_0_8")]
    builder.global();
    #[cfg(feature = "tmux_1_9")]
    builder.not_overwrite();
    #[cfg(feature = "tmux_1_7")]
    builder.quiet();
    #[cfg(feature = "tmux_0_8")]
    builder.unset();
    #[cfg(feature = "tmux_0_8")]
    builder.target_window(&target_window);
    let set_window_option = builder.build();
    tmux.set_window_option(Some(&set_window_option), "2", "3")
        .unwrap_err();
}
