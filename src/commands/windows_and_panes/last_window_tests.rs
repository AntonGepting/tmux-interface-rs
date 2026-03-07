// auto-generated file
//

// Select the last (previously selected) window
//
// # Manual
//
// tmux >=0.8:
// ```text
// last-window [-t target-session]
// (alias: last)
// ```
#[test]
fn last_window() {
    use crate::LastWindow;
    use std::borrow::Cow;

    let last_window = LastWindow::new();
    // `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    let last_window = last_window.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "last-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "last";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let last_window = last_window.build().to_vec();

    assert_eq!(last_window, v);
}
