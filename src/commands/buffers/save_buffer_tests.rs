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
    // save-buffer [-a] [-b buffer-name] path
    // (alias: saveb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // save-buffer [-a] [-b buffer-index] path
    // (alias: saveb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // save-buffer [-a] [-b buffer-index] [-t target-session] path
    // (alias: saveb)
    // ```
    let save_buffer = SaveBuffer::new();
    #[cfg(feature = "tmux_0_8")]
    let save_buffer = save_buffer.append();
    #[cfg(feature = "tmux_2_0")]
    let save_buffer = save_buffer.buffer_name("1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let save_buffer = save_buffer.buffer_index("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let save_buffer = save_buffer.target_session("3");
    #[cfg(feature = "tmux_0_8")]
    let save_buffer = save_buffer.path("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "save-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "saveb";

    let mut s = Vec::new();
    s.push(cmd);
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
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let save_buffer = save_buffer.build().to_vec();

    assert_eq!(save_buffer, s);
}
