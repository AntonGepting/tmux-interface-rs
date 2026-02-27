// auto-generated file
//

// Remove and free the history for the specified pane.
//
// # Manual
//
// tmux ^3.4:
// ```text
// clear-history [-H] [-t target-pane]
// (alias: clearhist)
// ```
//
// tmux ^1.5:
// ```text
// clear-history [-t target-pane]
// (alias: clearhist)
// ```
#[test]
fn clear_history() {
    use crate::{ClearHistory, TargetPane};
    use std::borrow::Cow;

    let target_pane = TargetPane::Raw("5").to_string();

    let clear_history = ClearHistory::new();
    /// `[-H]`
    #[cfg(feature = "tmux_3_4")]
    let clear_history = clear_history.no_hyperlinks();

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let clear_history = clear_history.target_pane("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "clear-history";
    #[cfg(feature = "cmd_alias")]
    let cmd = "clearhist";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_4")]
    v.push("-H");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let clear_history = clear_history.build().to_vec();

    assert_eq!(clear_history, v);
}
