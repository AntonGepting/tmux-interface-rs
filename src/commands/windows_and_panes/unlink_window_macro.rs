/// Unlink `target-window`
///
/// # Manual
///
/// tmux ^1.0:
/// ```text
/// unlink-window [-k] [-t target-window]
/// (alias: unlinkw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// unlink-window [-t target-window]
/// (alias: unlinkw)
/// ```
#[macro_export]
macro_rules! unlink_window {
    // `[-k]`
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::unlink_window!(@cmd ({
            $cmd.detach_other()
        }) $($tail)*)
    }};
    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::unlink_window!(@cmd ({
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
        $crate::UnlinkWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::unlink_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::unlink_window!(@cmd ({ $crate::UnlinkWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn unlink_window_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Unlink `target-window`
    //
    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // unlink-window [-k] [-t target-window]
    // (alias: unlinkw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // unlink-window [-t target-window]
    // (alias: unlinkw)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let unlink_window = unlink_window!();
    #[cfg(feature = "tmux_0_8")]
    let unlink_window = unlink_window!((unlink_window), -k);
    #[cfg(feature = "tmux_0_8")]
    let unlink_window = unlink_window!((unlink_window), -t & target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "unlink-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "unlinkw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-k");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let unlink_window = unlink_window.build().to_vec();

    assert_eq!(unlink_window, s);
}
