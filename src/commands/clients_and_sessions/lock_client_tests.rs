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
    let lock_client = LockClient::new();
    #[cfg(feature = "tmux_1_1")]
    let lock_client = lock_client.target_client("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lockc";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_1")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let lock_client = lock_client.build().to_vec();

    assert_eq!(lock_client, s);
}
