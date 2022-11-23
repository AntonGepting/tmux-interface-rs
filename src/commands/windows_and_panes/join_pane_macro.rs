/// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
/// and move `src-pane` into the space
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
///
/// tmux ^1.7:
/// ```text
/// join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
///
/// tmux ^1.2:
/// ```text
/// join-pane [-dhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
#[macro_export]
macro_rules! join_pane {
    // `[-b]` - cause src-pane to be joined to left of or above dst-pane
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.left_above()
        }) $($tail)*)
    }};
    // `[-d]` -
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-f]` - creates a new pane spanning the full window height/width
    (@cmd ($cmd:expr) -f, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.full_size()
        }) $($tail)*)
    }};
    // `[-h]` - full height
    (@cmd ($cmd:expr) -h, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.horizontal()
        }) $($tail)*)
    }};
    // `[-v]` - full width
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.vertical()
        }) $($tail)*)
    }};
    // `[-l size]`
    // `[-l size | -p percentage]` - specify the size of the new pane in lines/columns
    (@cmd ($cmd:expr) -l $size:expr, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.size($size)
        }) $($tail)*)
    }};
    // `[-s src-pane]` - src-pane
    (@cmd ($cmd:expr) -s $src_pane:expr, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.src_pane($src_pane)
        }) $($tail)*)
    }};
    // `[-t dst-pane]` - dst-pane
    (@cmd ($cmd:expr) -t $dst_pane:expr, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
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
        $crate::JoinPane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({ $crate::JoinPane::new() }) $($tail)*,)
    }};
}

// TODO: size and percentage both test
#[test]
fn join_pane() {
    use crate::{PaneSize, TargetPane};
    use std::borrow::Cow;

    // Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
    // and move `src-pane` into the space
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // join-pane [-dhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // ```
    let src_pane = TargetPane::Raw("2").to_string();
    let dst_pane = TargetPane::Raw("3").to_string();

    let join_pane = join_pane!();
    #[cfg(feature = "tmux_2_6")]
    let join_pane = join_pane!((join_pane), -b);
    #[cfg(feature = "tmux_1_2")]
    let join_pane = join_pane!((join_pane), -d);
    #[cfg(feature = "tmux_2_6")]
    let join_pane = join_pane!((join_pane), -f);
    #[cfg(feature = "tmux_1_2")]
    let join_pane = join_pane!((join_pane), -h);
    #[cfg(feature = "tmux_1_2")]
    let join_pane = join_pane!((join_pane), -v);
    #[cfg(feature = "tmux_1_2")]
    let join_pane = join_pane!((join_pane), -l & PaneSize::Percentage(1));
    #[cfg(feature = "tmux_1_2")]
    let join_pane = join_pane!((join_pane), -s & src_pane);
    #[cfg(feature = "tmux_1_2")]
    let join_pane = join_pane!((join_pane), -t & dst_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "join-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "joinp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_6")]
    s.push("-b");
    #[cfg(feature = "tmux_1_2")]
    s.push("-d");
    #[cfg(feature = "tmux_2_6")]
    s.push("-f");
    #[cfg(feature = "tmux_1_2")]
    s.push("-h");
    #[cfg(feature = "tmux_1_2")]
    s.push("-v");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_1")))]
    s.extend_from_slice(&["-l", "1%"]);
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-p", "1"]);
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-s", "2"]);
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let join_pane = join_pane.build().to_vec();

    assert_eq!(join_pane, s);
}
