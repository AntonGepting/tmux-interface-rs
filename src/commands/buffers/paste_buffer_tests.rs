#[test]
fn paste_buffer() {
    use crate::{PasteBuffer, TargetPane};
    use std::borrow::Cow;

    // Structure for inserting the contents of a paste buffer into the specified pane
    //
    // # Manual
    //
    // tmux ^1.7:
    // ```text
    // tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
    // (alias: pasteb)
    // ```
    //
    // tmux ^1.3:
    // ```text
    // tmux paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-window]
    // (alias: pasteb)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux paste-buffer [-dr] [-b buffer-index] [-t target-window]
    // (alias: pasteb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux paste-buffer [-d] [-b buffer-index] [-t target-window]
    // (alias: pasteb)
    // ```
    let target_pane = TargetPane::Raw("3").to_string();
    let mut paste_buffer = PasteBuffer::new();
    #[cfg(feature = "tmux_0_8")]
    paste_buffer.delete();
    #[cfg(feature = "tmux_1_7")]
    paste_buffer.bracket_codes();
    #[cfg(feature = "tmux_1_0")]
    paste_buffer.no_replacement();
    #[cfg(feature = "tmux_1_7")]
    paste_buffer.buffer_name("1");
    #[cfg(feature = "tmux_1_3")]
    paste_buffer.separator("2");
    #[cfg(feature = "tmux_1_7")]
    paste_buffer.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    paste_buffer.target_window(&target_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "paste-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "pasteb";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_7")]
    s.push("-p");
    #[cfg(feature = "tmux_1_0")]
    s.push("-r");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_1_3")]
    s.extend_from_slice(&["-s", "2"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    s.extend_from_slice(&["-t", "3"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(paste_buffer.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(paste_buffer.0.bin_args, None);
    assert_eq!(paste_buffer.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(paste_buffer.0.cmd_args, Some(s));
}
