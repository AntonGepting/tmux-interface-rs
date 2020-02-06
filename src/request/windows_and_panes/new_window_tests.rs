#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn new_window() {
    use crate::{Error, NewWindow, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format]
        // [-n window-name] [-t target-window] [shell-command]
        // (alias: neww)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["new-window", "-a", "-d", "-k", "-P", "-c", "1", "-e", "2", "-F", "3", "-n", "4", "-t", "5", "6"]"#
        );
        Err(Error::new("hook"))
    }));
    let new_window = NewWindow {
        add: Some(true),
        detached: Some(true),
        kill: Some(true),
        print: Some(true),
        cwd: Some("1"),
        environment: Some("2"),
        format: Some("3"),
        window_name: Some("4"),
        target_window: Some(&TargetWindow::Raw("5")),
        shell_command: Some("6"),
    };
    tmux.new_window(Some(&new_window)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn new_window() {
    use crate::{Error, NewWindow, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux new-window [-adkP] [-c start-directory] [-F format]
        // [-n window-name] [-t target-window] [shell-command]
        // (alias: neww)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["new-window", "-a", "-d", "-k", "-P", "-c", "1", "-F", "2", "-n", "3", "-t", "4", "5"]"#
        );
        Err(Error::new("hook"))
    }));
    let new_window = NewWindow {
        add: Some(true),
        detached: Some(true),
        kill: Some(true),
        print: Some(true),
        cwd: Some("1"),
        format: Some("2"),
        window_name: Some("3"),
        target_window: Some(&TargetWindow::Raw("4")),
        shell_command: Some("5"),
    };
    tmux.new_window(Some(&new_window)).unwrap_err();
}
