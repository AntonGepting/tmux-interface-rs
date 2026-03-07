// auto-generated file
//

/// Capture the contents of a pane
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// capture-pane [-aepPqCJMNT] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=3.4:
/// ```text
/// capture-pane [-aAepPqCJNT] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=3.1:
/// ```text
/// capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=2.4:
/// ```text
/// capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=1.8:
/// ```text
/// capture-pane [-aepPq] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=1.5:
/// ```text
/// capture-pane [-b buffer-index] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
#[macro_export]
macro_rules! capture_pane {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.alternate_screen()
        }) $($tail)*)
    }};

    // `[-A]`
    (@cmd ($cmd:expr) -A, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.a()
        }) $($tail)*)
    }};

    // `[-e]`
    (@cmd ($cmd:expr) -e, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.escape_sequences()
        }) $($tail)*)
    }};

    // `[-p]`
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.stdout()
        }) $($tail)*)
    }};

    // `[-P]`
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.pane()
        }) $($tail)*)
    }};

    // `[-q]`
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.quiet()
        }) $($tail)*)
    }};

    // `[-C]`
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.escape_non_printable()
        }) $($tail)*)
    }};

    // `[-J]`
    (@cmd ($cmd:expr) -J, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.join()
        }) $($tail)*)
    }};

    // `[-M]`
    (@cmd ($cmd:expr) -M, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.screen_for_mode()
        }) $($tail)*)
    }};

    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.trailing_spaces()
        }) $($tail)*)
    }};

    // `[-b buffer-index]`
    // `[-b buffer-name]`
    (@cmd ($cmd:expr) -b $buffer:expr, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
            {
                $cmd.buffer_index($buffer)
            }
            #[cfg(feature = "tmux_2_0")]
            {
                $cmd.buffer_name($buffer)
            }
        }) $($tail)*)
    }};

    // `[-E end-line]`
    (@cmd ($cmd:expr) -E $end_line:expr, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.end_line($end_line)
        }) $($tail)*)
    }};

    // `[-S start-line]`
    (@cmd ($cmd:expr) -S $start_line:expr, $($tail:tt)*) => {{
        $crate::capture_pane!(@cmd ({
            $cmd.start_line($start_line)
        }) $($tail)*)
    }};

    // `[-t target-pane]`
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
    use std::borrow::Cow;

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

    let capture_pane = capture_pane!();
    #[cfg(feature = "tmux_1_8")]
    let capture_pane = capture_pane!((capture_pane), -a);
    #[cfg(feature = "tmux_3_4")]
    let capture_pane = capture_pane!((capture_pane), -A);
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
    #[cfg(feature = "tmux_3_6")]
    let capture_pane = capture_pane!((capture_pane), -M);
    #[cfg(feature = "tmux_3_1")]
    let capture_pane = capture_pane!((capture_pane), -N);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    let capture_pane = capture_pane!((capture_pane), -b "1");
    #[cfg(feature = "tmux_2_0")]
    let capture_pane = capture_pane!((capture_pane), -b "2");
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane!((capture_pane), -E "3");
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane!((capture_pane), -S "4");
    #[cfg(feature = "tmux_1_5")]
    let capture_pane = capture_pane!((capture_pane), -t "5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "capture-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "capturep";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    s.push("-a");
    #[cfg(feature = "tmux_3_4")]
    s.push("-A");
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
    #[cfg(feature = "tmux_3_6")]
    s.push("-M");
    #[cfg(feature = "tmux_3_1")]
    s.push("-N");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-E", "3"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-S", "4"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "5"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let capture_pane = capture_pane.build().to_vec();

    assert_eq!(capture_pane, s);
}
