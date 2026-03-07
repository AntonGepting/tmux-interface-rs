// auto-generated file
//

/// Enter copy mode
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// copy-mode [-deHMqSu] [-s src-pane] [-t target-pane]
/// ```
///
/// tmux >=3.5:
/// ```text
/// copy-mode [-deHMqu] [-s src-pane] [-t target-pane]
/// ```
///
/// tmux >=3.1a:
/// ```text
/// copy-mode [-eHMqu] [-s src-pane] [-t target-pane]
/// ```
///
/// tmux >=2.1:
/// ```text
/// copy-mode [-Meu] [-t target-pane]
/// ```
///
/// tmux >=1.5:
/// ```text
/// copy-mode [-u] [-t target-pane]
/// ```
///
/// tmux >=0.8:
/// ```text
/// copy-mode [-u] [-t target-window]
/// ```
#[macro_export]
macro_rules! copy_mode {
    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.scroll_down()
        }) $($tail)*)
    }};

    // `[-e]`
    (@cmd ($cmd:expr) -e, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.bottom_exit()
        }) $($tail)*)
    }};

    // `[-H]`
    (@cmd ($cmd:expr) -H, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.hide_position()
        }) $($tail)*)
    }};

    // `[-M]`
    (@cmd ($cmd:expr) -M, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.mouse_drag()
        }) $($tail)*)
    }};

    // `[-q]`
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.cancel()
        }) $($tail)*)
    }};

    // `[-S]`
    (@cmd ($cmd:expr) -S, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.from_src_pane()
        }) $($tail)*)
    }};

    // `[-u]`
    (@cmd ($cmd:expr) -u, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.page_up()
        }) $($tail)*)
    }};

    // `[-s src-pane]`
    (@cmd ($cmd:expr) -s $src_pane:expr, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.src_pane($src_pane)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_1_0")]
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
        $crate::CopyMode::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({ $crate::CopyMode::new() }) $($tail)*,)
    }};
}

#[test]
fn copy_mode_macro() {
    use std::borrow::Cow;

    // Enter copy mode
    //
    // # Manual
    //
    // tmux >=3.6:
    // ```text
    // copy-mode [-deHMqSu] [-s src-pane] [-t target-pane]
    // ```
    //
    // tmux >=3.5:
    // ```text
    // copy-mode [-deHMqu] [-s src-pane] [-t target-pane]
    // ```
    //
    // tmux >=3.1a:
    // ```text
    // copy-mode [-eHMqu] [-s src-pane] [-t target-pane]
    // ```
    //
    // tmux >=2.1:
    // ```text
    // copy-mode [-Meu] [-t target-pane]
    // ```
    //
    // tmux >=1.5:
    // ```text
    // copy-mode [-u] [-t target-pane]
    // ```
    //
    // tmux >=0.8:
    // ```text
    // copy-mode [-u] [-t target-window]
    // ```

    let copy_mode = copy_mode!();
    #[cfg(feature = "tmux_3_5")]
    let copy_mode = copy_mode!((copy_mode), -d);
    #[cfg(feature = "tmux_2_1")]
    let copy_mode = copy_mode!((copy_mode), -e);
    #[cfg(feature = "tmux_3_1a")]
    let copy_mode = copy_mode!((copy_mode), -H);
    #[cfg(feature = "tmux_2_1")]
    let copy_mode = copy_mode!((copy_mode), -M);
    #[cfg(feature = "tmux_3_1a")]
    let copy_mode = copy_mode!((copy_mode), -q);
    #[cfg(feature = "tmux_3_6")]
    let copy_mode = copy_mode!((copy_mode), -S);
    #[cfg(feature = "tmux_0_8")]
    let copy_mode = copy_mode!((copy_mode), -u);
    #[cfg(feature = "tmux_3_2")]
    let copy_mode = copy_mode!((copy_mode), -s "1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_mode = copy_mode!((copy_mode), -t "2");
    #[cfg(feature = "tmux_1_5")]
    let copy_mode = copy_mode!((copy_mode), -t "3");

    let cmd = "copy-mode";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_5")]
    s.push("-d");
    #[cfg(feature = "tmux_2_1")]
    s.push("-e");
    #[cfg(feature = "tmux_3_1a")]
    s.push("-H");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_3_1a")]
    s.push("-q");
    #[cfg(feature = "tmux_3_6")]
    s.push("-S");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let copy_mode = copy_mode.build().to_vec();

    assert_eq!(copy_mode, s);
}
