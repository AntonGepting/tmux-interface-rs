#[test]
fn lock_server() {
    use crate::LockServer;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // tmux lock-server
    // (alias: lock)
    // ```
    let lock_server = LockServer::new();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-server";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lock";

    let mut s = Vec::new();
    s.push(cmd);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let lock_server = lock_server.build().to_vec();

    assert_eq!(lock_server, s);
}
