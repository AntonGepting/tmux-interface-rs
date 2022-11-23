/// Resize a window, up, down, left or right
///
/// # Manual
///
/// tmux ^2.9:
/// ```text
/// resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
/// (alias: resizew)
/// ```
#[macro_export]
macro_rules! resize_window {
    // `[-a]` - set the size of the smallest session containing the window
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.smallest()
        }) $($tail)*)
    }};
    // `[-A]` - set the size of the largest session containing the window
    (@cmd ($cmd:expr) -A, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.largest()
        }) $($tail)*)
    }};
    // `[-D]` - resize down by adjustment
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.down()
        }) $($tail)*)
    }};
    // `[-L]` - resize left by adjustment
    (@cmd ($cmd:expr) -L, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.left()
        }) $($tail)*)
    }};
    // `[-R]` - resize right by adjustment
    (@cmd ($cmd:expr) -R, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.right()
        }) $($tail)*)
    }};
    // `[-U]` - resize up by adjustment
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.up()
        }) $($tail)*)
    }};
    // `[-t target-window]` - target-window
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    // `[-x width]` - absolute size
    (@cmd ($cmd:expr) -x $width:expr, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.width($width)
        }) $($tail)*)
    }};
    // `[-y height]` - absolute size
    (@cmd ($cmd:expr) -y $height:expr, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.height($height)
        }) $($tail)*)
    }};
    // `[adjustment]` - adjustment
    (@cmd ($cmd:expr) $adjustment:expr, $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({
            $cmd.adjustment($adjustment)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ResizeWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::resize_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::resize_window!(@cmd ({ $crate::ResizeWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn resize_window_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Resize a window, up, down, left or right
    //
    // # Manual
    //
    // tmux ^2.9a:
    // ```text
    // resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
    // (alias: resizew)
    let target_window = TargetWindow::Raw("1").to_string();

    let resize_window = resize_window!();
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), -a);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), -A);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), -D);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), -L);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), -R);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), -U);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), -t & target_window);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), -x 2);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), -y 3);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window!((resize_window), "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "resize-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "resizew";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    s.push("-a");
    #[cfg(feature = "tmux_2_9")]
    s.push("-A");
    #[cfg(feature = "tmux_2_9")]
    s.push("-D");
    #[cfg(feature = "tmux_2_9")]
    s.push("-L");
    #[cfg(feature = "tmux_2_9")]
    s.push("-R");
    #[cfg(feature = "tmux_2_9")]
    s.push("-U");
    #[cfg(feature = "tmux_2_9")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_2_9")]
    s.extend_from_slice(&["-x", "2"]);
    #[cfg(feature = "tmux_2_9")]
    s.extend_from_slice(&["-y", "3"]);
    #[cfg(feature = "tmux_2_9")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let resize_window = resize_window.build().to_vec();

    assert_eq!(resize_window, s);
}
