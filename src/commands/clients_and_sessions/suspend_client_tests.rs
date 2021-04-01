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
    let mut suspend_client = SuspendClient::new();
    suspend_client.target_client("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "suspend-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "suspendc";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-c", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(suspend_client.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(suspend_client.0.bin_args, None);
    assert_eq!(suspend_client.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(suspend_client.0.cmd_args, Some(s));
}
