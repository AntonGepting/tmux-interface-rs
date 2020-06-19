#[test]
fn new_window() {
    use crate::{Error, NewWindow, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name] [-t
        // target-window] [shell-command]
        // (alias: neww)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
        // [shell-command]
        // (alias: neww)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux new-window [-adkP] [-n window-name] [-t target-window] [shell-command]
        // (alias: neww)
        // ```
        //
        // tmux ^1.3:
        // ```text
        // tmux new-window [-adk] [-n window-name] [-t target-window] [shell-command]
        // (alias: neww)
        // ```
        //
        // tmux ^1.2:
        // ```text
        // tmux new-window [-dk] [-n window-name] [-t target-window] [shell-command]
        // (alias: neww)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux new-window [-dk] [-n window-name] [-t target-window] [command]
        // (alias: neww)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux new-window [-d] [-n window-name] [-t target-window] [command]
        // (alias: neww)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("new-window");
        #[cfg(feature = "tmux_1_3")]
        s.push("-a");
        #[cfg(feature = "tmux_0_8")]
        s.push("-d");
        #[cfg(feature = "tmux_1_0")]
        s.push("-k");
        #[cfg(feature = "tmux_1_5")]
        s.push("-P");
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-c", "1"]);
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-e", "2"]);
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-F", "3"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-n", "4"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "5"]);
        #[cfg(feature = "tmux_1_2")]
        s.push("6");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_window = TargetWindow::Raw("5").to_string();
    let new_window = NewWindow {
        #[cfg(feature = "tmux_1_3")]
        add: Some(true),
        #[cfg(feature = "tmux_0_8")]
        detached: Some(true),
        #[cfg(feature = "tmux_1_0")]
        kill: Some(true),
        #[cfg(feature = "tmux_1_5")]
        print: Some(true),
        #[cfg(feature = "tmux_1_7")]
        cwd: Some("1"),
        #[cfg(feature = "tmux_3_0")]
        environment: Some("2"),
        #[cfg(feature = "tmux_1_7")]
        format: Some("3"),
        #[cfg(feature = "tmux_0_8")]
        window_name: Some("4"),
        #[cfg(feature = "tmux_0_8")]
        target_window: Some(&target_window),
        #[cfg(feature = "tmux_1_2")]
        shell_command: Some("6"),
    };
    tmux.new_window(Some(&new_window)).unwrap_err();
}
