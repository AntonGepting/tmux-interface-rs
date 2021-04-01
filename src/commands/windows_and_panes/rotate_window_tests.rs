#[test]
fn rotate_window() {
    use crate::{RotateWindow, TargetWindow};
    use std::borrow::Cow;

    // Rotate the positions of the panes within a window
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // tmux rotate-window [-DUZ] [-t target-window]
    // (alias: rotatew)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux rotate-window [-DU] [-t target-window]
    // (alias: rotatew)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let mut rotate_window = RotateWindow::new();
    #[cfg(feature = "tmux_0_8")]
    rotate_window.down();
    #[cfg(feature = "tmux_0_8")]
    rotate_window.up();
    #[cfg(feature = "tmux_3_1")]
    rotate_window.keep_zoomed();
    #[cfg(feature = "tmux_0_8")]
    rotate_window.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rotate-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "rotatew";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.push("-D");
    #[cfg(feature = "tmux_0_8")]
    s.push("-U");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(rotate_window.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(rotate_window.0.bin_args, None);
    assert_eq!(rotate_window.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(rotate_window.0.cmd_args, Some(s));
}
