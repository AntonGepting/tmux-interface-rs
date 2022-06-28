#[test]
fn capture_pane() {
    use crate::{CapturePane, TargetPane};
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
    // (alias: capturep)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
    // (alias: capturep)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // capture-pane [-aepPq] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
    // (alias: capturep)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // capture-pane [-b buffer-index] [-E end-line] [-S start-line] [-t target-pane]
    // (alias: capturep)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // capture-pane [-b buffer-index] [-t target-pane]
    // (alias: capturep)
    // ```

    #[cfg(feature = "tmux_1_2")]
    let target_pane = TargetPane::Raw("4").to_string();

    let capture_pane = CapturePane::new();
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.alternate_screen();
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.escape_sequences();
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.stdout();
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.pane();
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.quiet();
    #[cfg(feature = "tmux_2_4")]
    let capture_pane = capture_pane.escape_non_printable();
    #[cfg(feature = "tmux_2_4")]
    let capture_pane = capture_pane.join();
    #[cfg(feature = "tmux_3_1")]
    let capture_pane = capture_pane.trailing_spaces();
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.buffer_name("1");
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane.end_line("2");
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane.start_line("3");
    #[cfg(feature = "tmux_1_2")]
    let capture_pane = capture_pane.target_pane(&target_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "capture-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "capturep";

    let mut s = Vec::new();
    s.push(cmd);
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
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let capture_pane = capture_pane.build().to_vec();

    assert_eq!(capture_pane, s);
}
