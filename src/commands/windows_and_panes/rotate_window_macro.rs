/// Rotate the positions of the panes within a window
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// rotate-window [-DUZ] [-t target-window]
/// (alias: rotatew)
/// ```
///
/// tmux ^0.8:
/// ```text
/// rotate-window [-DU] [-t target-window]
/// (alias: rotatew)
/// ```
#[macro_export]
macro_rules! rotate_window {
    // `[-D]`
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::rotate_window!(@cmd ({
            $cmd.down()
        }) $($tail)*)
    }};
    // `[-U]`
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::rotate_window!(@cmd ({
            $cmd.up()
        }) $($tail)*)
    }};
    // `[-Z]`
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::rotate_window!(@cmd ({
            $cmd.keep_zoomed()
        }) $($tail)*)
    }};
    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::rotate_window!(@cmd ({
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
        $crate::RotateWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::rotate_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::rotate_window!(@cmd ({ $crate::RotateWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn rotate_window_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Rotate the positions of the panes within a window
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // rotate-window [-DUZ] [-t target-window]
    // (alias: rotatew)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // rotate-window [-DU] [-t target-window]
    // (alias: rotatew)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let rotate_window = rotate_window!();
    #[cfg(feature = "tmux_0_8")]
    let rotate_window = rotate_window!((rotate_window), -D);
    #[cfg(feature = "tmux_0_8")]
    let rotate_window = rotate_window!((rotate_window), -U);
    #[cfg(feature = "tmux_3_1")]
    let rotate_window = rotate_window!((rotate_window), -Z);
    #[cfg(feature = "tmux_0_8")]
    let rotate_window = rotate_window!((rotate_window), -t & target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rotate-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "rotatew";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-D");
    #[cfg(feature = "tmux_0_8")]
    s.push("-U");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let rotate_window = rotate_window.build().to_vec();

    assert_eq!(rotate_window, s);
}
