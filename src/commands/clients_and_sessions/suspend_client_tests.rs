#[test]
fn suspend_client() {
    use crate::SuspendClient;
    use std::borrow::Cow;

    // Suspend a client by sending SIGTSTP (tty stop)
    //
    // # Manual
    //
    // tmux ^1.5:
    // ```text
    // tmux suspend-client [-t target-client]
    // (alias: suspendc)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux suspend-client [-c target-client]
    // (alias: suspendc)
    // ```
    let suspend_client = SuspendClient::new();
    #[cfg(feature = "tmux_0_8")]
    let suspend_client = suspend_client.target_client("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "suspend-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "suspendc";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-c", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let suspend_client = suspend_client.build().to_vec();

    assert_eq!(suspend_client, s);
}
