/// Create a new pane by splitting target-pane
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// split-window [-bdfhIvPZ] [-c start-directory] [-e environment] [-l size] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^3.1:
/// ```text
/// split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^3.0:
/// ```text
/// split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size | -p percentage]
/// [-t target-pane] [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^2.4:
/// ```text
/// split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^2.0:
/// ```text
/// split-window [-bdhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^1.7:
/// ```text
/// split-window [-dhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^1.5:
/// ```text
/// split-window [-dhvP] [-l size | -p percentage] [-t target-pane] [shell-command]
/// (alias: splitw)
/// ```
///
/// tmux ^1.2:
/// ```text
/// split-window [-dhv] [-l size | -p percentage] [-t target-pane] [shell-command]
/// (alias: splitw)
/// ```
///
/// tmux ^1.0:
/// ```text
/// split-window [-dhv] [-l size | -p percentage] [-t target-window] [command]
/// (alias: splitw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// split-window [-d] [-l size | -p percentage] [-t target-window] [command]
/// (alias: splitw)
/// ```
#[macro_export]
macro_rules! split_window {
    // `[-b]` - cause the new pane to be created to the left of or above target-pane
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.before()
        }) $($tail)*)
    }};
    // `[-d]` - do not make the new window the current window
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-f]` - creates a new pane spanning the full window size (h, v)
    (@cmd ($cmd:expr) -f, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.full()
        }) $($tail)*)
    }};
    // `[-h]` - horizontal split
    (@cmd ($cmd:expr) -h, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.horizontal()
        }) $($tail)*)
    }};
    // `[-I]` - create an empty pane and forward any output from stdin to it
    (@cmd ($cmd:expr) -I, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.stdin_forward()
        }) $($tail)*)
    }};
    // `[-v]` - vertical split
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.vertical()
        }) $($tail)*)
    }};
    // `[-P]` - print information about the new window after it has been created
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.print()
        }) $($tail)*)
    }};
    // `[-Z]` - print information about the new window after it has been created
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};
    // `[-c start_directory]` - start-directory
    (@cmd ($cmd:expr) -c $start_directory:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.start_directory($start_directory)
        }) $($tail)*)
    }};
    // `[-e environment]` - environment
    (@cmd ($cmd:expr) -e $environment:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.environment($environment)
        }) $($tail)*)
    }};
    // `[-l size]` - specify the size of the new pane in lines
    // `[-l size | -p percentage]` - specify the size of the new pane in lines
    (@cmd ($cmd:expr) -l $size:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.size($size)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -p $percentage:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.size($percentage)
        }) $($tail)*)
    }};
    // `[-t target-pane]` -
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[-t target-window]` -
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    // `[-F format]` - format
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    // `[shell-command]` - shell-command
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
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
        $crate::SplitWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::split_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::split_window!(@cmd ({ $crate::SplitWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn split_window_macro() {
    use crate::{PaneSize, TargetPane};
    use std::borrow::Cow;

    // Create a new pane by splitting target-pane
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^3.0:
    // ```text
    // split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size | -p percentage]
    // [-t target-pane] [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // split-window [-bdhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // split-window [-dhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // split-window [-dhvP] [-l size | -p percentage] [-t target-pane] [shell-command]
    // (alias: splitw)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // split-window [-dhv] [-l size | -p percentage] [-t target-pane] [shell-command]
    // (alias: splitw)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // split-window [-dhv] [-l size | -p percentage] [-t target-window] [command]
    // (alias: splitw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // split-window [-d] [-l size | -p percentage] [-t target-window] [command]
    // (alias: splitw)
    // ```
    let target_pane = TargetPane::Raw("5").to_string();

    let split_window = split_window!();
    #[cfg(feature = "tmux_2_4")]
    let split_window = split_window!((split_window), -b);
    #[cfg(feature = "tmux_0_8")]
    let split_window = split_window!((split_window), -d);
    #[cfg(feature = "tmux_2_4")]
    let split_window = split_window!((split_window), -f);
    #[cfg(feature = "tmux_1_0")]
    let split_window = split_window!((split_window), -h);
    #[cfg(feature = "tmux_3_0")]
    let split_window = split_window!((split_window), -I);
    #[cfg(feature = "tmux_1_0")]
    let split_window = split_window!((split_window), -v);
    #[cfg(feature = "tmux_1_5")]
    let split_window = split_window!((split_window), -P);
    #[cfg(feature = "tmux_1_7")]
    let split_window = split_window!((split_window), -c "1");
    #[cfg(feature = "tmux_3_1")]
    let split_window = split_window!((split_window), -e "2");
    #[cfg(feature = "tmux_0_8")]
    let split_window = split_window!((split_window), -l & PaneSize::Size(3));
    #[cfg(feature = "tmux_1_7")]
    let split_window = split_window!((split_window), -F "4");
    #[cfg(feature = "tmux_1_2")]
    let split_window = split_window!((split_window), -t & target_pane);
    #[cfg(feature = "tmux_1_2")]
    let split_window = split_window!((split_window), "6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "split-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "splitw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_4")]
    s.push("-b");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_2_4")]
    s.push("-f");
    #[cfg(feature = "tmux_1_0")]
    s.push("-h");
    #[cfg(feature = "tmux_3_0")]
    s.push("-I");
    #[cfg(feature = "tmux_1_0")]
    s.push("-v");
    #[cfg(feature = "tmux_1_5")]
    s.push("-P");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-l", "3"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "4"]);
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let split_window = split_window.build().to_vec();

    assert_eq!(split_window, s);
}
