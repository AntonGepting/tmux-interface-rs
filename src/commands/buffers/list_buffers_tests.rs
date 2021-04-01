#[test]
fn list_buffers() {
    use crate::ListBuffers;
    use std::borrow::Cow;

    // List the global buffers.
    //
    // # Manual
    //
    // tmux ^1.7:
    // ```text
    // tmux list-buffers [-F format]
    // (alias: lsb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux list-buffers
    // (alias: lsb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux list-buffers [-t target-session]
    // (alias: lsb)
    // ```
    let mut list_buffers = ListBuffers::new();
    #[cfg(feature = "tmux_1_7")]
    list_buffers.format("1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    list_buffers.target_session("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-buffers";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsb";

    #[cfg(any(
        all(feature = "tmux_0_8", not(feature = "tmux_1_5")),
        feature = "tmux_1_7"
    ))]
    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(any(
        all(feature = "tmux_0_8", not(feature = "tmux_1_5")),
        feature = "tmux_1_7"
    ))]
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(list_buffers.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(list_buffers.0.bin_args, None);
    assert_eq!(list_buffers.0.cmd, Some(Cow::Borrowed(cmd)));
    #[cfg(any(
        all(feature = "tmux_0_8", not(feature = "tmux_1_5")),
        feature = "tmux_1_7"
    ))]
    assert_eq!(list_buffers.0.cmd_args, Some(s));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_1_7")))]
    assert_eq!(list_buffers.0.cmd_args, None);
}
