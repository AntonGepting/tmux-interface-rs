#[test]
fn last_pane() {
    use crate::{LastPane, TargetWindow};
    use std::borrow::Cow;

    // Select the last (previously selected) pane
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // tmux last-pane [-deZ] [-t target-window]
    // (alias: lastp)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // tmux last-pane [-de] [-t target-window]
    // (alias: lastp)
    // ```
    //
    // tmux ^1.4:
    // ```text
    // tmux last-pane [-t target-window]
    // (alias: lastp)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let mut last_pane = LastPane::new();
    #[cfg(feature = "tmux_2_0")]
    last_pane.disable();
    #[cfg(feature = "tmux_2_0")]
    last_pane.enable();
    #[cfg(feature = "tmux_3_1")]
    last_pane.keep_zoomed();
    #[cfg(feature = "tmux_1_4")]
    last_pane.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "last-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lastp";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_0")]
    s.push("-d");
    #[cfg(feature = "tmux_2_0")]
    s.push("-e");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_1_4")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(last_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(last_pane.0.bin_args, None);
    assert_eq!(last_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(last_pane.0.cmd_args, Some(s));
}
