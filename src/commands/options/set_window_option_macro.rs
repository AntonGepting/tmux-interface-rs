/// # Manual
///
/// tmux ^3.0:
/// ```text
/// (removed)
/// ```
///
/// tmux ^2.6:
/// ```text
/// set-window-option [-aFgoqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.9:
/// ```text
/// set-window-option [-agoqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.7:
/// ```text
/// set-window-option [-agqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.0:
/// ```text
/// set-window-option [-agu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// set-window-option [-gu] [-t target-window] option value
/// (alias: setw)
/// ```
#[macro_export]
macro_rules! set_window_option {
    // `[-a]` -
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.append()
        }) $($tail)*)
    }};
    // `[-F]` -
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.format()
        }) $($tail)*)
    }};
    // `[-g]` -
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};
    // `[-o]` -
    (@cmd ($cmd:expr) -o, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.not_overwrite()
        }) $($tail)*)
    }};
    // `[-q]` -
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.quiet()
        }) $($tail)*)
    }};
    // `[-u]` -
    (@cmd ($cmd:expr) -u, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.unset()
        }) $($tail)*)
    }};
    // `[-t target-window]` -
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    // `option`
    (@cmd ($cmd:expr) $option:expr, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.option($option)
        }) $($tail)*)
    }};
    // `value`
    (@cmd ($cmd:expr) $value:expr, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.value($value)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({
            $cmd.shell_command($shell_command)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SetWindowOption::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::set_window_option!(@cmd ({ $crate::SetWindowOption::new() }) $($tail)*,)
    }};
}

#[test]
fn set_window_option_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // (removed)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // set-window-option [-aFgoqu] [-t target-window] option value
    // (alias: setw)
    // ```
    //
    // tmux ^1.9:
    // ```text
    // set-window-option [-agoqu] [-t target-window] option value
    // (alias: setw)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // set-window-option [-agqu] [-t target-window] option value
    // (alias: setw)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // set-window-option [-agu] [-t target-window] option value
    // (alias: setw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // set-window-option [-gu] [-t target-window] option value
    // (alias: setw)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let set_window_option = set_window_option!();
    #[cfg(feature = "tmux_1_0")]
    let set_window_option = set_window_option!((set_window_option), -a);
    #[cfg(feature = "tmux_2_6")]
    let set_window_option = set_window_option!((set_window_option), -F);
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option!((set_window_option), -g);
    #[cfg(feature = "tmux_1_9")]
    let set_window_option = set_window_option!((set_window_option), -o);
    #[cfg(feature = "tmux_1_7")]
    let set_window_option = set_window_option!((set_window_option), -q);
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option!((set_window_option), -u);
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option!((set_window_option), -t & target_window);
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option!((set_window_option), "2");
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option!((set_window_option), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-window-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_0")]
    s.push("-a");
    #[cfg(feature = "tmux_2_6")]
    s.push("-F");
    #[cfg(feature = "tmux_0_8")]
    s.push("-g");
    #[cfg(feature = "tmux_1_9")]
    s.push("-o");
    #[cfg(feature = "tmux_1_7")]
    s.push("-q");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let set_window_option = set_window_option.build().to_vec();

    assert_eq!(set_window_option, s);
}
