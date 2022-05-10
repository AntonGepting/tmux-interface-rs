#[test]
fn kill_window() {
    use crate::{KillWindow, TargetWindow};
    use std::borrow::Cow;

    // Kill the current window or the window at target-window, removing it from any sessions
    // to which it is linked
    //
    // # Manual
    // tmux ^1.7:
    // ```text
    // tmux kill-window [-a] [-t target-window]
    // (alias: killw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux kill-window [-t target-window]
    // (alias: killw)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();
    let mut kill_window = KillWindow::new();
    #[cfg(feature = "tmux_1_7")]
    kill_window.parent_sighup();
    #[cfg(feature = "tmux_0_8")]
    kill_window.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "kill-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "killw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let kill_window = kill_window.build().to_vec();

    assert_eq!(kill_window, s);
}
