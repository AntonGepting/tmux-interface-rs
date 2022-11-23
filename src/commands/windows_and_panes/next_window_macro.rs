/// Move to the next window in the session
///
/// # Manual
///
/// tmux ^0.9:
/// ```text
/// next-window [-a] [-t target-session]
/// (alias: next)
/// ```
///
/// tmux ^0.8:
/// ```text
/// next-window [-t target-session]
/// (alias: next)
/// ```
#[macro_export]
macro_rules! next_window {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::next_window!(@cmd ({
            $cmd.attach()
        }) $($tail)*)
    }};
    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::next_window!(@cmd ({
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
        $crate::NextWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::next_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::next_window!(@cmd ({ $crate::NextWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn next_window_macro() {
    use std::borrow::Cow;

    // Move to the next window in the session
    //
    // # Manual
    //
    // tmux ^0.9:
    // ```text
    // next-window [-a] [-t target-session]
    // (alias: next)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // next-window [-t target-session]
    // (alias: next)
    // ```
    let next_window = next_window!();
    let next_window = next_window!((next_window), -a);
    #[cfg(feature = "tmux_0_8")]
    let next_window = next_window!((next_window), -t "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "next-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "next";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_9")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let next_window = next_window.build().to_vec();

    assert_eq!(next_window, s);
}
