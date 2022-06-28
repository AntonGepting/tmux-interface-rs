#[test]
fn send_prefix() {
    use crate::SendPrefix;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^1.6
    // ```text
    // send-prefix [-2] [-t target-pane]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // send-prefix [-t target-pane]
    // ```
    let send_prefix = SendPrefix::new();
    #[cfg(feature = "tmux_1_6")]
    let send_prefix = send_prefix.secondary();
    #[cfg(feature = "tmux_0_8")]
    let send_prefix = send_prefix.target_pane("1");

    let cmd = "send-prefix";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_6")]
    s.push("-2");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let send_prefix = send_prefix.build().to_vec();

    assert_eq!(send_prefix, s);
}
