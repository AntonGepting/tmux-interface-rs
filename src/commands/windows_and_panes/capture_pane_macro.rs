/// # Manual
///
/// tmux ^3.1:
/// ```text
/// capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux ^2.4:
/// ```text
/// capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux ^1.8:
/// ```text
/// capture-pane [-aepPq] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux ^1.5:
/// ```text
/// capture-pane [-b buffer-index] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux ^1.2:
/// ```text
/// capture-pane [-b buffer-index] [-t target-pane]
/// (alias: capturep)
/// ```
#[macro_export]
macro_rules! capture_pane {
    // `[-a]` - the alternate screen is used, and the history is not accessible
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.alternate_screen()
        }) $($tail)*)
    }};
    // `[-e]` - the output includes escape sequences for text and background attributes
    (@cmd ($cmd:expr) -e, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.escape_sequences()
        }) $($tail)*)
    }};
    // `[-p]` - the output goes to stdout
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.stdout()
        }) $($tail)*)
    }};
    // `[-P]` - capture only any output that the pane has received that is the beginning of an
    // as-yet incomplete escape sequence
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.pane()
        }) $($tail)*)
    }};
    // `[-q]` - quiet
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.quiet()
        }) $($tail)*)
    }};
    // `[-C]` - escape non-printable characters as octal \xxx
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.escape_non_printable()
        }) $($tail)*)
    }};
    // `[-J]` - preserve trailing spaces and joins any wrapped lines
    (@cmd ($cmd:expr) -J, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.join()
        }) $($tail)*)
    }};
    // `[-N]` - preserves trailing spaces at each line's end
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.trailing_spaces()
        }) $($tail)*)
    }};
    // `[-b buffer-name]` - buffer-name
    (@cmd ($cmd:expr) -b $buffer_name:expr, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.buffer_name($buffer_name)
        }) $($tail)*)
    }};
    // XXX: type?
    // `[-E end-line]` - specify the ending line number
    (@cmd ($cmd:expr) -E $end_line:expr, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.end_line($end_line)
        }) $($tail)*)
    }};
    // XXX: type?
    // `[-S start-line]` - specify the starting line number
    (@cmd ($cmd:expr) -S $start_line:expr, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.start_line($start_line)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - specify target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::CapturePane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({ $crate::CapturePane::new() }) $($tail)*,)
    }};
}

#[test]
fn capture_pane_macro() {
    use crate::TargetPane;
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

    let capture_pane = capture_pane!();
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane!((capture_pane), -a);
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane!((capture_pane), -e);
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane!((capture_pane), -p);
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane!((capture_pane), -P);
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane!((capture_pane), -q);
    #[cfg(feature = "tmux_2_4")]
    let capture_pane = capture_pane!((capture_pane), -C);
    #[cfg(feature = "tmux_2_4")]
    let capture_pane = capture_pane!((capture_pane), -J);
    #[cfg(feature = "tmux_3_1")]
    let capture_pane = capture_pane!((capture_pane), -N);
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane!((capture_pane), -b "1");
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane!((capture_pane), -E "2");
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane!((capture_pane), -S "3");
    #[cfg(feature = "tmux_1_2")]
    let capture_pane = capture_pane!((capture_pane), -t & target_pane);

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
