/// Structure for setting a pane/window/session/server option
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// set-option [-aFgopqsuUw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux ^3.0:
/// ```text
/// set-option [-aFgopqsuw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux ^2.6:
/// ```text
/// set-option [-aFgoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.8:
/// ```text
/// set-option [-agoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.7:
/// ```text
/// set-option [-agqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.2:
/// ```text
/// set-option [-agsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.0:
/// ```text
/// set-option [-agu] [-t target-session] option value
/// (alias: set)
/// ```
///
/// tmux ^0.8:
/// ```text
/// set-option [-gu] [-t target-session] option value
/// (alias: set)
/// ```
#[macro_export]
macro_rules! set_option {
    // `[-a]` - value is appended to the existing setting, if the option expects a string or a style
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.append()
        }) $($tail)*)
    }};
    // `[-F]` - expand formats in the option value
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.format()
        }) $($tail)*)
    }};
    // `[-g]` - the global session or window option is set
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};
    // `[-o]` - prevents setting an option that is already set
    (@cmd ($cmd:expr) -o, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.not_overwrite()
        }) $($tail)*)
    }};
    // `[-p]` - set a pane option
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.pane()
        }) $($tail)*)
    }};
    // `[-q]` - suppress errors about unknown or ambiguous options
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.quiet()
        }) $($tail)*)
    }};
    // `[-s]` - set a server option
    (@cmd ($cmd:expr) -s, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.server()
        }) $($tail)*)
    }};
    // `[-u]` - unset an option, so a session inherits the option from the global options
    (@cmd ($cmd:expr) -u, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.unset()
        }) $($tail)*)
    }};
    // `[-U]` - unsets an option (like -u) but if the option is a pane option also unsets the option on any panes in the window
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.unset_on_all()
        }) $($tail)*)
    }};
    // `[-w]` - set a window option
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.window()
        }) $($tail)*)
    }};
    // `[-t target-session | target-window]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.target($target)
        }) $($tail)*)
    }};
    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - specify the target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    // FIXME: option valuer pair in one fn
    // `option`
    (@cmd ($cmd:expr) $option:expr, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.option($option)
        }) $($tail)*)
    }};
    // `value`
    (@cmd ($cmd:expr) $value:expr, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.value($value)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SetOption::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::set_option!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::set_option!(@cmd ({ $crate::SetOption::new() }) $($tail)*,)
    }};
}

#[test]
fn set_option_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Structure for setting a pane/window/session/server option
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // set-option [-aFgopqsuUw] [-t target-pane] option value
    // (alias: set)
    // ```
    //
    // tmux ^3.0:
    // ```text
    // set-option [-aFgopqsuw] [-t target-pane] option value
    // (alias: set)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // set-option [-aFgoqsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // set-option [-agoqsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // set-option [-agqsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // set-option [-agsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // set-option [-agu] [-t target-session] option value
    // (alias: set)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // set-option [-gu] [-t target-session] option value
    // (alias: set)
    // ```
    // FIXME: target, target-sesion, target-window
    let target_pane = TargetPane::Raw("1").to_string();

    let set_option = set_option!();
    #[cfg(feature = "tmux_1_0")]
    let set_option = set_option!((set_option), -a);
    #[cfg(feature = "tmux_2_6")]
    let set_option = set_option!((set_option), -F);
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option!((set_option), -g);
    #[cfg(feature = "tmux_1_8")]
    let set_option = set_option!((set_option), -o);
    #[cfg(feature = "tmux_3_0")]
    let set_option = set_option!((set_option), -p);
    #[cfg(feature = "tmux_1_7")]
    let set_option = set_option!((set_option), -q);
    #[cfg(feature = "tmux_1_2")]
    let set_option = set_option!((set_option), -s);
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option!((set_option), -u);
    #[cfg(feature = "tmux_3_2")]
    let set_option = set_option!((set_option), -U);
    #[cfg(feature = "tmux_1_2")]
    let set_option = set_option!((set_option), -w);
    #[cfg(feature = "tmux_3_0")]
    let set_option = set_option!((set_option), -t & target_pane);
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    let set_option = set_option!((set_option), -t & target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    let set_option = set_option!((set_option), -t & target_pane);
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option!((set_option), "2");
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option!((set_option), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "set";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_0")]
    s.push("-a");
    #[cfg(feature = "tmux_2_6")]
    s.push("-F");
    #[cfg(feature = "tmux_0_8")]
    s.push("-g");
    #[cfg(feature = "tmux_1_8")]
    s.push("-o");
    #[cfg(feature = "tmux_3_0")]
    s.push("-p");
    #[cfg(feature = "tmux_1_7")]
    s.push("-q");
    #[cfg(feature = "tmux_1_2")]
    s.push("-s");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_3_2")]
    s.push("-U");
    #[cfg(feature = "tmux_1_2")]
    s.push("-w");
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let set_option = set_option.build().to_vec();

    assert_eq!(set_option, s);
}
