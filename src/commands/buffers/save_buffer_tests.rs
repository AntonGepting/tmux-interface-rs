#[test]
fn save_buffer() {
    use crate::SaveBuffer;
    use std::borrow::Cow;

    // Save the contents of the specified paste buffer to path.
    //
    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // tmux save-buffer [-a] [-b buffer-name] path
    // (alias: saveb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux save-buffer [-a] [-b buffer-index] path
    // (alias: saveb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux save-buffer [-a] [-b buffer-index] [-t target-session] path
    // (alias: saveb)
    // ```
    let mut save_buffer = SaveBuffer::new();
    #[cfg(feature = "tmux_0_8")]
    save_buffer.append();
    #[cfg(feature = "tmux_2_0")]
    save_buffer.buffer_name("1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    save_buffer.buffer_index("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    save_buffer.target_session("3");
    #[cfg(feature = "tmux_0_8")]
    save_buffer.path("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "save-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "saveb";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.push("-a");
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("4");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(save_buffer.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(save_buffer.0.bin_args, None);
    assert_eq!(save_buffer.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(save_buffer.0.cmd_args, Some(s));
}
