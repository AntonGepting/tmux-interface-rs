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

    assert_eq!(lock_server.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(lock_server.0.bin_args, None);
    assert_eq!(lock_server.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(lock_server.0.cmd_args, None);
}
