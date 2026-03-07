// auto-generated file
//

/// Unlink `target-window`
///
/// # Manual
///
/// tmux >=1.5:
/// ```text
/// unlink-window [-k] [-t target-window]
/// (alias: unlinkw)
/// ```
///
/// tmux >=0.8:
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
    use std::borrow::Cow;

    // Unlink `target-window`
    //
    // # Manual
    //
    // tmux >=1.5:
    // ```text
    // unlink-window [-k] [-t target-window]
    // (alias: unlinkw)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // unlink-window [-t target-window]
    // (alias: unlinkw)
    // ```

    let unlink_window = unlink_window!();
    #[cfg(feature = "tmux_1_5")]
    let unlink_window = unlink_window!((unlink_window), -k);
    #[cfg(feature = "tmux_0_8")]
    let unlink_window = unlink_window!((unlink_window), -t "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "unlink-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "unlinkw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-k");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let unlink_window = unlink_window.build().to_vec();

    assert_eq!(unlink_window, s);
}
