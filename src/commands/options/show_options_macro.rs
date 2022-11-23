/// Structure for showing options
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// show-options [-AgHpqsvw] [-t target-pane] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.8:
/// ```text
/// show-options [-gqsvw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.7:
/// ```text
/// show-options [-gsw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.2:
/// ```text
/// show-options [-gsw] [-t target-session | target-window]
/// (alias: show)
/// ```
///
/// tmux ^1.0:
/// ```text
/// show-options [-t target-session]
/// (alias: show)
/// ```
///
/// tmux ^0.8:
/// ```text
/// show-options [-t target-session] option value
/// (alias: show)
/// ```
// XXX: better result type?
#[macro_export]
macro_rules! show_options {
    // `[-A]` - includes options inherited from a parent set of options
    (@cmd ($cmd:expr) -A, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.include_inherited()
        }) $($tail)*)
    }};
    // `[-g]` - global session or window options are listed
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};
    // `[-H]` - includes hooks (omitted by default)
    (@cmd ($cmd:expr) -H, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.hooks()
        }) $($tail)*)
    }};
    // `[-p]` - show window options
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.pane()
        }) $($tail)*)
    }};
    // `[-q]` - no error will be returned if `option` is unset
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.quiet()
        }) $($tail)*)
    }};
    // `[-s]` - show the server options
    (@cmd ($cmd:expr) -s, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.server()
        }) $($tail)*)
    }};
    // `[-v]` - shows only the option value
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.value()
        }) $($tail)*)
    }};
    // `[-w]` - show the window options
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.window()
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target session or window name
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.target($target)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::show_options!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[option]` - specify option name
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
    use crate::TargetPane;
    use std::borrow::Cow;

    // Structure for showing options
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // show-options [-AgHpqsvw] [-t target-pane] [option]
    // (alias: show)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // show-options [-gqsvw] [-t target-session | target-window] [option]
    // (alias: show)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // show-options [-gsw] [-t target-session | target-window] [option]
    // (alias: show)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // show-options [-gsw] [-t target-session | target-window]
    // (alias: show)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // show-options [-t target-session]
    // (alias: show)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // show-options [-t target-session] option value
    // (alias: show)
    // ```

    let target_pane = TargetPane::Raw("1").to_string();

    let show_options = show_options!();
    #[cfg(feature = "tmux_3_0")]
    let show_options = show_options!((show_options), -A);
    #[cfg(feature = "tmux_1_2")]
    let show_options = show_options!((show_options), -g);
    #[cfg(feature = "tmux_3_0")]
    let show_options = show_options!((show_options), -H);
    #[cfg(feature = "tmux_3_0")]
    let show_options = show_options!((show_options), -p);
    #[cfg(feature = "tmux_1_8")]
    let show_options = show_options!((show_options), -q);
    #[cfg(feature = "tmux_1_2")]
    let show_options = show_options!((show_options), -s);
    #[cfg(feature = "tmux_1_8")]
    let show_options = show_options!((show_options), -v);
    #[cfg(feature = "tmux_1_2")]
    let show_options = show_options!((show_options), -w);
    let show_options = show_options!((show_options), -t & target_pane);
    #[cfg(feature = "tmux_1_7")]
    let show_options = show_options!((show_options), "2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "show";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_0")]
    s.push("-A");
    #[cfg(feature = "tmux_1_2")]
    s.push("-g");
    #[cfg(feature = "tmux_3_0")]
    s.push("-H");
    #[cfg(feature = "tmux_3_0")]
    s.push("-p");
    #[cfg(feature = "tmux_1_8")]
    s.push("-q");
    #[cfg(feature = "tmux_1_2")]
    s.push("-s");
    #[cfg(feature = "tmux_1_8")]
    s.push("-v");
    #[cfg(feature = "tmux_1_2")]
    s.push("-w");
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_7")]
    s.push("2");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let show_options = show_options.build().to_vec();

    assert_eq!(show_options, s);
}
