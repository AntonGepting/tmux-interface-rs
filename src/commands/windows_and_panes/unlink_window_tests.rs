#[test]
fn unlink_window() {
    use crate::{TargetWindow, UnlinkWindow};
    use std::borrow::Cow;

    // Unlink `target-window`
    //
    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // tmux unlink-window [-k] [-t target-window]
    // (alias: unlinkw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux unlink-window [-t target-window]
    // (alias: unlinkw)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let unlink_window = UnlinkWindow::new();
    #[cfg(feature = "tmux_0_8")]
    let unlink_window = unlink_window.detach_other();
    #[cfg(feature = "tmux_0_8")]
    let unlink_window = unlink_window.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "unlink-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "unlinkw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-k");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let unlink_window = unlink_window.build().to_vec();

    assert_eq!(unlink_window, s);
}
