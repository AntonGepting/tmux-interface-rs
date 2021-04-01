#[test]
fn list_sessions() {
    use crate::ListSessions;
    use std::borrow::Cow;

    // List all sessions managed by the server
    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // tmux list-sessions [-F format]
    // (alias: ls)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux list-sessions
    // (alias: ls)
    // ```
    let mut list_sessions = ListSessions::new();
    #[cfg(feature = "tmux_1_6")]
    list_sessions.format("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-sessions";
    #[cfg(feature = "cmd_alias")]
    let cmd = "ls";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(list_sessions.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(list_sessions.0.bin_args, None);
    assert_eq!(list_sessions.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(list_sessions.0.cmd_args, Some(s));
}
