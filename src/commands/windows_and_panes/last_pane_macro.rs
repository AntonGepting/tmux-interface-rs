/// Select the last (previously selected) pane
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// last-pane [-deZ] [-t target-window]
/// (alias: lastp)
/// ```
///
/// tmux ^2.0:
/// ```text
/// last-pane [-de] [-t target-window]
/// (alias: lastp)
/// ```
///
/// tmux ^1.4:
/// ```text
/// last-pane [-t target-window]
/// (alias: lastp)
/// ```
// FIXME: versions and function parameters
#[macro_export]
macro_rules! last_pane {
    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::last_pane!(@cmd ({
            $cmd.disable()
        }) $($tail)*)
    }};
    // `[-e]`
    (@cmd ($cmd:expr) -e, $($tail:tt)*) => {{
        $crate::last_pane!(@cmd ({
            $cmd.enable()
        }) $($tail)*)
    }};
    // `[-Z]`
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::last_pane!(@cmd ({
            $cmd.keep_zoomed()
        }) $($tail)*)
    }};
    // `[-t target-window]`
    (@cmd ($cmd:expr) -c $target_window:expr, $($tail:tt)*) => {{
        $crate::last_pane!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::LastPane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::last_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::last_pane!(@cmd ({ $crate::LastPane::new() }) $($tail)*,)
    }};
}

#[test]
fn last_pane_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Select the last (previously selected) pane
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // last-pane [-deZ] [-t target-window]
    // (alias: lastp)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // last-pane [-de] [-t target-window]
    // (alias: lastp)
    // ```
    //
    // tmux ^1.4:
    // ```text
    // last-pane [-t target-window]
    // (alias: lastp)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let last_pane = last_pane!();
    #[cfg(feature = "tmux_2_0")]
    let last_pane = last_pane!((last_pane), -d);
    #[cfg(feature = "tmux_2_0")]
    let last_pane = last_pane!((last_pane), -e);
    #[cfg(feature = "tmux_3_1")]
    let last_pane = last_pane!((last_pane), -Z);
    #[cfg(feature = "tmux_1_4")]
    let last_pane = last_pane!((last_pane), -t & target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "last-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lastp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_0")]
    s.push("-d");
    #[cfg(feature = "tmux_2_0")]
    s.push("-e");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_1_4")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let last_pane = last_pane.build().to_vec();

    assert_eq!(last_pane, s);
}
