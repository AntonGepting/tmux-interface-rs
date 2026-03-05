// auto-generated file
//

// Lock `target-client`
//
// # Manual
//
// tmux >=1.5:
// ```text
// lock-client [-t target-client]
// (alias: lockc)
// ```
#[test]
fn lock_client() {
    use crate::LockClient;
    use std::borrow::Cow;

    let lock_client = LockClient::new();
    // `[-t target-client]`
    #[cfg(feature = "tmux_1_5")]
    let lock_client = lock_client.target_client("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lockc";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let lock_client = lock_client.build().to_vec();

    assert_eq!(lock_client, v);
}
