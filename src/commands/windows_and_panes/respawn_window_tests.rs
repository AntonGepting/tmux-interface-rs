#[test]
fn respawn_window() {
    use crate::{Error, RespawnWindow, RespawnWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // tmux respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
        // [shell-command]
        // (alias: respawnw)
        //
        // tmux ^2.6:
        // ```text
        // tmux respawn-window [-k] [-c start-directory] [-t target-window]
        // [shell-command]
        // (alias: respawnw)
        //
        // tmux ^1.2:
        // ```text
        // tmux respawn-window [-k] [-t target-window] [shell-command]
        // (alias: respawnw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux respawn-window [-k] [-t target-window] [command]
        // (alias: respawnw)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("respawn-window");
        #[cfg(feature = "tmux_0_8")]
        s.push("-k");
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-c", "1"]);
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-e", "2"]);
        #[cfg(feature = "tmux_0_9")]
        s.extend_from_slice(&["-t", "3"]);
        #[cfg(feature = "tmux_1_2")]
        s.push("4");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_window = TargetWindow::Raw("3").to_string();

    let respawn_window = RespawnWindow {
        #[cfg(feature = "tmux_0_8")]
        kill: Some(true),
        #[cfg(feature = "tmux_2_6")]
        start_directory: Some("1"),
        #[cfg(feature = "tmux_3_0")]
        environment: Some("2"),
        #[cfg(feature = "tmux_0_9")]
        target_window: Some(&target_window),
        #[cfg(feature = "tmux_1_2")]
        shell_command: Some("4"),
    };
    tmux.respawn_window(Some(&respawn_window)).unwrap_err();

    let mut builder = RespawnWindowBuilder::new();
    #[cfg(feature = "tmux_0_8")]
    builder.kill();
    #[cfg(feature = "tmux_2_6")]
    builder.start_directory("1");
    #[cfg(feature = "tmux_3_0")]
    builder.environment("2");
    #[cfg(feature = "tmux_0_9")]
    builder.target_window(&target_window);
    #[cfg(feature = "tmux_1_2")]
    builder.shell_command("4");
    let respawn_window = builder.build();
    tmux.respawn_window(Some(&respawn_window)).unwrap_err();
}
