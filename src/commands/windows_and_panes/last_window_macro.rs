/// Select the last (previously selected) window
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// last-window [-t target-session]
/// (alias: last)
/// ```
#[macro_export]
macro_rules! last_window {
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::last_window!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::LastWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::last_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::last_window!(@cmd ({ $crate::LastWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn last_window_macro() {
    use std::borrow::Cow;

    // Select the last (previously selected) window
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // last-window [-t target-session]
    // (alias: last)
    // ```
    let last_window = last_window!();
    #[cfg(feature = "tmux_0_8")]
    let last_window = last_window!((last_window), -t "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "last-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "last";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let last_window = last_window.build().to_vec();

    assert_eq!(last_window, s);
}
