// auto-generated file
//

// Move to the next window in the session
//
// # Manual
//
// tmux >=1.5:
// ```text
// next-window [-a] [-t target-session]
// (alias: next)
// ```
//
// tmux >=0.8:
// ```text
// next-window [-t target-session]
// (alias: next)
// ```
#[test]
fn next_window() {
    use crate::NextWindow;
    use std::borrow::Cow;

    let next_window = NextWindow::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_5")]
    let next_window = next_window.attach();

    // `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    let next_window = next_window.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "next-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "next";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-a");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let next_window = next_window.build().to_vec();

    assert_eq!(next_window, v);
}
