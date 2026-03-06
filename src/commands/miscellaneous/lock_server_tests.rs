// auto-generated file
//

// Lock each client individually
//
// # Manual
//
// tmux >=0.8:
// ```text
// lock-server
// (alias: lock)
// ```
#[test]
fn lock_server() {
    use crate::LockServer;
    use std::borrow::Cow;

    let lock_server = LockServer::new();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-server";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lock";

    let mut v = Vec::new();
    v.push(cmd);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let lock_server = lock_server.build().to_vec();

    assert_eq!(lock_server, v);
}
