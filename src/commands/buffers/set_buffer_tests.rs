#[test]
fn set_buffer() {
    use crate::SetBuffer;
    use std::borrow::Cow;

    // Set the contents of the specified buffer to data.
    //
    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // tmux set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
    // (alias: setb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux set-buffer [-b buffer-index] data
    // (alias: setb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux set-buffer [-b buffer-index] [-t target-session] data
    // (alias: setb)
    // ```
    let mut set_buffer = SetBuffer::new();
    #[cfg(feature = "tmux_2_0")]
    set_buffer.append();
    #[cfg(feature = "tmux_2_0")]
    set_buffer.buffer_name("1");
    #[cfg(feature = "tmux_2_0")]
    set_buffer.new_buffer_name("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    set_buffer.buffer_index("3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    set_buffer.target_session("4");
    #[cfg(feature = "tmux_0_8")]
    set_buffer.data("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setb";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_0")]
    s.push("-a");
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-n", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("5");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(set_buffer.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(set_buffer.0.bin_args, None);
    assert_eq!(set_buffer.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(set_buffer.0.cmd_args, Some(s));
}
