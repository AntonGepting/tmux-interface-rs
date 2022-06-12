#[test]
fn clear_history() {
    use crate::{ClearHistory, TargetPane};
    use std::borrow::Cow;

    // Remove and free the history for the specified pane.
    //
    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // tmux clear-history [-t target-pane]
    // (alias: clearhist)
    // ```
    //
    // tmux ^0.9:
    // ```text
    // tmux clear-history [-p pane-index] [-t target-window]
    // (alias: clearhist)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let clear_history = ClearHistory::new();
    #[cfg(feature = "tmux_1_0")]
    let clear_history = clear_history.target_pane(target_pane);
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    let clear_history = clear_history.pane_index(target_pane);
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    let clear_history = clear_history.target_window(target_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "clear-history";
    #[cfg(feature = "cmd_alias")]
    let cmd = "clearhist";

    let mut s = Vec::new();
    s.push(cmd);

    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-p", "1"]);
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let clear_history = clear_history.build().to_vec();

    assert_eq!(clear_history, s);
}
