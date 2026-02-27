// auto-generated file
//

// Save the contents of the specified paste buffer to path.
//
// # Manual
//
// tmux >=2.0:
// ```text
// save-buffer [-a] [-b buffer-name] path
// (alias: saveb)
// ```
//
// tmux >=1.5:
// ```text
// save-buffer [-a] [-b buffer-index] path
// (alias: saveb)
// ```
//
// tmux >=0.8:
// ```text
// save-buffer [-a] [-b buffer-index] [-t target-session] path
// (alias: saveb)
// ```
#[test]
fn save_buffer() {
    use crate::{SaveBuffer, TargetPane};
    use std::borrow::Cow;

    let target_pane = TargetPane::Raw("5").to_string();

    let save_buffer = SaveBuffer::new();
    /// `[-a]`
    #[cfg(feature = "tmux_0_8")]
    let save_buffer = save_buffer.append();

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let save_buffer = save_buffer.buffer_index("1");

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    let save_buffer = save_buffer.buffer_name("2");

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let save_buffer = save_buffer.target_session("3");

    /// `[path]`
    #[cfg(feature = "tmux_0_8")]
    let save_buffer = save_buffer.path("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "save-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "saveb";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.push("-a");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    v.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    v.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("4");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let save_buffer = save_buffer.build().to_vec();

    assert_eq!(save_buffer, v);
}
