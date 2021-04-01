#[test]
fn lock_client() {
    use crate::LockClient;
    use std::borrow::Cow;

    // Lock `target-client`
    //
    // # Manual
    //
    // tmux ^1.1:
    // ```text
    // tmux lock-client [-t target-client]
    // (alias: lockc)
    // ```
    let mut lock_client = LockClient::new();
    lock_client.target_client("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lockc";

    let mut s = Vec::new();
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(lock_client.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(lock_client.0.bin_args, None);
    assert_eq!(lock_client.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(lock_client.0.cmd_args, Some(s));
}
