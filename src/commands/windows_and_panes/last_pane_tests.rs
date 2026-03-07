// auto-generated file
//

// Select the last (previously selected) pane
//
// # Manual
//
// tmux >=3.1:
// ```text
// last-pane [-deZ] [-t target-window]
// (alias: lastp)
// ```
//
// tmux >=2.0:
// ```text
// last-pane [-de] [-t target-window]
// (alias: lastp)
// ```
//
// tmux >=1.5:
// ```text
// last-pane [-t target-window]
// (alias: lastp)
// ```
#[test]
fn last_pane() {
    use crate::LastPane;
    use std::borrow::Cow;

    let last_pane = LastPane::new();
    // `[-d]`
    #[cfg(feature = "tmux_2_0")]
    let last_pane = last_pane.disable();

    // `[-e]`
    #[cfg(feature = "tmux_2_0")]
    let last_pane = last_pane.enable();

    // `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    let last_pane = last_pane.keep_zoomed();

    // `[-t target-window]`
    #[cfg(feature = "tmux_1_5")]
    let last_pane = last_pane.target_window("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "last-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lastp";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_0")]
    v.push("-d");
    #[cfg(feature = "tmux_2_0")]
    v.push("-e");
    #[cfg(feature = "tmux_3_1")]
    v.push("-Z");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let last_pane = last_pane.build().to_vec();

    assert_eq!(last_pane, v);
}
