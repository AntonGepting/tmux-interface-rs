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
    // tmux delete-buffer [-b buffer-name]
    // (alias: deleteb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux delete-buffer [-b buffer-index]
    // (alias: deleteb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux delete-buffer [-b buffer-index] [-t target-session]
    // (alias: deleteb)
    // ```
    let buffer_name = TargetPane::Raw("1").to_string();
    let mut delete_buffer = DeleteBuffer::new();
    delete_buffer.buffer_name(buffer_name);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "delete-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "deleteb";

    let mut s = Vec::new();
    s.extend_from_slice(&["-b", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(delete_buffer.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(delete_buffer.0.bin_args, None);
    assert_eq!(delete_buffer.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(delete_buffer.0.cmd_args, Some(s));
}
