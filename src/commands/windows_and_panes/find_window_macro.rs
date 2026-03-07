// auto-generated file
//

/// Search for the fnmatch(3) pattern `match-string` in window names,
/// titles, and visible content (but not history)
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// find-window [-iCNrTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=3.0a:
/// ```text
/// find-window [-rCNTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=2.9:
/// ```text
/// find-window [-CNTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=2.6:
/// ```text
/// find-window [-CNT] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=1.7:
/// ```text
/// find-window [-CNT] [-F format] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=0.8:
/// ```text
/// find-window [-t target-pane] match-string
/// (alias: findw)
/// ```
#[macro_export]
macro_rules! find_window {
    // `[-i]`
    (@cmd ($cmd:expr) -i, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.ignore_case()
        }) $($tail)*)
    }};

    // `[-C]`
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.only_visible()
        }) $($tail)*)
    }};

    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.only_name()
        }) $($tail)*)
    }};

    // `[-r]`
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.regex()
        }) $($tail)*)
    }};

    // `[-T]`
    (@cmd ($cmd:expr) -T, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.only_title()
        }) $($tail)*)
    }};

    // `[-Z]`
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};

    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_6")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_2_6")]
            {
                $cmd.target_pane($target)
            }
        }) $($tail)*)
    }};

    // `[match-string]`
    (@cmd ($cmd:expr) $match_string:expr, $($tail:tt)*) => {{
        $crate::find_window!(@cmd ({
            $cmd.match_string($match_string)
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
    use std::borrow::Cow;

    // Search for the fnmatch(3) pattern `match-string` in window names,
    // titles, and visible content (but not history)
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // find-window [-iCNrTZ] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux >=3.0a:
    // ```text
    // find-window [-rCNTZ] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux >=2.9:
    // ```text
    // find-window [-CNTZ] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux >=2.6:
    // ```text
    // find-window [-CNT] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux >=1.7:
    // ```text
    // find-window [-CNT] [-F format] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux >=0.8:
    // ```text
    // find-window [-t target-pane] match-string
    // (alias: findw)
    // ```

    let find_window = find_window!();
    #[cfg(feature = "tmux_3_2")]
    let find_window = find_window!((find_window), -i);
    #[cfg(feature = "tmux_1_7")]
    let find_window = find_window!((find_window), -C);
    #[cfg(feature = "tmux_1_7")]
    let find_window = find_window!((find_window), -N);
    #[cfg(feature = "tmux_3_0a")]
    let find_window = find_window!((find_window), -r);
    #[cfg(feature = "tmux_1_7")]
    let find_window = find_window!((find_window), -T);
    #[cfg(feature = "tmux_2_9")]
    let find_window = find_window!((find_window), -Z);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let find_window = find_window!((find_window), -F "1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_6")))]
    let find_window = find_window!((find_window), -t "2");
    #[cfg(feature = "tmux_2_6")]
    let find_window = find_window!((find_window), -t "3");
    #[cfg(feature = "tmux_0_8")]
    let find_window = find_window!((find_window), "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "find-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "findw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-i");
    #[cfg(feature = "tmux_1_7")]
    s.push("-C");
    #[cfg(feature = "tmux_1_7")]
    s.push("-N");
    #[cfg(feature = "tmux_3_0a")]
    s.push("-r");
    #[cfg(feature = "tmux_1_7")]
    s.push("-T");
    #[cfg(feature = "tmux_2_9")]
    s.push("-Z");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let find_window = find_window.build().to_vec();

    assert_eq!(find_window, s);
}
