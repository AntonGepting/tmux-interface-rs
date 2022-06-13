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

    let next_layout = NextLayout::new();
    #[cfg(feature = "tmux_0_8")]
    let next_layout = next_layout.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "next-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "nextl";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let next_layout = next_layout.build().to_vec();

    assert_eq!(next_layout, s);
}
