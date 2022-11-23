/// Resize a pane, up, down, left or right
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// resize-pane [-DLMRTUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^2.1:
/// ```text
/// resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^1.8:
/// ```text
/// resize-pane [-DLRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^1.0:
/// ```text
/// resize-pane [-DLRU] [-t target-pane] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^0.9:
/// ```text
/// resize-pane [-DU] [-p pane-index] [-t target-pane] [adjustment]
/// (alias: resizep)
/// ```
#[macro_export]
macro_rules! resize_pane {
    // `[-D]` - resize down by adjustment
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.down()
        }) $($tail)*)
    }};
    // `[-L]` - resize left by adjustment
    (@cmd ($cmd:expr) -L, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.left()
        }) $($tail)*)
    }};
    // `[-M]` - begin mouse resizing
    (@cmd ($cmd:expr) -M, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.mouse()
        }) $($tail)*)
    }};
    // `[-R]` - resize right by adjustment
    (@cmd ($cmd:expr) -R, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.right()
        }) $($tail)*)
    }};
    // `[-T]` - trims all lines below the current cursor position
    (@cmd ($cmd:expr) -T, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.trim()
        }) $($tail)*)
    }};
    // `[-U]` - resize up by adjustment
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.up()
        }) $($tail)*)
    }};
    // `[-Z]` - the active pane is toggled between zoomed and unzoomed
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[-x width]` - absolute size
    (@cmd ($cmd:expr) -x $width:expr, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.width($width)
        }) $($tail)*)
    }};
    // `[-y height]` - absolute size
    (@cmd ($cmd:expr) -y $height:expr, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
            $cmd.height($height)
        }) $($tail)*)
    }};
    // `[adjustment]` - adjustment
    (@cmd ($cmd:expr) $adjustment:expr, $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({
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
        $crate::ResizePane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::resize_pane!(@cmd ({ $crate::ResizePane::new() }) $($tail)*,)
    }};
}

#[test]
fn resize_pane_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Resize a pane, up, down, left or right
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // resize-pane [-DLMRTUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    // (alias: resizep)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    // (alias: resizep)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // resize-pane [-DLRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    // (alias: resizep)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // resize-pane [-DLRU] [-t target-pane] [adjustment]
    // (alias: resizep)
    // ```
    //
    // tmux ^0.9:
    // ```text
    // resize-pane [-DU] [-p pane-index] [-t target-pane] [adjustment]
    // (alias: resizep)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();
    let resize_pane = resize_pane!();
    #[cfg(feature = "tmux_0_9")]
    let resize_pane = resize_pane!((resize_pane), -D);
    #[cfg(feature = "tmux_1_8")]
    let resize_pane = resize_pane!((resize_pane), -L);
    #[cfg(feature = "tmux_2_1")]
    let resize_pane = resize_pane!((resize_pane), -M);
    #[cfg(feature = "tmux_1_8")]
    let resize_pane = resize_pane!((resize_pane), -R);
    #[cfg(feature = "tmux_3_2")]
    let resize_pane = resize_pane!((resize_pane), -T);
    #[cfg(feature = "tmux_0_9")]
    let resize_pane = resize_pane!((resize_pane), -U);
    #[cfg(feature = "tmux_1_8")]
    let resize_pane = resize_pane!((resize_pane), -Z);
    #[cfg(feature = "tmux_0_9")]
    let resize_pane = resize_pane!((resize_pane), -t & target_pane);
    #[cfg(feature = "tmux_1_8")]
    let resize_pane = resize_pane!((resize_pane), -x 2);
    #[cfg(feature = "tmux_1_8")]
    let resize_pane = resize_pane!((resize_pane), -y 3);
    #[cfg(feature = "tmux_0_9")]
    let resize_pane = resize_pane!((resize_pane), "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "resize-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "resizep";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_9")]
    s.push("-D");
    #[cfg(feature = "tmux_1_8")]
    s.push("-L");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_1_8")]
    s.push("-R");
    #[cfg(feature = "tmux_3_2")]
    s.push("-T");
    #[cfg(feature = "tmux_0_9")]
    s.push("-U");
    #[cfg(feature = "tmux_1_8")]
    s.push("-Z");
    #[cfg(feature = "tmux_0_9")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-x", "2"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-y", "3"]);
    #[cfg(feature = "tmux_0_9")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let resize_pane = resize_pane.build().to_vec();

    assert_eq!(resize_pane, s);
}
