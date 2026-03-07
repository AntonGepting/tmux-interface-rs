// auto-generated file
//

/// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// move-window [-abrdk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux >=2.1:
/// ```text
/// move-window [-ardk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux >=1.7:
/// ```text
/// move-window [-rdk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux >=1.5:
/// ```text
/// move-window [-dk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux >=0.8:
/// ```text
/// move-window [-d] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
#[macro_export]
macro_rules! move_window {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.after()
        }) $($tail)*)
    }};

    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.before()
        }) $($tail)*)
    }};

    // `[-r]`
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.renumber()
        }) $($tail)*)
    }};

    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};

    // `[-k]`
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.kill()
        }) $($tail)*)
    }};

    // `[-s src-window]`
    (@cmd ($cmd:expr) -s $src_window:expr, $($tail:tt)*) => {{
        $crate::move_window!(@cmd ({
            $cmd.src_window($src_window)
        }) $($tail)*)
    }};

    // `[-t dst-window]`
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
fn move_window_macro() {
    use std::borrow::Cow;

    // This is similar to link-window, except the window at `src-window` is moved to `dst-window`
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // move-window [-abrdk] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```
    //
    // tmux >=2.1:
    // ```text
    // move-window [-ardk] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```
    //
    // tmux >=1.7:
    // ```text
    // move-window [-rdk] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // move-window [-dk] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // move-window [-d] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```

    let move_window = move_window!();
    #[cfg(feature = "tmux_2_1")]
    let move_window = move_window!((move_window), -a);
    #[cfg(feature = "tmux_3_2")]
    let move_window = move_window!((move_window), -b);
    #[cfg(feature = "tmux_1_7")]
    let move_window = move_window!((move_window), -r);
    #[cfg(feature = "tmux_0_8")]
    let move_window = move_window!((move_window), -d);
    #[cfg(feature = "tmux_1_5")]
    let move_window = move_window!((move_window), -k);
    #[cfg(feature = "tmux_0_8")]
    let move_window = move_window!((move_window), -s "1");
    #[cfg(feature = "tmux_0_8")]
    let move_window = move_window!((move_window), -t "2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "move-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "movew";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    s.push("-a");
    #[cfg(feature = "tmux_3_2")]
    s.push("-b");
    #[cfg(feature = "tmux_1_7")]
    s.push("-r");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_5")]
    s.push("-k");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let move_window = move_window.build().to_vec();

    assert_eq!(move_window, s);
}
