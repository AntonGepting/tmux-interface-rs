#[test]
fn delete_buffer() {
    use crate::{DeleteBuffer, TargetPane};
    use std::borrow::Cow;

    // Delete the buffer named buffer-name, or the most recently added automatically named buffer
    // if not specified.
    //
    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // delete-buffer [-b buffer-name]
    // (alias: deleteb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // delete-buffer [-b buffer-index]
    // (alias: deleteb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // delete-buffer [-b buffer-index] [-t target-session]
    // (alias: deleteb)
    // ```
    let buffer_name = TargetPane::Raw("1").to_string();
    let delete_buffer = DeleteBuffer::new();
    #[cfg(feature = "tmux_0_8")]
    let delete_buffer = delete_buffer.buffer_name(buffer_name);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    let delete_buffer = delete_buffer.buffer_index(buffer_name);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let delete_buffer = delete_buffer.target_session(buffer_name);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "delete-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "deleteb";

    let mut s = Vec::new();
    s.push(cmd);

    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let delete_buffer = delete_buffer.build().to_vec();

    assert_eq!(delete_buffer, s);
}
