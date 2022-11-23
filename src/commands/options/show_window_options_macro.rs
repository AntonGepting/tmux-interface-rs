/// # Manual
///
/// tmux ^3.0:
/// ```text
/// (removed)
/// ```
///
/// tmux ^1.8:
/// ```text
/// show-window-options [-gv] [-t target-window] [option]
/// (alias: showw)
/// ```
///
/// tmux ^1.7:
/// ```text
/// show-window-options [-g] [-t target-window] [option]
/// (alias: showw)
/// ```
///
/// tmux ^1.0:
/// ```text
/// show-window-options [-g] [-t target-window]
/// (alias: showw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// show-window-options [-t target-window] option value
/// (alias: showw)
/// ```
#[macro_export]
macro_rules! show_window_options {
    // `[-g]`
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::show_window_options!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};
    // `[-v]`
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::show_window_options!(@cmd ({
            $cmd.only_value()
        }) $($tail)*)
    }};
    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::show_window_options!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    // `option`
    (@cmd ($cmd:expr) $option:expr, $($tail:tt)*) => {{
        $crate::show_window_options!(@cmd ({
            $cmd.option($option)
        }) $($tail)*)
    }};
    // `value`
    (@cmd ($cmd:expr) $value:expr, $($tail:tt)*) => {{
        $crate::show_window_options!(@cmd ({
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
        $crate::ShowWindowOptions::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::show_window_options!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::show_window_options!(@cmd ({ $crate::ShowWindowOptions::new() }) $($tail)*,)
    }};
}

#[test]
fn show_window_options_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // tmux ^3.0:
    // ```text
    // (removed)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // show-window-options [-gv] [-t target-window] [option]
    // (alias: showw)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // show-window-options [-g] [-t target-window] [option]
    // (alias: showw)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // show-window-options [-g] [-t target-window]
    // (alias: showw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // show-window-options [-t target-window] option value
    // (alias: showw)
    // ```
    // (alias: showw)
    let target_window = TargetWindow::Raw("1").to_string();

    let show_window_options = show_window_options!();
    #[cfg(feature = "tmux_1_0")]
    let show_window_options = show_window_options!((show_window_options), -g);
    #[cfg(feature = "tmux_1_8")]
    let show_window_options = show_window_options!((show_window_options), -v);
    #[cfg(feature = "tmux_0_8")]
    let show_window_options = show_window_options!((show_window_options), -t & target_window);
    #[cfg(feature = "tmux_0_8")]
    let show_window_options = show_window_options!((show_window_options), "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    let show_window_options = show_window_options!((show_window_options), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-window-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_0")]
    s.push("-g");
    #[cfg(feature = "tmux_1_8")]
    s.push("-v");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let show_window_options = show_window_options.build().to_vec();

    assert_eq!(show_window_options, s);
}
