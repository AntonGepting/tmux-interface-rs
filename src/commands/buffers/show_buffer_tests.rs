// auto-generated file
//

// Display the contents of the specified buffer.
//
// # Manual
//
// tmux >=2.0:
// ```text
// show-buffer [-b buffer-name]
// (alias: showb)
// ```
//
// tmux >=1.5:
// ```text
// show-buffer [-b buffer-index]
// (alias: showb)
// ```
//
// tmux >=0.8:
// ```text
// show-buffer [-b buffer-index] [-t target-session]
// (alias: showb)
// ```
#[test]
fn show_buffer() {
    use crate::{ShowBuffer, TargetPane};
    use std::borrow::Cow;

    let target_pane = TargetPane::Raw("5").to_string();

    let show_buffer = ShowBuffer::new();
    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let show_buffer = show_buffer.buffer_index("1");

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    let show_buffer = show_buffer.buffer_name("2");

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let show_buffer = show_buffer.target_session("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showb";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    v.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    v.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let show_buffer = show_buffer.build().to_vec();

    assert_eq!(show_buffer, v);
}
