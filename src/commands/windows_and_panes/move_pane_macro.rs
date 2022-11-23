/// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
///
/// tmux ^1.7:
/// ```text
/// move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
#[macro_export]
macro_rules! move_pane {
    // `[-b]` - cause src-pane to be joined to left of or above dst-pane
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.left_above()
        }) $($tail)*)
    }};
    // `[-d]` -
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-h]` - full height
    (@cmd ($cmd:expr) -h, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.horizontal()
        }) $($tail)*)
    }};
    // `[-v]` - full width
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.vertical()
        }) $($tail)*)
    }};

    // `[-l size]` - specify the size of the new pane in lines/columns
    (@cmd ($cmd:expr) -l $size:expr, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.size($size)
        }) $($tail)*)
    }};
    // `[-s src-pane]` - src-pane
    (@cmd ($cmd:expr) -s $src_pane:expr, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.src_pane($src_pane)
        }) $($tail)*)
    }};
    // `[-t dst-pane]` - dst-pane
    (@cmd ($cmd:expr) -t $dst_pane:expr, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
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
        $crate::MovePane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({ $crate::MovePane::new() }) $($tail)*,)
    }};
}

#[test]
fn move_pane_macro() {
    use crate::{PaneSize, TargetPane};
    use std::borrow::Cow;

    // Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
    // (alias: movep)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: movep)
    // ```
    let src_pane = TargetPane::Raw("2").to_string();
    let dst_pane = TargetPane::Raw("3").to_string();

    let move_pane = move_pane!();
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -b);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -d);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -h);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -v);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -l & PaneSize::Size(1));
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -s & src_pane);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -t & dst_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "move-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "movep";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    s.push("-b");
    #[cfg(feature = "tmux_1_7")]
    s.push("-d");
    #[cfg(feature = "tmux_1_7")]
    s.push("-h");
    #[cfg(feature = "tmux_1_7")]
    s.push("-v");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-l", "1"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-s", "2"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let move_pane = move_pane.build().to_vec();

    assert_eq!(move_pane, s);
}
