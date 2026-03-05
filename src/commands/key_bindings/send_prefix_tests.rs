// auto-generated file
//

// Send the prefix key
//
// # Manual
//
// tmux >=1.6:
// ```text
// send-prefix [-2] [-t target-pane]
// ```
//
// tmux >=1.5:
// ```text
// send-prefix [-t target-pane]
// ```
//
// tmux >=0.8:
// ```text
// send-prefix [-t target-window]
// ```
#[test]
fn send_prefix() {
    use crate::SendPrefix;
    use std::borrow::Cow;

    let send_prefix = SendPrefix::new();
    // `[-2]`
    #[cfg(feature = "tmux_1_6")]
    let send_prefix = send_prefix.secondary();

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let send_prefix = send_prefix.target_window("1");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let send_prefix = send_prefix.target_pane("2");

    let cmd = "send-prefix";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_6")]
    v.push("-2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "2"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let send_prefix = send_prefix.build().to_vec();

    assert_eq!(send_prefix, v);
}
