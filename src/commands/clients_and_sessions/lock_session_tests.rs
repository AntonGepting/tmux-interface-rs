// auto-generated file
//

// Lock all clients attached to `target-session`
// # Manual
//
// tmux >=1.5:
// ```text
// lock-session [-t target-session]
// (alias: locks)
// ```
#[test]
fn lock_session() {
    use crate::LockSession;
    use std::borrow::Cow;

    let lock_session = LockSession::new();
    // `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    let lock_session = lock_session.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "locks";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let lock_session = lock_session.build().to_vec();

    assert_eq!(lock_session, v);
}
