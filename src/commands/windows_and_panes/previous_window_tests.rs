#[test]
fn previous_window() {
    use crate::PreviousWindow;
    use std::borrow::Cow;

    // tmux ^0.9:
    // ```text
    // tmux previous-window [-a] [-t target-session]
    // (alias: prev)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux previous-window [-t target-session]
    // (alias: prev)
    // ```
    let mut previous_window = PreviousWindow::new();
    #[cfg(feature = "tmux_0_9")]
    previous_window.parent_sighup();
    #[cfg(feature = "tmux_0_8")]
    previous_window.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "previous-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "prev";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_9")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(previous_window.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(previous_window.0.bin_args, None);
    assert_eq!(previous_window.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(previous_window.0.cmd_args, Some(s));
}
