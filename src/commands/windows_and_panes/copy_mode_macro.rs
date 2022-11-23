/// Enter copy mode
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// copy-mode [-eHMqu] [-s src-pane] [-t target-pane]
/// ```
///
/// tmux ^2.1:
/// ```text
/// copy-mode [-Meu] [-t target-pane]
/// ```
///
/// tmux ^1.0:
/// ```text
/// copy-mode [-u] [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// copy-mode [-u] [-t target-window]
/// ```
#[macro_export]
macro_rules! copy_mode {
    // `[-e]`
    (@cmd ($cmd:expr) -e, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.bottom_exit()
        }) $($tail)*)
    }};
    // `[-H]` - hides the position indicator in the top right
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
    // `[-q]` - cancels copy mode and any other modes
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.cancel()
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
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::copy_mode!(@cmd ({
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
    use crate::TargetPane;
    use std::borrow::Cow;

    // Enter copy mode
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // copy-mode [-eHMqu] [-s src-pane] [-t target-pane]
    // ```
    //
    // tmux ^2.1:
    // ```text
    // copy-mode [-Meu] [-t target-pane]
    // ```
    //
    // tmux ^1.0:
    // ```text
    // copy-mode [-u] [-t target-pane]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // copy-mode [-u] [-t target-window]
    // ```
    let target_pane = TargetPane::Raw("2").to_string();

    let copy_mode = copy_mode!();
    #[cfg(feature = "tmux_2_1")]
    let copy_mode = copy_mode!((copy_mode), -e);
    #[cfg(feature = "tmux_3_2")]
    let copy_mode = copy_mode!((copy_mode), -H);
    #[cfg(feature = "tmux_2_1")]
    let copy_mode = copy_mode!((copy_mode), -M);
    #[cfg(feature = "tmux_3_2")]
    let copy_mode = copy_mode!((copy_mode), -q);
    #[cfg(feature = "tmux_0_8")]
    let copy_mode = copy_mode!((copy_mode), -u);
    #[cfg(feature = "tmux_3_2")]
    let copy_mode = copy_mode!((copy_mode), -s "1");
    #[cfg(feature = "tmux_1_0")]
    let copy_mode = copy_mode!((copy_mode), -t & target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    let copy_mode = copy_mode!((copy_mode), -t & target_pane);

    let cmd = "copy-mode";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    s.push("-e");
    #[cfg(feature = "tmux_3_2")]
    s.push("-H");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_3_2")]
    s.push("-q");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let copy_mode = copy_mode.build().to_vec();

    assert_eq!(copy_mode, s);
}
