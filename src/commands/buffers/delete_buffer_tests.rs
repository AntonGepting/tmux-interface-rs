// auto-generated file
//

// Delete the buffer named buffer-name, or the most recently added automatically named buffer
// if not specified.
//
// # Manual
//
// tmux >=2.0:
// ```text
// delete-buffer [-b buffer-name]
// (alias: deleteb)
// ```
//
// tmux >=1.5 && <2.0:
// ```text
// delete-buffer [-b buffer-index]
// (alias: deleteb)
// ```
//
// tmux >=0.8:
// ```text
// delete-buffer [-b buffer-index] [-t target-session]
// (alias: deleteb)
// ```
#[test]
fn delete_buffer() {
    use crate::{DeleteBuffer, TargetPane};
    use std::borrow::Cow;

    let target_pane = TargetPane::Raw("5").to_string();

    let delete_buffer = DeleteBuffer::new();
    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let delete_buffer = delete_buffer.buffer_index("1");

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    let delete_buffer = delete_buffer.buffer_name("2");

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let delete_buffer = delete_buffer.target_session("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "delete-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "deleteb";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    v.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    v.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let delete_buffer = delete_buffer.build().to_vec();

    assert_eq!(delete_buffer, v);
}
