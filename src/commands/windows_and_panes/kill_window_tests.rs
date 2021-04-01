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
    let mut kill_pane = KillWindow::new();
    #[cfg(feature = "tmux_1_7")]
    kill_pane.parent_sighup();
    #[cfg(feature = "tmux_0_8")]
    kill_pane.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "kill-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "killw";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_7")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(kill_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(kill_pane.0.bin_args, None);
    assert_eq!(kill_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(kill_pane.0.cmd_args, Some(s));
}
