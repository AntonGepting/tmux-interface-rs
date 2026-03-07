// auto-generated file
//

/// Make pane `target-pane` the active pane in window `target-window`
///
/// # Manual
///
/// tmux >=3.1:
/// ```text
/// select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=2.6:
/// ```text
/// select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=2.1:
/// ```text
/// select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=2.0:
/// ```text
/// select-pane [-DdeLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=1.5:
/// ```text
/// select-pane [-DLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=0.8:
/// ```text
/// select-pane [-p pane-index] [-t target-window]
/// (alias: selectp)
/// ```
#[macro_export]
macro_rules! select_pane {
    // `[-D]`
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.down()
        }) $($tail)*)
    }};

    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.disable()
        }) $($tail)*)
    }};

    // `[-e]`
    (@cmd ($cmd:expr) -e, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.enable()
        }) $($tail)*)
    }};

    // `[-g]`
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.show_style()
        }) $($tail)*)
    }};

    // `[-L]`
    (@cmd ($cmd:expr) -L, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.left()
        }) $($tail)*)
    }};

    // `[-l]`
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.last()
        }) $($tail)*)
    }};

    // `[-M]`
    (@cmd ($cmd:expr) -M, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.set_marked()
        }) $($tail)*)
    }};

    // `[-m]`
    (@cmd ($cmd:expr) -m, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.clear_marked()
        }) $($tail)*)
    }};

    // `[-R]`
    (@cmd ($cmd:expr) -R, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.right()
        }) $($tail)*)
    }};

    // `[-U]`
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.up()
        }) $($tail)*)
    }};

    // `[-Z]`
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.keep_zoomed()
        }) $($tail)*)
    }};

    // `[-p pane-index]`
    (@cmd ($cmd:expr) -p $pane_index:expr, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.pane_index($pane_index)
        }) $($tail)*)
    }};

    // `[-P style]`
    (@cmd ($cmd:expr) -P $style:expr, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.style($style)
        }) $($tail)*)
    }};

    // `[-T title]`
    (@cmd ($cmd:expr) -T $title:expr, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            $cmd.title($title)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::select_pane!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_1_5")]
            {
                $cmd.target_pane($target)
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
    use std::borrow::Cow;

    // Make pane `target-pane` the active pane in window `target-window`
    //
    // # Manual
    //
    // tmux >=3.1:
    // ```text
    // select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux >=2.6:
    // ```text
    // select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux >=2.1:
    // ```text
    // select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux >=2.0:
    // ```text
    // select-pane [-DdeLlRU] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // select-pane [-DLlRU] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // select-pane [-p pane-index] [-t target-window]
    // (alias: selectp)
    // ```

    let select_pane = select_pane!();
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane!((select_pane), -D);
    #[cfg(feature = "tmux_2_0")]
    let select_pane = select_pane!((select_pane), -d);
    #[cfg(feature = "tmux_2_0")]
    let select_pane = select_pane!((select_pane), -e);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_1")))]
    let select_pane = select_pane!((select_pane), -g);
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane!((select_pane), -L);
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane!((select_pane), -l);
    #[cfg(feature = "tmux_2_1")]
    let select_pane = select_pane!((select_pane), -M);
    #[cfg(feature = "tmux_2_1")]
    let select_pane = select_pane!((select_pane), -m);
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane!((select_pane), -R);
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane!((select_pane), -U);
    #[cfg(feature = "tmux_3_1")]
    let select_pane = select_pane!((select_pane), -Z);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let select_pane = select_pane!((select_pane), -p "1");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0a")))]
    let select_pane = select_pane!((select_pane), -P "2");
    #[cfg(feature = "tmux_2_6")]
    let select_pane = select_pane!((select_pane), -T "3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let select_pane = select_pane!((select_pane), -t "4");
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane!((select_pane), -t "5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-D");
    #[cfg(feature = "tmux_2_0")]
    s.push("-d");
    #[cfg(feature = "tmux_2_0")]
    s.push("-e");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_1")))]
    s.push("-g");
    #[cfg(feature = "tmux_1_5")]
    s.push("-L");
    #[cfg(feature = "tmux_1_5")]
    s.push("-l");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_2_1")]
    s.push("-m");
    #[cfg(feature = "tmux_1_5")]
    s.push("-R");
    #[cfg(feature = "tmux_1_5")]
    s.push("-U");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-p", "1"]);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0a")))]
    s.extend_from_slice(&["-P", "2"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-T", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "5"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let select_pane = select_pane.build().to_vec();

    assert_eq!(select_pane, s);
}
