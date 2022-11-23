/// Make pane `target-pane` the active pane in window `target-window`
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^2.6:
/// ```text
/// select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^2.1:
/// ```text
/// select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^2.0:
/// ```text
/// select-pane [-DdeLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^1.5:
/// ```text
/// select-pane [-DLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^1.3:
/// ```text
/// select-pane [-DLRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// select-pane [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// select-pane [-p pane-index] [-t target-window]
/// (alias: selectp)
/// ```
#[macro_export]
macro_rules! select_pane {
    // `[-D]` - pane below
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.down()
        }) $($tail)*)
    }};
    // `[-d]` - disable input
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.disable()
        }) $($tail)*)
    }};
    // `[-e]` - enable input
    (@cmd ($cmd:expr) -e, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.enable()
        }) $($tail)*)
    }};
    // `[-g]` - show the current pane style
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.show_style()
        }) $($tail)*)
    }};
    // `[-L]` - pane left
    (@cmd ($cmd:expr) -L, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.left()
        }) $($tail)*)
    }};
    // `[-l]` - equivalent to last-pane command
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.last()
        }) $($tail)*)
    }};
    // `[-M]` - clear marked pane
    (@cmd ($cmd:expr) -M, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.set_marked()
        }) $($tail)*)
    }};
    // `[-m]` - set marked pane
    (@cmd ($cmd:expr) -m, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.clear_marked()
        }) $($tail)*)
    }};
    // `[-R]` - pane right
    (@cmd ($cmd:expr) -R, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.right()
        }) $($tail)*)
    }};
    // `[-U]` - pane above
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.up()
        }) $($tail)*)
    }};
    // `[-Z]` - keep the window zoomed if it was zoomed
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.keep_zoomed()
        }) $($tail)*)
    }};
    // `[-P style]` - set the style for a single pane
    (@cmd ($cmd:expr) -P $style:expr, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.style($style)
        }) $($tail)*)
    }};
    // `[-T title]` - title
    (@cmd ($cmd:expr) -T $title:expr, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.title($title)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SelectPane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({ $crate::SelectPane::new() }) $($tail)*,)
    }};
}

#[test]
fn select_pane_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Make pane `target-pane` the active pane in window `target-window`
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // select-pane [-DdeLlRU] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // select-pane [-DLlRU] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^1.3:
    // ```text
    // select-pane [-DLRU] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // select-pane [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // select-pane [-p pane-index] [-t target-window]
    // (alias: selectp)
    // ```
    let target_pane = TargetPane::Raw("3").to_string();

    let select_pane = select_pane!();
    #[cfg(feature = "tmux_1_3")]
    let select_pane = select_pane!((select_pane), -D);
    #[cfg(feature = "tmux_2_0")]
    let select_pane = select_pane!((select_pane), -d);
    #[cfg(feature = "tmux_2_0")]
    let select_pane = select_pane!((select_pane), -e);
    #[cfg(feature = "tmux_2_1")]
    let select_pane = select_pane!((select_pane), -g);
    #[cfg(feature = "tmux_1_3")]
    let select_pane = select_pane!((select_pane), -L);
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane!((select_pane), -l);
    #[cfg(feature = "tmux_2_1")]
    let select_pane = select_pane!((select_pane), -M);
    #[cfg(feature = "tmux_2_1")]
    let select_pane = select_pane!((select_pane), -m);
    #[cfg(feature = "tmux_1_3")]
    let select_pane = select_pane!((select_pane), -R);
    #[cfg(feature = "tmux_1_3")]
    let select_pane = select_pane!((select_pane), -U);
    #[cfg(feature = "tmux_3_1")]
    let select_pane = select_pane!((select_pane), -Z);
    #[cfg(feature = "tmux_2_1")]
    let select_pane = select_pane!((select_pane), -P "1");
    #[cfg(feature = "tmux_2_6")]
    let select_pane = select_pane!((select_pane), -T "2");
    #[cfg(feature = "tmux_1_0")]
    let select_pane = select_pane!((select_pane), -t & target_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_3")]
    s.push("-D");
    #[cfg(feature = "tmux_2_0")]
    s.push("-d");
    #[cfg(feature = "tmux_2_0")]
    s.push("-e");
    #[cfg(feature = "tmux_2_1")]
    s.push("-g");
    #[cfg(feature = "tmux_1_3")]
    s.push("-L");
    #[cfg(feature = "tmux_1_5")]
    s.push("-l");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_2_1")]
    s.push("-m");
    #[cfg(feature = "tmux_1_3")]
    s.push("-R");
    #[cfg(feature = "tmux_1_3")]
    s.push("-U");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-P", "1"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-T", "2"]);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let select_pane = select_pane.build().to_vec();

    assert_eq!(select_pane, s);
}
