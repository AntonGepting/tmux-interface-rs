#[test]
fn last_window() {
    use crate::LastWindow;
    use std::borrow::Cow;

    // Select the last (previously selected) window
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // tmux last-window [-t target-session]
    // (alias: last)
    // ```
    let mut last_window = LastWindow::new();
    #[cfg(feature = "tmux_0_8")]
    last_window.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "last-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "last";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(last_window.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(last_window.0.bin_args, None);
    assert_eq!(last_window.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(last_window.0.cmd_args, Some(s));
}
