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
    let previous_window = PreviousWindow::new();
    #[cfg(feature = "tmux_0_9")]
    let previous_window = previous_window.parent_sighup();
    #[cfg(feature = "tmux_0_8")]
    let previous_window = previous_window.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "previous-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "prev";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_9")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let previous_window = previous_window.build().to_vec();

    assert_eq!(previous_window, s);
}
