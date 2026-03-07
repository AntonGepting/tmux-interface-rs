// auto-generated file
//

// Reactivate a window in which the command has exited
//
// # Manual
//
// tmux >=3.0:
// ```text
// respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
// [shell-command]
// (alias: respawnw)
// ```
//
// tmux >=2.6:
// ```text
// respawn-window [-k] [-c start-directory] [-t target-window]
// [shell-command]
// (alias: respawnw)
// ```
//
// tmux >=1.5:
// ```text
// respawn-window [-k] [-t target-window] [shell-command]
// (alias: respawnw)
// ```
//
// tmux >=0.8:
// ```text
// respawn-window [-k] [-t target-window] [command]
// (alias: respawnw)
// ```
#[test]
fn respawn_window() {
    use crate::RespawnWindow;
    use std::borrow::Cow;

    let respawn_window = RespawnWindow::new();
    // `[-k]`
    #[cfg(feature = "tmux_0_8")]
    let respawn_window = respawn_window.kill();

    // `[-c start-directory]`
    #[cfg(feature = "tmux_2_6")]
    let respawn_window = respawn_window.start_directory("1");

    // `[-e environment]`
    #[cfg(feature = "tmux_3_0")]
    let respawn_window = respawn_window.environment("2");

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let respawn_window = respawn_window.target_window("3");

    // `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    let respawn_window = respawn_window.shell_command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "respawn-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "respawnw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.push("-k");
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_0")]
    v.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("4");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let respawn_window = respawn_window.build().to_vec();

    assert_eq!(respawn_window, v);
}
