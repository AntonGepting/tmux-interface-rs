#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn split_window() {
    use crate::{Error, PaneSize, SplitWindow, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
        // [shell-command] [-F format]
        // (alias: splitw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["split-window", "-b", "-d", "-f", "-h", "-I", "-v", "-P", "-c", "1", "-e", "2", "-l", "3", "-t", "4", "5", "-F", "6"]"#
        );
        Err(Error::new("hook"))
    }));
    let split_window = SplitWindow {
        before: Some(true),
        detached: Some(true),
        full: Some(true),
        horizontal: Some(true),
        stdin_forward: Some(true),
        vertical: Some(true),
        print: Some(true),
        cwd: Some("1"),
        environment: Some("2"),
        size: Some(PaneSize::Size(3)),
        target_pane: Some(&TargetPane::Raw("4")),
        shell_command: Some("5"),
        format: Some("6"),
    };
    tmux.split_window(Some(&split_window)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn split_window() {
    use crate::{Error, PaneSize, SplitWindow, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
        // [shell-command] [-F format]
        // (alias: splitw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["split-window", "-b", "-d", "-f", "-h", "-v", "-P", "-c", "1", "-l", "2", "-t", "3", "4", "-F", "5"]"#
        );
        Err(Error::new("hook"))
    }));
    let split_window = SplitWindow {
        before: Some(true),
        detached: Some(true),
        full: Some(true),
        horizontal: Some(true),
        vertical: Some(true),
        print: Some(true),
        cwd: Some("1"),
        size: Some(PaneSize::Size(2)),
        target_pane: Some(&TargetPane::Raw("3")),
        shell_command: Some("4"),
        format: Some("5"),
    };
    tmux.split_window(Some(&split_window)).unwrap_err();
}
