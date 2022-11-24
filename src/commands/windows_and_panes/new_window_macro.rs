/// Structure for creating new window, using `tmux new-window` command
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// new-window [-abdkPS] [-c start-directory] [-e environment] [-F format] [-n window-name]
/// [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^3.0:
/// ```text
/// new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name] [-t
/// target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.7:
/// ```text
/// new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
/// [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.5:
/// ```text
/// new-window [-adkP] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.3:
/// ```text
/// new-window [-adk] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.2:
/// ```text
/// new-window [-dk] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.0:
/// ```text
/// new-window [-dk] [-n window-name] [-t target-window] [command]
/// (alias: neww)
/// ```
///
/// tmux ^0.8:
/// ```text
/// new-window [-d] [-n window-name] [-t target-window] [command]
/// (alias: neww)
/// ```
#[macro_export]
macro_rules! new_window {
    // `[-a]` - new window is inserted at the next index up from the specified target-window
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.after()
        }) $($tail)*)
    }};
    // `[-b]` - new window is inserted at the next index before the specified target-window
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.before()
        }) $($tail)*)
    }};
    // `[-d]` - the session does not make the new window the current window
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-k]` - destroy if already exists
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.kill()
        }) $($tail)*)
    }};
    // `[-P]` - print information about the new window after it has been created
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.print()
        }) $($tail)*)
    }};
    // `[-S]` - is given and a window named window-name already exists, it is selected
    (@cmd ($cmd:expr) -S, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.select()
        }) $($tail)*)
    }};
    // `[-c start-directory]` - start-directory
    (@cmd ($cmd:expr) -c $start_directory:expr, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.start_directory($start_directory)
        }) $($tail)*)
    }};
    // `[-e environment]` - environment
    (@cmd ($cmd:expr) -e $environment:expr, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.environment($environment)
        }) $($tail)*)
    }};
    // `[-F format]` - format
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    // `[-n window-name]` - window-name
    (@cmd ($cmd:expr) -n $window_name:expr, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.window_name($window_name)
        }) $($tail)*)
    }};
    // `[-t target-window]` - target-window
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    // `[shell-command]` - shell-command
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::new_window!(@cmd ({
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
        $crate::NewWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::new_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::new_window!(@cmd ({ $crate::NewWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn new_window_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Structure for creating new window, using `tmux new-window` command
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // new-window [-abdkPS] [-c start-directory] [-e environment] [-F format] [-n window-name]
    // [-t target-window] [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^3.0:
    // ```text
    // new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name] [-t
    // target-window] [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
    // [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // new-window [-adkP] [-n window-name] [-t target-window] [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.3:
    // ```text
    // new-window [-adk] [-n window-name] [-t target-window] [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // new-window [-dk] [-n window-name] [-t target-window] [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // new-window [-dk] [-n window-name] [-t target-window] [command]
    // (alias: neww)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // new-window [-d] [-n window-name] [-t target-window] [command]
    // (alias: neww)
    // ```
    let target_window = TargetWindow::Raw("5").to_string();

    let new_window = new_window!();
    #[cfg(feature = "tmux_1_3")]
    let new_window = new_window!((new_window), -a);
    #[cfg(feature = "tmux_3_2")]
    let new_window = new_window!((new_window), -b);
    #[cfg(feature = "tmux_0_8")]
    let new_window = new_window!((new_window), -d);
    #[cfg(feature = "tmux_1_0")]
    let new_window = new_window!((new_window), -k);
    #[cfg(feature = "tmux_1_5")]
    let new_window = new_window!((new_window), -P);
    #[cfg(feature = "tmux_3_2")]
    let new_window = new_window!((new_window), -S);
    #[cfg(feature = "tmux_1_7")]
    let new_window = new_window!((new_window), -c "1");
    #[cfg(feature = "tmux_3_0")]
    let new_window = new_window!((new_window), -e "2");
    #[cfg(feature = "tmux_1_7")]
    let new_window = new_window!((new_window), -F "3");
    #[cfg(feature = "tmux_0_8")]
    let new_window = new_window!((new_window), -n "4");
    #[cfg(feature = "tmux_0_8")]
    let new_window = new_window!((new_window), -t & target_window);
    #[cfg(feature = "tmux_1_2")]
    let new_window = new_window!((new_window), "6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "new-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "neww";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_3")]
    s.push("-a");
    #[cfg(feature = "tmux_3_2")]
    s.push("-b");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_0")]
    s.push("-k");
    #[cfg(feature = "tmux_1_5")]
    s.push("-P");
    #[cfg(feature = "tmux_3_2")]
    s.push("-S");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-n", "4"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let new_window = new_window.build().to_vec();

    assert_eq!(new_window, s);
}
