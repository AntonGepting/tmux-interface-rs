#[test]
fn show_window_options() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // (removed)
        // ```
        //
        // tmux ^1.8:
        // ```text
        // tmux show-window-options [-gv] [-t target-window] [option]
        // (alias: showw)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux show-window-options [-g] [-t target-window] [option]
        // (alias: showw)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux show-window-options [-g] [-t target-window]
        // (alias: showw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux show-window-options [-t target-window] option value
        // (alias: showw)
        // ```
        // (alias: showw)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("show-window-options");
        #[cfg(feature = "use_cmd_alias")]
        s.push("showw");
        #[cfg(feature = "tmux_1_0")]
        s.push("-g");
        #[cfg(feature = "tmux_1_8")]
        s.push("-v");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(feature = "tmux_0_8")]
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_window = TargetWindow::Raw("1").to_string();
    tmux.show_window_options(Some(true), Some(true), Some(&target_window), Some("2"))
        .unwrap_err();
}
