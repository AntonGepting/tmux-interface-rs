// auto-generated file
//

// Move to the previous window in the session
//
// # Manual
//
// tmux >=1.5:
// ```text
// previous-window [-a] [-t target-session]
// (alias: prev)
// ```
//
// tmux >=0.8:
// ```text
// previous-window [-t target-session]
// (alias: prev)
// ```
#[test]
fn previous_window() {
    use crate::PreviousWindow;
    use std::borrow::Cow;

    let previous_window = PreviousWindow::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_5")]
    let previous_window = previous_window.parent_sighup();

    // `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    let previous_window = previous_window.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "previous-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "prev";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-a");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let previous_window = previous_window.build().to_vec();

    assert_eq!(previous_window, v);
}
