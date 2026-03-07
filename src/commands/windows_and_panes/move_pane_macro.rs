// auto-generated file
//

/// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// move-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
///
/// tmux >=3.1:
/// ```text
/// move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
///
/// tmux >=1.7:
/// ```text
/// move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
#[macro_export]
macro_rules! move_pane {
    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.left_above()
        }) $($tail)*)
    }};

    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};

    // `[-f]`
    (@cmd ($cmd:expr) -f, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.full_size()
        }) $($tail)*)
    }};

    // `[-h]`
    (@cmd ($cmd:expr) -h, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.horizontal()
        }) $($tail)*)
    }};

    // `[-v]`
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.vertical()
        }) $($tail)*)
    }};

    // `[-l size]`
    (@cmd ($cmd:expr) -l $size:expr, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.size($size)
        }) $($tail)*)
    }};

    // `[-p percentage]`
    (@cmd ($cmd:expr) -p $percentage:expr, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.percentage($percentage)
        }) $($tail)*)
    }};

    // `[-s src-pane]`
    (@cmd ($cmd:expr) -s $src_pane:expr, $($tail:tt)*) => {{
        $crate::move_pane!(@cmd ({
            $cmd.src_pane($src_pane)
        }) $($tail)*)
    }};

    // `[-t dst-pane]`
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
    use crate::PaneSize;
    use std::borrow::Cow;

    // Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // move-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
    // (alias: movep)
    // ```
    //
    // tmux >=3.1:
    // ```text
    // move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
    // (alias: movep)
    // ```
    //
    // tmux >=1.7:
    // ```text
    // move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: movep)
    // ```

    let move_pane = move_pane!();
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -b);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -d);
    #[cfg(feature = "tmux_3_2")]
    let move_pane = move_pane!((move_pane), -f);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -h);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -v);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -l & PaneSize::Size(1));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_3_1")))]
    let move_pane = move_pane!((move_pane), -p "2");
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -s "3");
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane!((move_pane), -t "4");

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
    #[cfg(feature = "tmux_3_2")]
    s.push("-f");
    #[cfg(feature = "tmux_1_7")]
    s.push("-h");
    #[cfg(feature = "tmux_1_7")]
    s.push("-v");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-l", "1"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_3_1")))]
    s.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-s", "3"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-t", "4"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let move_pane = move_pane.build().to_vec();

    assert_eq!(move_pane, s);
}
