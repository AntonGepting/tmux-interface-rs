// auto-generated file
//

/// Shows hooks
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// show-hooks [-gpw] [-t target-session]
/// ```
///
/// tmux >=2.2:
/// ```text
/// show-hooks [-g] [-t target-session]
/// ```
#[macro_export]
macro_rules! show_hooks {
    // `[-g]`
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};

    // `[-p]`
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ({
            $cmd.pane()
        }) $($tail)*)
    }};

    // `[-w]`
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ({
            $cmd.window()
        }) $($tail)*)
    }};

    // `[-t target-session]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ({
            #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_3_2")))]
            {
                $cmd.target_session($target)
            }
            #[cfg(feature = "tmux_3_2")]
            {
                $cmd.target_pane($target)
            }
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ShowHooks::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ({ $crate::ShowHooks::new() }) $($tail)*,)
    }};
}

#[test]
fn show_hooks_macro() {
    use std::borrow::Cow;

    // Shows hooks
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // show-hooks [-gpw] [-t target-session]
    // ```
    //
    // tmux >=2.2:
    // ```text
    // show-hooks [-g] [-t target-session]
    // ```

    let show_hooks = show_hooks!();
    #[cfg(feature = "tmux_2_2")]
    let show_hooks = show_hooks!((show_hooks), -g);
    #[cfg(feature = "tmux_3_2")]
    let show_hooks = show_hooks!((show_hooks), -p);
    #[cfg(feature = "tmux_3_2")]
    let show_hooks = show_hooks!((show_hooks), -w);
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_3_2")))]
    let show_hooks = show_hooks!((show_hooks), -t "1");
    #[cfg(feature = "tmux_3_2")]
    let show_hooks = show_hooks!((show_hooks), -t "2");

    let cmd = "show-hooks";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_2")]
    s.push("-g");
    #[cfg(feature = "tmux_3_2")]
    s.push("-p");
    #[cfg(feature = "tmux_3_2")]
    s.push("-w");
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_3_2")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let show_hooks = show_hooks.build().to_vec();

    assert_eq!(show_hooks, s);
}
