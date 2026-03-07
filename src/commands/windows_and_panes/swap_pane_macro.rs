// auto-generated file
//

/// Swap two panes
///
/// # Manual
///
/// tmux >=3.1:
/// ```text
/// swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux >=1.5:
/// ```text
/// swap-pane [-dDU] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux >=0.8:
/// ```text
/// swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
/// (alias: swapp)
/// ```
#[macro_export]
macro_rules! swap_pane {
    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};

    // `[-D]`
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.previous_pane()
        }) $($tail)*)
    }};

    // `[-U]`
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.next_pane()
        }) $($tail)*)
    }};

    // `[-Z]`
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.keep_zoomed()
        }) $($tail)*)
    }};

    // `[-p src-index]`
    (@cmd ($cmd:expr) -p $src_index:expr, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.src_index($src_index)
        }) $($tail)*)
    }};

    // `[-s src-pane]`
    (@cmd ($cmd:expr) -s $src_pane:expr, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.src_pane($src_pane)
        }) $($tail)*)
    }};

    // `[-q dst-index]`
    (@cmd ($cmd:expr) -q $dst_index:expr, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            $cmd.dst_index($dst_index)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    // `[-t dst-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::swap_pane!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_1_5")]
            {
                $cmd.dst_pane($target)
            }
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
    use std::borrow::Cow;

    // Swap two panes
    //
    // # Manual
    //
    // tmux >=3.1:
    // ```text
    // swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
    // (alias: swapp)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // swap-pane [-dDU] [-s src-pane] [-t dst-pane]
    // (alias: swapp)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
    // (alias: swapp)
    // ```

    let swap_pane = swap_pane!();
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane!((swap_pane), -d);
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane!((swap_pane), -D);
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane!((swap_pane), -U);
    #[cfg(feature = "tmux_3_1")]
    let swap_pane = swap_pane!((swap_pane), -Z);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let swap_pane = swap_pane!((swap_pane), -p "1");
    #[cfg(feature = "tmux_1_5")]
    let swap_pane = swap_pane!((swap_pane), -s "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let swap_pane = swap_pane!((swap_pane), -q "3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let swap_pane = swap_pane!((swap_pane), -t "4");
    #[cfg(feature = "tmux_1_5")]
    let swap_pane = swap_pane!((swap_pane), -t "5");

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
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-p", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-s", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-q", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "5"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let swap_pane = swap_pane.build().to_vec();

    assert_eq!(swap_pane, s);
}
