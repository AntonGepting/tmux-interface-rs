#[test]
fn respawn_window() {
    use crate::{RespawnWindow, TargetWindow};
    use std::borrow::Cow;

    // Reactivate a window in which the command has exited
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
    // [shell-command]
    // (alias: respawnw)
    //
    // tmux ^2.6:
    // ```text
    // respawn-window [-k] [-c start-directory] [-t target-window]
    // [shell-command]
    // (alias: respawnw)
    //
    // tmux ^1.2:
    // ```text
    // respawn-window [-k] [-t target-window] [shell-command]
    // (alias: respawnw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // respawn-window [-k] [-t target-window] [command]
    // (alias: respawnw)
    // ```
    let target_window = TargetWindow::Raw("3").to_string();

    let respawn_window = RespawnWindow::new();
    #[cfg(feature = "tmux_0_8")]
    let respawn_window = respawn_window.kill();
    #[cfg(feature = "tmux_2_6")]
    let respawn_window = respawn_window.start_directory("1");
    #[cfg(feature = "tmux_3_0")]
    let respawn_window = respawn_window.environment("2");
    #[cfg(feature = "tmux_0_9")]
    let respawn_window = respawn_window.target_window(&target_window);
    #[cfg(feature = "tmux_1_2")]
    let respawn_window = respawn_window.shell_command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "respawn-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "respawnw";

    let mut s = Vec::new();
    s.push(cmd);
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
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let respawn_window = respawn_window.build().to_vec();

    assert_eq!(respawn_window, s);
}
