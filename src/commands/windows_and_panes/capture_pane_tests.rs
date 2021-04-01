#[test]
fn capture_pane() {
    use crate::{CapturePane, TargetPane};
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // tmux capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
    // (alias: capturep)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // tmux capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
    // (alias: capturep)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux capture-pane [-aepPq] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
    // (alias: capturep)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux capture-pane [-b buffer-index] [-E end-line] [-S start-line] [-t target-pane]
    // (alias: capturep)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux capture-pane [-b buffer-index] [-t target-pane]
    // (alias: capturep)
    // ```

    #[cfg(feature = "tmux_1_2")]
    let target_pane = TargetPane::Raw("4").to_string();

    let mut capture_pane = CapturePane::new();
    #[cfg(feature = "tmux_1_8")]
    capture_pane.alternate_screen();
    #[cfg(feature = "tmux_1_8")]
    capture_pane.escape_sequences();
    #[cfg(feature = "tmux_1_8")]
    capture_pane.stdout();
    #[cfg(feature = "tmux_1_8")]
    capture_pane.pane();
    #[cfg(feature = "tmux_1_8")]
    capture_pane.quite();
    #[cfg(feature = "tmux_2_4")]
    capture_pane.escape_non_printable();
    #[cfg(feature = "tmux_2_4")]
    capture_pane.join();
    #[cfg(feature = "tmux_3_1")]
    capture_pane.trailing_spaces();
    #[cfg(feature = "tmux_1_8")]
    capture_pane.buffer_name("1");
    #[cfg(feature = "tmux_1_5")]
    capture_pane.end_line("2");
    #[cfg(feature = "tmux_1_5")]
    capture_pane.start_line("3");
    #[cfg(feature = "tmux_1_2")]
    capture_pane.target_pane(&target_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "capture-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "capturep";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_8")]
    s.push("-a");
    #[cfg(feature = "tmux_1_8")]
    s.push("-e");
    #[cfg(feature = "tmux_1_8")]
    s.push("-p");
    #[cfg(feature = "tmux_1_8")]
    s.push("-P");
    #[cfg(feature = "tmux_1_8")]
    s.push("-q");
    #[cfg(feature = "tmux_2_4")]
    s.push("-C");
    #[cfg(feature = "tmux_2_4")]
    s.push("-J");
    #[cfg(feature = "tmux_3_1")]
    s.push("-N");
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-E", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-S", "3"]);
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-t", "4"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(capture_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(capture_pane.0.bin_args, None);
    assert_eq!(capture_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(capture_pane.0.cmd_args, Some(s));
}
