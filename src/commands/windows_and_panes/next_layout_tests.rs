#[test]
fn next_layout() {
    use crate::{NextLayout, TargetWindow};
    use std::borrow::Cow;

    // Move a window to the next layout and rearrange the panes to fit
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // tmux next-layout [-t target-window]
    // (alias: nextl)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let mut next_layout = NextLayout::new();
    #[cfg(feature = "tmux_0_8")]
    next_layout.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "next-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "nextl";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(next_layout.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(next_layout.0.bin_args, None);
    assert_eq!(next_layout.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(next_layout.0.cmd_args, Some(s));
}
