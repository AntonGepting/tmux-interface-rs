// auto-generated file
//

// Suspend a client by sending SIGTSTP (tty stop)
//
// # Manual
//
// tmux >=1.5:
// ```text
// suspend-client [-t target-client]
// (alias: suspendc)
// ```
//
// tmux >=0.8:
// ```text
// suspend-client [-c target-client]
// (alias: suspendc)
// ```
#[test]
fn suspend_client() {
    use crate::SuspendClient;
    use std::borrow::Cow;

    let suspend_client = SuspendClient::new();
    // `[-c target-client]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let suspend_client = suspend_client.target_client("1");

    // `[-t target-client]`
    #[cfg(feature = "tmux_1_5")]
    let suspend_client = suspend_client.target_client("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "suspend-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "suspendc";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "2"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let suspend_client = suspend_client.build().to_vec();

    assert_eq!(suspend_client, v);
}
