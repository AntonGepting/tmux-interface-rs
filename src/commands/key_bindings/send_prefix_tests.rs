#[test]
fn send_prefix() {
    use crate::SendPrefix;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^1.6
    // ```text
    // tmux send-prefix [-2] [-t target-pane]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux send-prefix [-t target-pane]
    // ```
    let mut send_prefix = SendPrefix::new();
    #[cfg(feature = "tmux_1_6")]
    send_prefix.secondary();
    #[cfg(feature = "tmux_0_8")]
    send_prefix.target_pane("1");

    let cmd = "send-prefix";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_6")]
    s.push("-2");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(send_prefix.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(send_prefix.0.bin_args, None);
    assert_eq!(send_prefix.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(send_prefix.0.cmd_args, Some(s));
}
