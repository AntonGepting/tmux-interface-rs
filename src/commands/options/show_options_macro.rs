// auto-generated file
//

/// Show options
///
/// # Manual
///
/// tmux >=3.0a:
/// ```text
/// show-options [-AgHpqsvw] [-t target-pane] [option]
/// (alias: show)
/// ```
///
/// tmux >=1.8:
/// ```text
/// show-options [-gqsvw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux >=1.5:
/// ```text
/// show-options [-gsw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux >=0.8:
/// ```text
/// show-options [-t target-session] option value
/// (alias: show)
/// ```
#[macro_export]
macro_rules! show_options {
    // `[-A]`
    (@cmd ($cmd:expr) -A, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.include_inherited()
        }) $($tail)*)
    }};

    // `[-g]`
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};

    // `[-H]`
    (@cmd ($cmd:expr) -H, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.hooks()
        }) $($tail)*)
    }};

    // `[-p]`
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.pane()
        }) $($tail)*)
    }};

    // `[-q]`
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.quiet()
        }) $($tail)*)
    }};

    // `[-s]`
    (@cmd ($cmd:expr) -s, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.server()
        }) $($tail)*)
    }};

    // `[-v]`
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.value()
        }) $($tail)*)
    }};

    // `[-w]`
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.window()
        }) $($tail)*)
    }};

    // `[-t target-session]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_0a")))]
            {
                $cmd.target_session($target)
            }
            #[cfg(feature = "tmux_3_0a")]
            {
                $cmd.target_pane($target)
            }
        }) $($tail)*)
    }};

    // `[option]`
    (@cmd ($cmd:expr) $option:expr, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.option($option)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ShowOptions::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::show_options!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::show_options!(@cmd ({ $crate::ShowOptions::new() }) $($tail)*,)
    }};
}

#[test]
fn show_options_macro() {
    use std::borrow::Cow;

    // Show options
    //
    // # Manual
    //
    // tmux >=3.0a:
    // ```text
    // show-options [-AgHpqsvw] [-t target-pane] [option]
    // (alias: show)
    // ```
    //
    // tmux >=1.8:
    // ```text
    // show-options [-gqsvw] [-t target-session | target-window] [option]
    // (alias: show)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // show-options [-gsw] [-t target-session | target-window] [option]
    // (alias: show)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // show-options [-t target-session] option value
    // (alias: show)
    // ```

    let show_options = show_options!();
    #[cfg(feature = "tmux_3_0a")]
    let show_options = show_options!((show_options), -A);
    #[cfg(feature = "tmux_1_5")]
    let show_options = show_options!((show_options), -g);
    #[cfg(feature = "tmux_3_0a")]
    let show_options = show_options!((show_options), -H);
    #[cfg(feature = "tmux_3_0a")]
    let show_options = show_options!((show_options), -p);
    #[cfg(feature = "tmux_1_8")]
    let show_options = show_options!((show_options), -q);
    #[cfg(feature = "tmux_1_5")]
    let show_options = show_options!((show_options), -s);
    #[cfg(feature = "tmux_1_8")]
    let show_options = show_options!((show_options), -v);
    #[cfg(feature = "tmux_1_5")]
    let show_options = show_options!((show_options), -w);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_0a")))]
    let show_options = show_options!((show_options), -t "1");
    #[cfg(feature = "tmux_3_0a")]
    let show_options = show_options!((show_options), -t "2");
    #[cfg(feature = "tmux_0_8")]
    let show_options = show_options!((show_options), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "show";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_0a")]
    s.push("-A");
    #[cfg(feature = "tmux_1_5")]
    s.push("-g");
    #[cfg(feature = "tmux_3_0a")]
    s.push("-H");
    #[cfg(feature = "tmux_3_0a")]
    s.push("-p");
    #[cfg(feature = "tmux_1_8")]
    s.push("-q");
    #[cfg(feature = "tmux_1_5")]
    s.push("-s");
    #[cfg(feature = "tmux_1_8")]
    s.push("-v");
    #[cfg(feature = "tmux_1_5")]
    s.push("-w");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_0a")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_3_0a")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let show_options = show_options.build().to_vec();

    assert_eq!(show_options, s);
}
