/// Link the window at src-window to the specified dst-window
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// link-window [-adk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// link-window [-dk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
#[macro_export]
macro_rules! link_window {
    // `[-a]` - the window is moved to the next index up
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
            $cmd.add()
        }) $($tail)*)
    }};
    // `[-d]` - the newly linked window is not selected
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-k]` - if dst-window exists, it is killed, otherwise an error is generated
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
            $cmd.kill()
        }) $($tail)*)
    }};
    // `[-s src-window]` - src-window
    (@cmd ($cmd:expr) -s $src_window:expr, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
            $cmd.src_window($src_window)
        }) $($tail)*)
    }};
    // `[-t dst-window]` - dst-window
    (@cmd ($cmd:expr) -t $dst_window:expr, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
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
        $crate::LinkWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::link_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::link_window!(@cmd ({ $crate::LinkWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn link_window_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Link the window at src-window to the specified dst-window
    //
    // # Manual
    //
    // tmux ^2.1:
    // ```text
    // link-window [-adk] [-s src-window] [-t dst-window]
    // (alias: linkw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // link-window [-dk] [-s src-window] [-t dst-window]
    // (alias: linkw)
    // ```
    let src_window = TargetWindow::Raw("1").to_string();
    let dst_window = TargetWindow::Raw("2").to_string();

    let link_window = link_window!();
    #[cfg(feature = "tmux_2_1")]
    let link_window = link_window!((link_window), -a);
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window!((link_window), -d);
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window!((link_window), -k);
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window!((link_window), -s & src_window);
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window!((link_window), -t & dst_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "link-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "linkw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_0_8")]
    s.push("-k");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let link_window = link_window.build().to_vec();

    assert_eq!(link_window, s);
}
