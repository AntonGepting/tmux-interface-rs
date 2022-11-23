/// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// break-pane [-abdP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.4:
/// ```text
/// break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.2:
/// ```text
/// break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.1:
/// ```text
/// break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
/// (alias: breakp)
/// ```
///
/// tmux ^1.7:
/// ```text
/// break-pane [-dP] [-F format] [-t target-pane]
/// (alias: breakp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// break-pane [-d] [-t target-window]
/// (alias: breakp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// break-pane [-d] [-p pane-index] [-t target-window]
/// (alias: breakp)
/// ```
#[macro_export]
macro_rules! break_pane {
    // `[-a]` - the window is moved to the next index after
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.after()
        }) $($tail)*)
    }};
    // `[-b]` - the window is moved to the next index before
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.before()
        }) $($tail)*)
    }};
    // `[-d]` - the new window does not become the current window
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-P]` - option prints information about the new window after it has been created
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.print()
        }) $($tail)*)
    }};
    // `[-F format]` - specify format
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    // `[-n window-name]` - window-name
    (@cmd ($cmd:expr) -n $window_name:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.window_name($window_name)
        }) $($tail)*)
    }};
    // `[-s src-pane]` - src-pane
    (@cmd ($cmd:expr) -s $src_pane:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.src_pane($src_pane)
        }) $($tail)*)
    }};
    // `[-t dst-window]` - dst-window
    (@cmd ($cmd:expr) -t $dst_window:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.dst_window($dst_window)
        }) $($tail)*)
    }};
    // `[-t dst-pane]` - dst-pane
    (@cmd ($cmd:expr) -t $dst_pane:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.dst_pane($dst_pane)
        }) $($tail)*)
    }};
    // `[-t target-window]` - target-window
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::break_pane!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // FIXME:
    // `[-p pane-index]` - pane-index
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
    use crate::TargetPane;
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Break `src-pane` off from its containing window to make it the only pane in `dst-window`
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // break-pane [-abdP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^2.2:
    // ```text
    // break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
    // (alias: breakp)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // break-pane [-dP] [-F format] [-t target-pane]
    // (alias: breakp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // break-pane [-d] [-t target-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // break-pane [-d] [-p pane-index] [-t target-window]
    // (alias: breakp)
    // ```
    let src_pane = TargetPane::Raw("3").to_string();
    let dst_pane = TargetPane::Raw("4").to_string();
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let target_window = TargetWindow::Raw("4").to_string();

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
    #[cfg(feature = "tmux_2_1")]
    let break_pane = break_pane!((break_pane), -s & src_pane);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    let break_pane = break_pane!((break_pane), -t & dst_pane);
    #[cfg(feature = "tmux_2_2")]
    let break_pane = break_pane!((break_pane), -t & dst_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    let break_pane = break_pane!((break_pane), -t & dst_pane);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let break_pane = break_pane!((break_pane), -t & target_window);

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
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-s", "3"]);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_2_2")]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    s.extend_from_slice(&["-t", "4"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let break_pane = break_pane.build().to_vec();

    assert_eq!(break_pane, s);
}
