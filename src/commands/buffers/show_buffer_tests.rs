#[test]
fn show_buffer() {
    use crate::ShowBuffer;
    use std::borrow::Cow;

    // Display the contents of the specified buffer.
    //
    // # Manual
    //
    // tmux ^1.5:
    // ```text
    // tmux show-buffer [-b buffer-name]
    // (alias: showb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux show-buffer [-b buffer-index] [-t target-session]
    // (alias: showb)
    // ```
    let mut show_buffer = ShowBuffer::new();
    #[cfg(feature = "tmux_1_5")]
    show_buffer.buffer_name("1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    show_buffer.buffer_index("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    show_buffer.target_session("3");

    let mut s = Vec::new();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showb";

    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-b", "3"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(show_buffer.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(show_buffer.0.bin_args, None);
    assert_eq!(show_buffer.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(show_buffer.0.cmd_args, Some(s));
}
