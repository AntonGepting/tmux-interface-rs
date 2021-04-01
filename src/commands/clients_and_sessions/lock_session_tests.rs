#[test]
fn lock_session() {
    use crate::LockSession;
    use std::borrow::Cow;

    // Lock all clients attached to `target-session`
    // # Manual
    //
    // tmux ^1.1:
    // ```text
    // tmux lock-session [-t target-session]
    // (alias: locks)
    // ```
    let mut lock_session = LockSession::new();
    #[cfg(feature = "tmux_1_1")]
    lock_session.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "locks";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_1")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(lock_session.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(lock_session.0.bin_args, None);
    assert_eq!(lock_session.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(lock_session.0.cmd_args, Some(s));
}
