// auto-generated file
//

/// Link the window at src-window to the specified dst-window
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// link-window [-abdk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
///
/// tmux >=2.1:
/// ```text
/// link-window [-adk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
///
/// tmux >=0.8:
/// ```text
/// link-window [-dk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
#[macro_export]
macro_rules! link_window {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
            $cmd.after()
        }) $($tail)*)
    }};

    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
            $cmd.before()
        }) $($tail)*)
    }};

    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};

    // `[-k]`
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
            $cmd.kill()
        }) $($tail)*)
    }};

    // `[-s src-window]`
    (@cmd ($cmd:expr) -s $src_window:expr, $($tail:tt)*) => {{
        $crate::link_window!(@cmd ({
            $cmd.src_window($src_window)
        }) $($tail)*)
    }};

    // `[-t dst-window]`
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
    use std::borrow::Cow;

    // Link the window at src-window to the specified dst-window
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // link-window [-abdk] [-s src-window] [-t dst-window]
    // (alias: linkw)
    // ```
    //
    // tmux >=2.1:
    // ```text
    // link-window [-adk] [-s src-window] [-t dst-window]
    // (alias: linkw)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // link-window [-dk] [-s src-window] [-t dst-window]
    // (alias: linkw)
    // ```

    let link_window = link_window!();
    #[cfg(feature = "tmux_2_1")]
    let link_window = link_window!((link_window), -a);
    #[cfg(feature = "tmux_3_2")]
    let link_window = link_window!((link_window), -b);
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window!((link_window), -d);
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window!((link_window), -k);
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window!((link_window), -s "1");
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window!((link_window), -t "2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "link-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "linkw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    s.push("-a");
    #[cfg(feature = "tmux_3_2")]
    s.push("-b");
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
