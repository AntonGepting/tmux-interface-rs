// auto-generated file
//

// List all sessions managed by the server
//
// # Manual
//
// tmux >=3.2:
// ```text
// list-sessions [-F format] [-f filter]
// (alias: ls)
// ```
//
// tmux >=1.6:
// ```text
// list-sessions [-F format]
// (alias: ls)
// ```
//
// tmux >=0.8:
// ```text
// list-sessions
// (alias: ls)
// ```
#[test]
fn list_sessions() {
    use crate::ListSessions;
    use std::borrow::Cow;

    let list_sessions = ListSessions::new();
    // `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    let list_sessions = list_sessions.format("1");

    // `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    let list_sessions = list_sessions.filter("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-sessions";
    #[cfg(feature = "cmd_alias")]
    let cmd = "ls";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_6")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-f", "2"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let list_sessions = list_sessions.build().to_vec();

    assert_eq!(list_sessions, v);
}
