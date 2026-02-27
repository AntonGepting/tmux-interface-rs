// auto-generated file
//

// Structure for inserting the contents of a paste buffer into the specified pane
//
// # Manual
//
// tmux >=1.9:
// ```text
// paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
// (alias: pasteb)
// ```
//
// tmux >=1.5:
// ```text
// paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-pane]
// (alias: pasteb)
// ```
//
// tmux >=0.8:
// ```text
// paste-buffer [-d] [-b buffer-index] [-t target-window]
// (alias: pasteb)
// ```
#[test]
fn paste_buffer() {
    use crate::{PasteBuffer, TargetPane};
    use std::borrow::Cow;

    let target_pane = TargetPane::Raw("5").to_string();

    let paste_buffer = PasteBuffer::new();
    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let paste_buffer = paste_buffer.delete();

    /// `[-p]`
    #[cfg(feature = "tmux_1_7")]
    let paste_buffer = paste_buffer.bracket_codes();

    /// `[-r]`
    #[cfg(feature = "tmux_1_5")]
    let paste_buffer = paste_buffer.no_replacement();

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let paste_buffer = paste_buffer.buffer_index("1");

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    let paste_buffer = paste_buffer.buffer_name("2");

    /// `[-s separator]`
    #[cfg(feature = "tmux_1_5")]
    let paste_buffer = paste_buffer.separator("3");

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let paste_buffer = paste_buffer.target_window("4");

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let paste_buffer = paste_buffer.target_pane("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "paste-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "pasteb";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_1_7")]
    v.push("-p");
    #[cfg(feature = "tmux_1_5")]
    v.push("-r");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    v.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    v.extend_from_slice(&["-b", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-s", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "5"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let paste_buffer = paste_buffer.build().to_vec();

    assert_eq!(paste_buffer, v);
}
