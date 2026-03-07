// auto-generated file
//

/// Create a new pane by splitting target-pane
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// split-window [-bdfhIvPZ] [-c start-directory] [-e environment] [-l size] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=3.1:
/// ```text
/// split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=3.0:
/// ```text
/// split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size | -p percentage]
/// [-t target-pane] [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=2.4:
/// ```text
/// split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=2.0:
/// ```text
/// split-window [-bdhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=1.7:
/// ```text
/// split-window [-dhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=1.5:
/// ```text
/// split-window [-dhvP] [-l size | -p percentage] [-t target-pane] [shell-command]
/// (alias: splitw)
/// ```
///
/// tmux >=0.8:
/// ```text
/// split-window [-d] [-l size | -p percentage] [-t target-window] [command]
/// (alias: splitw)
/// ```
#[macro_export]
macro_rules! split_window {
    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.before()
        }) $($tail)*)
    }};

    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};

    // `[-f]`
    (@cmd ($cmd:expr) -f, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.full_size()
        }) $($tail)*)
    }};

    // `[-h]`
    (@cmd ($cmd:expr) -h, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.horizontal()
        }) $($tail)*)
    }};

    // `[-I]`
    (@cmd ($cmd:expr) -I, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.stdin_forward()
        }) $($tail)*)
    }};

    // `[-v]`
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.vertical()
        }) $($tail)*)
    }};

    // `[-P]`
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.print()
        }) $($tail)*)
    }};

    // `[-Z]`
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};

    // `[-c start-directory]`
    (@cmd ($cmd:expr) -c $start_directory:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.start_directory($start_directory)
        }) $($tail)*)
    }};

    // `[-e environment]`
    (@cmd ($cmd:expr) -e $environment:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.environment($environment)
        }) $($tail)*)
    }};

    // `[-l size]`
    // `[-l size | -p percentage]`
    (@cmd ($cmd:expr) -l $size:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.size($size)
        }) $($tail)*)
    }};

    // `[-p precentage]`
    (@cmd ($cmd:expr) -p $precentage:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.precentage($precentage)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_1_5")]
            {
                $cmd.target_pane($target)
            }
        }) $($tail)*)
    }};

    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::split_window!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[shell-command]`
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
    use crate::PaneSize;
    use std::borrow::Cow;

    // Create a new pane by splitting target-pane
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // split-window [-bdfhIvPZ] [-c start-directory] [-e environment] [-l size] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux >=3.1:
    // ```text
    // split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux >=3.0:
    // ```text
    // split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size | -p percentage]
    // [-t target-pane] [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux >=2.4:
    // ```text
    // split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux >=2.0:
    // ```text
    // split-window [-bdhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux >=1.7:
    // ```text
    // split-window [-dhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // split-window [-dhvP] [-l size | -p percentage] [-t target-pane] [shell-command]
    // (alias: splitw)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // split-window [-d] [-l size | -p percentage] [-t target-window] [command]
    // (alias: splitw)
    // ```

    let split_window = split_window!();
    #[cfg(feature = "tmux_2_0")]
    let split_window = split_window!((split_window), -b);
    #[cfg(feature = "tmux_0_8")]
    let split_window = split_window!((split_window), -d);
    #[cfg(feature = "tmux_2_4")]
    let split_window = split_window!((split_window), -f);
    #[cfg(feature = "tmux_1_5")]
    let split_window = split_window!((split_window), -h);
    #[cfg(feature = "tmux_3_0")]
    let split_window = split_window!((split_window), -I);
    #[cfg(feature = "tmux_1_5")]
    let split_window = split_window!((split_window), -v);
    #[cfg(feature = "tmux_1_5")]
    let split_window = split_window!((split_window), -P);
    #[cfg(feature = "tmux_3_2")]
    let split_window = split_window!((split_window), -Z);
    #[cfg(feature = "tmux_1_7")]
    let split_window = split_window!((split_window), -c "1");
    #[cfg(feature = "tmux_3_1")]
    let split_window = split_window!((split_window), -e "2");
    #[cfg(feature = "tmux_0_8")]
    let split_window = split_window!((split_window), -l & PaneSize::Size(3));
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
    let split_window = split_window!((split_window), -p "4");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let split_window = split_window!((split_window), -t "5");
    #[cfg(feature = "tmux_1_5")]
    let split_window = split_window!((split_window), -t "6");
    #[cfg(feature = "tmux_1_7")]
    let split_window = split_window!((split_window), -F "7");
    #[cfg(feature = "tmux_0_8")]
    let split_window = split_window!((split_window), "8");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "split-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "splitw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_0")]
    s.push("-b");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_2_4")]
    s.push("-f");
    #[cfg(feature = "tmux_1_5")]
    s.push("-h");
    #[cfg(feature = "tmux_3_0")]
    s.push("-I");
    #[cfg(feature = "tmux_1_5")]
    s.push("-v");
    #[cfg(feature = "tmux_1_5")]
    s.push("-P");
    #[cfg(feature = "tmux_3_2")]
    s.push("-Z");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-l", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
    s.extend_from_slice(&["-p", "4"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "6"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "7"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("8");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let split_window = split_window.build().to_vec();

    assert_eq!(split_window, s);
}
