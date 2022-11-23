/// This is similar to link-window, except the source and destination windows are swapped
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// swap-window [-d] [-s src-window] [-t dst-window]
/// (alias: swapw)
/// ```
#[macro_export]
macro_rules! swap_window {
    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-s src-window]`
    (@cmd ($cmd:expr) -s $src_window:expr, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.src_window($src_window)
        }) $($tail)*)
    }};
    // `[-t dst-window]`
    (@cmd ($cmd:expr) -t $dst_window:expr, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.dst_window($dst_window)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SwitchClient::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({ $crate::SwitchClient::new() }) $($tail)*,)
    }};
}

#[test]
fn swap_window_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // This is similar to link-window, except the source and destination windows are swapped
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // swap-window [-d] [-s src-window] [-t dst-window]
    // (alias: swapw)
    // ```
    let src_window = TargetWindow::Raw("1").to_string();
    let dst_window = TargetWindow::Raw("2").to_string();

    let swap_window = swap_window!();
    #[cfg(feature = "tmux_0_8")]
    let swap_window = swap_window!((swap_window), -d);
    #[cfg(feature = "tmux_0_8")]
    let swap_window = swap_window!((swap_window), -s & src_window);
    #[cfg(feature = "tmux_0_8")]
    let swap_window = swap_window!((swap_window), -t & dst_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "swap-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "swapw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);

    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let swap_window = swap_window.build().to_vec();

    assert_eq!(swap_window, s);
}
