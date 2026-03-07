// auto-generated file
//

/// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// break-pane [-abdP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux >=2.4:
/// ```text
/// break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux >=2.2:
/// ```text
/// break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux >=2.1:
/// ```text
/// break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
/// (alias: breakp)
/// ```
///
/// tmux >=1.7:
/// ```text
/// break-pane [-dP] [-F format] [-t target-pane]
/// (alias: breakp)
/// ```
///
/// tmux >=1.5:
/// ```text
/// break-pane [-d] [-t target-pane]
/// (alias: breakp)
/// ```
///
/// tmux >=0.8:
/// ```text
/// break-pane [-d] [-p pane-index] [-t target-window]
/// (alias: breakp)
/// ```
#[macro_export]
macro_rules! break_pane {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.after()
        }) $($tail)*)
    }};

    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.before()
        }) $($tail)*)
    }};

    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};

    // `[-P]`
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.print()
        }) $($tail)*)
    }};

    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-n window-name]`
    (@cmd ($cmd:expr) -n $window_name:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.window_name($window_name)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    // `[-t target-pane]`
    // `[-t dst-pane]`
    // `[-t dst-window]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
            {
                $cmd.target_pane($target)
            }
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_1")))]
            {
                $cmd.dst_pane($target)
            }
            #[cfg(feature = "tmux_2_2")]
            {
                $cmd.dst_window($target)
            }
        }) $($tail)*)
    }};

    // `[-s src-pane]`
    (@cmd ($cmd:expr) -s $src_pane:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.src_pane($src_pane)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::BreakPane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({ $crate::BreakPane::new() }) $($tail)*,)
    }};
}

#[test]
fn break_pane_macro() {
    use std::borrow::Cow;

    // Break `src-pane` off from its containing window to make it the only pane in `dst-window`
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // break-pane [-abdP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux >=2.4:
    // ```text
    // break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux >=2.2:
    // ```text
    // break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux >=2.1:
    // ```text
    // break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
    // (alias: breakp)
    // ```
    //
    // tmux >=1.7:
    // ```text
    // break-pane [-dP] [-F format] [-t target-pane]
    // (alias: breakp)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // break-pane [-d] [-t target-pane]
    // (alias: breakp)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // break-pane [-d] [-p pane-index] [-t target-window]
    // (alias: breakp)
    // ```

    let break_pane = break_pane!();
    #[cfg(feature = "tmux_3_2")]
    let break_pane = break_pane!((break_pane), -a);
    #[cfg(feature = "tmux_3_2")]
    let break_pane = break_pane!((break_pane), -b);
    #[cfg(feature = "tmux_0_8")]
    let break_pane = break_pane!((break_pane), -d);
    #[cfg(feature = "tmux_1_7")]
    let break_pane = break_pane!((break_pane), -P);
    #[cfg(feature = "tmux_1_7")]
    let break_pane = break_pane!((break_pane), -F "1");
    #[cfg(feature = "tmux_2_4")]
    let break_pane = break_pane!((break_pane), -n "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let break_pane = break_pane!((break_pane), -t "3");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let break_pane = break_pane!((break_pane), -t "4");
    #[cfg(feature = "tmux_2_1")]
    let break_pane = break_pane!((break_pane), -s "5");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_1")))]
    let break_pane = break_pane!((break_pane), -t "6");
    #[cfg(feature = "tmux_2_2")]
    let break_pane = break_pane!((break_pane), -t "7");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "break-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "breakp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-a");
    #[cfg(feature = "tmux_3_2")]
    s.push("-b");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_7")]
    s.push("-P");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-n", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-s", "5"]);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_1")))]
    s.extend_from_slice(&["-t", "6"]);
    #[cfg(feature = "tmux_2_2")]
    s.extend_from_slice(&["-t", "7"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let break_pane = break_pane.build().to_vec();

    assert_eq!(break_pane, s);
}
