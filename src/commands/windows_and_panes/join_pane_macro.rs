// auto-generated file
//

/// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
/// and move `src-pane` into the space
///
/// # Manual
///
/// tmux >=3.1:
/// ```text
/// join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
///
/// tmux >=1.7:
/// ```text
/// join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
///
/// tmux >=1.5:
/// ```text
/// join-pane [-dhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
#[macro_export]
macro_rules! join_pane {
    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.left_above()
        }) $($tail)*)
    }};

    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};

    // `[-f]`
    (@cmd ($cmd:expr) -f, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.full_size()
        }) $($tail)*)
    }};

    // `[-h]`
    (@cmd ($cmd:expr) -h, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.horizontal()
        }) $($tail)*)
    }};

    // `[-v]`
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.vertical()
        }) $($tail)*)
    }};

    // `[-l size]`
    (@cmd ($cmd:expr) -l $size:expr, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.size($size)
        }) $($tail)*)
    }};

    // `[-p percentage]`
    (@cmd ($cmd:expr) -p $percentage:expr, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.percentage($percentage)
        }) $($tail)*)
    }};

    // `[-s src-pane]`
    (@cmd ($cmd:expr) -s $src_pane:expr, $($tail:tt)*) => {{
        $crate::join_pane!(@cmd ({
            $cmd.src_pane($src_pane)
        }) $($tail)*)
    }};

    // `[-t dst-pane]`
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
fn join_pane_macro() {
    use crate::PaneSize;
    use std::borrow::Cow;

    // Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
    // and move `src-pane` into the space
    //
    // # Manual
    //
    // tmux >=3.1:
    // ```text
    // join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // ```
    //
    // tmux >=1.7:
    // ```text
    // join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // join-pane [-dhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // ```

    let join_pane = join_pane!();
    #[cfg(feature = "tmux_1_7")]
    let join_pane = join_pane!((join_pane), -b);
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane!((join_pane), -d);
    #[cfg(feature = "tmux_3_1")]
    let join_pane = join_pane!((join_pane), -f);
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane!((join_pane), -h);
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane!((join_pane), -v);
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane!((join_pane), -l & PaneSize::Size(1));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_1")))]
    let join_pane = join_pane!((join_pane), -p "2");
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane!((join_pane), -s "3");
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane!((join_pane), -t "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "join-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "joinp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    s.push("-b");
    #[cfg(feature = "tmux_1_5")]
    s.push("-d");
    #[cfg(feature = "tmux_3_1")]
    s.push("-f");
    #[cfg(feature = "tmux_1_5")]
    s.push("-h");
    #[cfg(feature = "tmux_1_5")]
    s.push("-v");
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-l", "1"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_1")))]
    s.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-s", "3"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "4"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let join_pane = join_pane.build().to_vec();

    assert_eq!(join_pane, s);
}
