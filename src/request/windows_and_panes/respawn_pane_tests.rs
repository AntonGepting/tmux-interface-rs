#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn respawn_pane() {
    use crate::{Error, RespawnPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
        // (alias: respawnp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["respawn-pane", "-k", "-c", "1", "-e", "2", "-t", "3", "4"]"#
        );
        Err(Error::new("hook"))
    }));
    let respawn_pane = RespawnPane {
        kill: Some(true),
        start_directory: Some("1"),
        environment: Some("2"),
        target_pane: Some("3"),
        shell_command: Some("4"),
    };
    tmux.respawn_pane(Some(&respawn_pane)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn respawn_pane() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
        // (alias: respawnp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["respawn-pane", "-k", "-c", "1", "-t", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.respawn_pane(Some(true), Some("1"), Some("2"), Some("3"))
        .unwrap_err();
}