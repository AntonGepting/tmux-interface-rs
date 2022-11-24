/// Search for the fnmatch(3) pattern `match-string` in window names,
/// titles, and visible content (but not history)
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// find-window [-rCNTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^2.6:
/// ```text
/// find-window [-CNT] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^1.7:
/// ```text
/// find-window [-CNT] [-F format] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^0.8:
/// ```text
/// find-window [-t target-pane] match-string
/// (alias: findw)
/// ```
#[macro_export]
macro_rules! find_window {
    // `[-r]` - regular expression
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.regex()
        }) $($tail)*)
    }};
    // `[-C]` - match only visible window contents
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.only_visible()
        }) $($tail)*)
    }};
    // `[-N]` - match only the window name
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.only_name()
        }) $($tail)*)
    }};
    // `[-T]` - match only the window title
    (@cmd ($cmd:expr) -T, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.only_title()
        }) $($tail)*)
    }};
    // `[-Z]` - zoom the pane
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::FindWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::find_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::find_window!(@cmd ({ $crate::FindWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn find_window_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // find-window [-rCNTZ] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux ^2.6:
    // ```text
    // find-window [-CNT] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux ^1.7:
    // ```text
    // find-window [-CNT] [-F format] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux ^0.8:
    // ```text
    // find-window [-t target-pane] match-string
    // (alias: findw)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let find_window = find_window!();
    #[cfg(feature = "tmux_3_0")]
    let find_window = find_window!((find_window), -r);
    #[cfg(feature = "tmux_1_7")]
    let find_window = find_window!((find_window), -C);
    #[cfg(feature = "tmux_1_7")]
    let find_window = find_window!((find_window), -N);
    #[cfg(feature = "tmux_1_7")]
    let find_window = find_window!((find_window), -T);
    #[cfg(feature = "tmux_3_0")]
    let find_window = find_window!((find_window), -Z);
    #[cfg(feature = "tmux_0_8")]
    let find_window = find_window!((find_window), -t & target_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "find-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "findw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_0")]
    s.push("-r");
    #[cfg(feature = "tmux_1_7")]
    s.push("-C");
    #[cfg(feature = "tmux_1_7")]
    s.push("-N");
    #[cfg(feature = "tmux_1_7")]
    s.push("-T");
    #[cfg(feature = "tmux_3_0")]
    s.push("-Z");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let find_window = find_window.build().to_vec();

    assert_eq!(find_window, s);
}
