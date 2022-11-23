/// Move to the previous window in the session
///
/// # Manual
///
/// tmux ^0.9:
/// ```text
/// previous-window [-a] [-t target-session]
/// (alias: prev)
/// ```
///
/// tmux ^0.8:
/// ```text
/// previous-window [-t target-session]
/// (alias: prev)
/// ```
#[macro_export]
macro_rules! previous_window {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::previous_window!(@cmd ({
            $cmd.parent_sighup()
        }) $($tail)*)
    }};
    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::previous_window!(@cmd ({
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
        $crate::PreviousWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::previous_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::previous_window!(@cmd ({ $crate::PreviousWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn previous_window_macro() {
    use std::borrow::Cow;

    // tmux ^0.9:
    // ```text
    // previous-window [-a] [-t target-session]
    // (alias: prev)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // previous-window [-t target-session]
    // (alias: prev)
    // ```
    let previous_window = previous_window!();
    #[cfg(feature = "tmux_0_9")]
    let previous_window = previous_window!((previous_window), -a);
    #[cfg(feature = "tmux_0_8")]
    let previous_window = previous_window!((previous_window), -t "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "previous-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "prev";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_9")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let previous_window = previous_window.build().to_vec();

    assert_eq!(previous_window, s);
}
