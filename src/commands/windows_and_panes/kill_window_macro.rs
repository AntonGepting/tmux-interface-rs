/// Kill the current window or the window at target-window, removing it from any sessions
/// to which it is linked
///
/// # Manual
///
/// tmux ^1.7:
/// ```text
/// kill-window [-a] [-t target-window]
/// (alias: killw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// kill-window [-a] [-t target-window]
/// (alias: killw)
/// ```
#[macro_export]
macro_rules! kill_window {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::kill_window!(@cmd ({
            $cmd.parent_sighup()
        }) $($tail)*)
    }};
    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::kill_window!(@cmd ({
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
        $crate::KillWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::kill_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::kill_window!(@cmd ({ $crate::KillWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn kill_window_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Kill the current window or the window at target-window, removing it from any sessions
    // to which it is linked
    //
    // # Manual
    // tmux ^1.7:
    // ```text
    // kill-window [-a] [-t target-window]
    // (alias: killw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // kill-window [-t target-window]
    // (alias: killw)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let kill_window = kill_window!();
    #[cfg(feature = "tmux_1_7")]
    let kill_window = kill_window!((kill_window), -a);
    #[cfg(feature = "tmux_0_8")]
    let kill_window = kill_window!((kill_window), -t & target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "kill-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "killw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let kill_window = kill_window.build().to_vec();

    assert_eq!(kill_window, s);
}
