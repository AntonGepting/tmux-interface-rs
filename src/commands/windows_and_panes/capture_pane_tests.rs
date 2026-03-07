// auto-generated file
//

// Capture the contents of a pane
//
// # Manual
//
// tmux >=3.6:
// ```text
// capture-pane [-aepPqCJMNT] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
// (alias: capturep)
// ```
//
// tmux >=3.4:
// ```text
// capture-pane [-aAepPqCJNT] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
// (alias: capturep)
// ```
//
// tmux >=3.1:
// ```text
// capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
// (alias: capturep)
// ```
//
// tmux >=2.4:
// ```text
// capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
// (alias: capturep)
// ```
//
// tmux >=1.8:
// ```text
// capture-pane [-aepPq] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
// (alias: capturep)
// ```
//
// tmux >=1.5:
// ```text
// capture-pane [-b buffer-index] [-E end-line] [-S start-line] [-t target-pane]
// (alias: capturep)
// ```
#[test]
fn capture_pane() {
    use crate::CapturePane;
    use std::borrow::Cow;

    let capture_pane = CapturePane::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.alternate_screen();

    // `[-A]`
    #[cfg(feature = "tmux_3_4")]
    let capture_pane = capture_pane.a();

    // `[-e]`
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.escape_sequences();

    // `[-p]`
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.stdout();

    // `[-P]`
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.pane();

    // `[-q]`
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane.quiet();

    // `[-C]`
    #[cfg(feature = "tmux_2_4")]
    let capture_pane = capture_pane.escape_non_printable();

    // `[-J]`
    #[cfg(feature = "tmux_2_4")]
    let capture_pane = capture_pane.join();

    // `[-M]`
    #[cfg(feature = "tmux_3_6")]
    let capture_pane = capture_pane.screen_for_mode();

    // `[-N]`
    #[cfg(feature = "tmux_3_1")]
    let capture_pane = capture_pane.trailing_spaces();

    // `[-b buffer-index]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    let capture_pane = capture_pane.buffer_index("1");

    // `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    let capture_pane = capture_pane.buffer_name("2");

    // `[-E end-line]`
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane.end_line("3");

    // `[-S start-line]`
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane.start_line("4");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane.target_pane("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "capture-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "capturep";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    v.push("-a");
    #[cfg(feature = "tmux_3_4")]
    v.push("-A");
    #[cfg(feature = "tmux_1_8")]
    v.push("-e");
    #[cfg(feature = "tmux_1_8")]
    v.push("-p");
    #[cfg(feature = "tmux_1_8")]
    v.push("-P");
    #[cfg(feature = "tmux_1_8")]
    v.push("-q");
    #[cfg(feature = "tmux_2_4")]
    v.push("-C");
    #[cfg(feature = "tmux_2_4")]
    v.push("-J");
    #[cfg(feature = "tmux_3_6")]
    v.push("-M");
    #[cfg(feature = "tmux_3_1")]
    v.push("-N");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    v.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    v.extend_from_slice(&["-b", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-E", "3"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-S", "4"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "5"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let capture_pane = capture_pane.build().to_vec();

    assert_eq!(capture_pane, v);
}
