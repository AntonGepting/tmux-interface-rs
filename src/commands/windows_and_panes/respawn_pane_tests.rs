// auto-generated file
//

// Reactivate a pane in which the command has exited
//
// # Manual
//
// tmux >=3.0:
// ```text
// respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
// (alias: respawnp)
// ```
//
// tmux >=2.6:
// ```text
// respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
// (alias: respawnp)
// ```
//
// tmux >=1.5:
// ```text
// respawn-pane [-k] [-t target-pane] [shell-command]
// (alias: respawnp)
// ```
#[test]
fn respawn_pane() {
    use crate::RespawnPane;
    use std::borrow::Cow;

    let respawn_pane = RespawnPane::new();
    // `[-k]`
    #[cfg(feature = "tmux_1_5")]
    let respawn_pane = respawn_pane.kill();

    // `[-c start-directory]`
    #[cfg(feature = "tmux_2_6")]
    let respawn_pane = respawn_pane.start_directory("1");

    // `[-e environment]`
    #[cfg(feature = "tmux_3_0")]
    let respawn_pane = respawn_pane.environment("2");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let respawn_pane = respawn_pane.target_pane("3");

    // `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    let respawn_pane = respawn_pane.shell_command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "respawn-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "respawnp";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-k");
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_0")]
    v.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("4");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let respawn_pane = respawn_pane.build().to_vec();

    assert_eq!(respawn_pane, v);
}
