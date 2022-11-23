/// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// move-window [-abrdk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^2.1:
/// ```text
/// move-window [-ardk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^1.7:
/// ```text
/// move-window [-rdk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^1.3:
/// ```text
/// move-window [-dk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^0.8:
/// ```text
/// move-window [-d] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
#[macro_export]
macro_rules! move_window {
    // `[-a]` - the window is moved to the next index up
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.after()
        }) $($tail)*)
    }};
    // `[-b]` - the window is moved to the next index before
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.before()
        }) $($tail)*)
    }};
    // `[-r]` - all windows in the session are renumbered in sequential order
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.renumber()
        }) $($tail)*)
    }};
    // `[-d]` - the newly linked window is not selected
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-k]` - if dst-window exists, it is killed, otherwise an error is generated
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.kill()
        }) $($tail)*)
    }};
    // `[-s src-window]` - src-window
    (@cmd ($cmd:expr) -s $src_window:expr, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.src_window($src_window)
        }) $($tail)*)
    }};
    // `[-t dst-window]` - dst-window
    (@cmd ($cmd:expr) -t $dst_window:expr, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.dst_window($dst_window)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::MoveWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::move_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::move_window!(@cmd ({ $crate::MoveWindow::new() }) $($tail)*,)
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
