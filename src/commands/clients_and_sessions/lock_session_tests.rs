#[test]
fn lock_session() {
    use crate::LockSession;
    use std::borrow::Cow;

    // Lock all clients attached to `target-session`
    // # Manual
    //
    // tmux ^1.1:
    // ```text
    // lock-session [-t target-session]
    // (alias: locks)
    // ```
    let lock_session = LockSession::new();
    #[cfg(feature = "tmux_1_1")]
    let lock_session = lock_session.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "locks";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_1")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let lock_session = lock_session.build().to_vec();

    assert_eq!(lock_session, s);
}
