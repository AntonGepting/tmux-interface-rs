#[test]
fn respawn_pane() {
    use crate::{RespawnPane, TargetPane};
    use std::borrow::Cow;

    // Reactivate a pane in which the command has exited
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
    // (alias: respawnp)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
    // (alias: respawnp)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // respawn-pane [-k] [-t target-pane] [shell-command]
    // (alias: respawnp)
    // ```
    let target_pane = TargetPane::Raw("3").to_string();

    let respawn_pane = RespawnPane::new();
    #[cfg(feature = "tmux_1_5")]
    let respawn_pane = respawn_pane.kill();
    #[cfg(feature = "tmux_2_6")]
    let respawn_pane = respawn_pane.start_directory("1");
    #[cfg(feature = "tmux_3_0")]
    let respawn_pane = respawn_pane.environment("2");
    #[cfg(feature = "tmux_1_5")]
    let respawn_pane = respawn_pane.target_pane(&target_pane);
    #[cfg(feature = "tmux_2_6")]
    let respawn_pane = respawn_pane.shell_command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "respawn-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "respawnp";

    let mut s = Vec::new();
    s.push(cmd);
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
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let respawn_pane = respawn_pane.build().to_vec();

    assert_eq!(respawn_pane, s);
}
