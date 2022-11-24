/// Select the window at target-window.
///
/// # Manual
///
/// tmux ^1.8:
/// ```text
/// select-window [-lnpT] [-t target-window]
/// (alias: selectw)
/// ```
///
/// tmux ^1.5:
/// ```text
/// select-window [-lnp] [-t target-window]
/// (alias: selectw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// select-window [-t target-window]
/// (alias: selectw)
/// ```
#[macro_export]
macro_rules! select_window {
    // `[-l]` - equivalent to last-window
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::select_window!(@cmd ({
            $cmd.last()
        }) $($tail)*)
    }};
    // `[-n]` - equivalent to next-window
    (@cmd ($cmd:expr) -n, $($tail:tt)*) => {{
        $crate::select_window!(@cmd ({
            $cmd.next()
        }) $($tail)*)
    }};
    // `[-p]` - equivalent to previous-window
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::select_window!(@cmd ({
            $cmd.previous()
        }) $($tail)*)
    }};
    // `[-T]` - if the selected window is already the current window, behave like last-window
    (@cmd ($cmd:expr) -T, $($tail:tt)*) => {{
        $crate::select_window!(@cmd ({
            $cmd.switch()
        }) $($tail)*)
    }};
    // `[-t target-window]` - target-window
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::select_window!(@cmd ({
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
        $crate::SelectWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::select_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::select_window!(@cmd ({ $crate::SelectWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn select_window_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Select the window at target-window.
    //
    // # Manual
    //
    // tmux ^1.8:
    // ```text
    // select-window [-lnpT] [-t target-window]
    // (alias: selectw)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // select-window [-lnp] [-t target-window]
    // (alias: selectw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // select-window [-t target-window]
    // (alias: selectw)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let select_window = select_window!();
    #[cfg(feature = "tmux_1_5")]
    let select_window = select_window!((select_window), -l);
    #[cfg(feature = "tmux_1_5")]
    let select_window = select_window!((select_window), -n);
    #[cfg(feature = "tmux_1_5")]
    let select_window = select_window!((select_window), -p);
    #[cfg(feature = "tmux_1_8")]
    let select_window = select_window!((select_window), -T);
    #[cfg(feature = "tmux_0_8")]
    let select_window = select_window!((select_window), -t & target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-l");
    #[cfg(feature = "tmux_1_5")]
    s.push("-n");
    #[cfg(feature = "tmux_1_5")]
    s.push("-p");
    #[cfg(feature = "tmux_1_8")]
    s.push("-T");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let select_window = select_window.build().to_vec();

    assert_eq!(select_window, s);
}
