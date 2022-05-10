#[test]
fn next_window() {
    use crate::NextWindow;
    use std::borrow::Cow;

    // Move to the next window in the session
    //
    // # Manual
    //
    // tmux ^0.9:
    // ```text
    // tmux next-window [-a] [-t target-session]
    // (alias: next)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux next-window [-t target-session]
    // (alias: next)
    // ```
    let mut next_window = NextWindow::new();
    next_window.attach();
    #[cfg(feature = "tmux_0_8")]
    next_window.target_window("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "next-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "next";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_9")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let next_window = next_window.build().to_vec();

    assert_eq!(next_window, s);
}
