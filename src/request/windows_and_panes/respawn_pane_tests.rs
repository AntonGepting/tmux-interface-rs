#[test]
fn respawn_pane() {
    use crate::{Error, RespawnPane, RespawnPaneBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
        // (alias: respawnp)
        // ```
        //
        // tmux ^2.6:
        // ```text
        // tmux respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
        // (alias: respawnp)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux respawn-pane [-k] [-t target-pane] [shell-command]
        // (alias: respawnp)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("respawn-pane");
        #[cfg(feature = "tmux_1_5")]
        s.push("-k");
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-c", "1"]);
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-e", "2"]);
        #[cfg(feature = "tmux_1_5")]
        s.extend_from_slice(&["-t", "3"]);
        #[cfg(feature = "tmux_2_6")]
        s.push("4");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_pane = TargetPane::Raw("3").to_string();
    let respawn_pane = RespawnPane {
        #[cfg(feature = "tmux_1_5")]
        kill: Some(true),
        #[cfg(feature = "tmux_2_6")]
        start_directory: Some("1"),
        #[cfg(feature = "tmux_3_0")]
        environment: Some("2"),
        #[cfg(feature = "tmux_1_5")]
        target_pane: Some(&target_pane),
        #[cfg(feature = "tmux_2_6")]
        shell_command: Some("4"),
    };
    tmux.respawn_pane(Some(&respawn_pane)).unwrap_err();

    let mut builder = RespawnPaneBuilder::new();
    #[cfg(feature = "tmux_1_5")]
    builder.kill();
    #[cfg(feature = "tmux_2_6")]
    builder.start_directory("1");
    #[cfg(feature = "tmux_3_0")]
    builder.environment("2");
    #[cfg(feature = "tmux_1_5")]
    builder.target_pane(&target_pane);
    #[cfg(feature = "tmux_2_6")]
    builder.shell_command("4");
    let respawn_pane = builder.build();
    tmux.respawn_pane(Some(&respawn_pane)).unwrap_err();
}
