/// Swap two panes
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// swap-pane [-dDU] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
/// (alias: swapp)
/// ```
#[macro_export]
macro_rules! swap_pane {
    // `[-d]` - instruct tmux not to change the active pane
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-D]` - swap with the next pane
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.previous_pane()
        }) $($tail)*)
    }};
    // `[-U]` - swap with the previous pane
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.next_pane()
        }) $($tail)*)
    }};
    // `[-Z]` - keep the window zoomed if it was zoomed
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.keep_zoomed()
        }) $($tail)*)
    }};
    // `[-s src-pane]` - src-pane
    (@cmd ($cmd:expr) -s $src_pane:expr, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.src_pane($src_pane)
        }) $($tail)*)
    }};
    // `[-t dst-pane]` - dst-pane
    (@cmd ($cmd:expr) -t $dst_pane:expr, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.dst_pane($dst_pane)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SwapPane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({ $crate::SwapPane::new() }) $($tail)*,)
    }};
}

#[test]
fn swap_pane_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Swap two panes
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
    // (alias: swapp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // swap-pane [-dDU] [-s src-pane] [-t dst-pane]
    // (alias: swapp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
    // (alias: swapp)
    // ```
    let src_pane = TargetPane::Raw("1").to_string();
    let dst_pane = TargetPane::Raw("2").to_string();

    let swap_pane = swap_pane!();
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane!((swap_pane), -d);
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane!((swap_pane), -D);
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane!((swap_pane), -U);
    #[cfg(feature = "tmux_3_1")]
    let swap_pane = swap_pane!((swap_pane), -Z);
    #[cfg(feature = "tmux_1_0")]
    let swap_pane = swap_pane!((swap_pane), -s & src_pane);
    #[cfg(feature = "tmux_1_0")]
    let swap_pane = swap_pane!((swap_pane), -t & dst_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "swap-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "swapp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_0_8")]
    s.push("-D");
    #[cfg(feature = "tmux_0_8")]
    s.push("-U");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let swap_pane = swap_pane.build().to_vec();

    assert_eq!(swap_pane, s);
}
