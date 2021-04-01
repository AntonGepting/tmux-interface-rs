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
    let target_window = TargetWindow::Raw("3").to_string();

    let mut respawn_window = RespawnWindow::new();
    #[cfg(feature = "tmux_0_8")]
    respawn_window.kill();
    #[cfg(feature = "tmux_2_6")]
    respawn_window.start_directory("1");
    #[cfg(feature = "tmux_3_0")]
    respawn_window.environment("2");
    #[cfg(feature = "tmux_0_9")]
    respawn_window.target_window(&target_window);
    #[cfg(feature = "tmux_1_2")]
    respawn_window.shell_command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "respawn-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "respawnw";

    let mut s = Vec::new();
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
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(respawn_window.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(respawn_window.0.bin_args, None);
    assert_eq!(respawn_window.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(respawn_window.0.cmd_args, Some(s));
}
