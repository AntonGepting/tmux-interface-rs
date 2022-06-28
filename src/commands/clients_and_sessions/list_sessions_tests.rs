#[test]
fn list_sessions() {
    use crate::ListSessions;
    use std::borrow::Cow;

    // List all sessions managed by the server
    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // list-sessions [-F format]
    // (alias: ls)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // list-sessions
    // (alias: ls)
    // ```
    let list_sessions = ListSessions::new();
    #[cfg(feature = "tmux_1_6")]
    let list_sessions = list_sessions.format("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-sessions";
    #[cfg(feature = "cmd_alias")]
    let cmd = "ls";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_sessions = list_sessions.build().to_vec();

    assert_eq!(list_sessions, s);
}
