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

    let mut unlink_window = UnlinkWindow::new();
    #[cfg(feature = "tmux_0_8")]
    unlink_window.detach_other();
    #[cfg(feature = "tmux_0_8")]
    unlink_window.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "unlink-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "unlinkw";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.push("-k");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(unlink_window.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(unlink_window.0.bin_args, None);
    assert_eq!(unlink_window.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(unlink_window.0.cmd_args, Some(s));
}
