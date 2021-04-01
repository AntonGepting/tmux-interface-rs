#[test]
fn load_buffer() {
    use crate::LoadBuffer;
    use std::borrow::Cow;

    // Load the contents of the specified paste buffer from path.
    //
    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // tmux load-buffer [-b buffer-name] path
    // (alias: loadb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux load-buffer [-b buffer-index] path
    // (alias: loadb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux load-buffer [-b buffer-index] [-t target-session] path
    // (alias: loadb)
    // ```
    let mut load_buffer = LoadBuffer::new();
    #[cfg(feature = "tmux_2_0")]
    load_buffer.buffer_name("1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    load_buffer.buffer_index("1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    load_buffer.target_session("1");
    #[cfg(feature = "tmux_0_8")]
    load_buffer.path("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "load-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "loadb";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(load_buffer.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(load_buffer.0.bin_args, None);
    assert_eq!(load_buffer.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(load_buffer.0.cmd_args, Some(s));
}
