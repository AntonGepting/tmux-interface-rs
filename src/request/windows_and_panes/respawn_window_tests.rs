#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn respawn_window() {
    use crate::{Error, RespawnWindow, RespawnWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
        // [shell-command]
        // (alias: respawnw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["respawn-window", "-k", "-c", "1", "-e", "2", "-t", "3", "4"]"#
        );
        Err(Error::new("hook"))
    }));

    let respawn_window = RespawnWindow {
        kill: Some(true),
        start_directory: Some("1"),
        environment: Some("2"),
        target_window: Some(&TargetWindow::Raw("3")),
        shell_command: Some("4"),
    };
    tmux.respawn_window(Some(&respawn_window)).unwrap_err();

    let respawn_window = RespawnWindowBuilder::new()
        .kill()
        .start_directory("1")
        .environment("2")
        .target_window(&TargetWindow::Raw("3"))
        .shell_command("4")
        .build();
    tmux.respawn_window(Some(&respawn_window)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn respawn_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux respawn-window [-k] [-c start-directory] [-t target-window] [shell-command]
        // (alias: respawnw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["respawn-window", "-k", "-c", "1", "-t", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.respawn_window(
        Some(true),
        Some("1"),
        Some(&TargetWindow::Raw("2")),
        Some("3"),
    )
    .unwrap_err();
}
