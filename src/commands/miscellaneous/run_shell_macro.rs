// auto-generated file
//

/// Execute shell-command using `/bin/sh`
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// run-shell [-bCE] [-d delay] [-t target-pane] [shell-command]
/// (alias: run)
/// ```
///
/// tmux >=3.2:
/// ```text
/// run-shell [-bC] [-d delay] [-t target-pane] [shell-command]
/// (alias: run)
/// ```
///
/// tmux >=1.8:
/// ```text
/// run-shell [-b] [-t target-pane] shell-command
/// (alias: run)
/// ```
///
/// tmux >=1.5:
/// ```text
/// run-shell shell-command
/// (alias: run)
/// ```
#[macro_export]
macro_rules! run_shell {
    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.background()
        }) $($tail)*)
    }};

    // `[-C]`
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.tmux_command()
        }) $($tail)*)
    }};

    // `[-E]`
    (@cmd ($cmd:expr) -E, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.redirect_stderr()
        }) $($tail)*)
    }};

    // `[-d delay]`
    (@cmd ($cmd:expr) -d $delay:expr, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.delay($delay)
        }) $($tail)*)
    }};

    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};

    // `[shell-command]`
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
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
        $crate::RunShell::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({ $crate::RunShell::new() }) $($tail)*,)
    }};
}

#[test]
fn run_shell_macro() {
    use std::borrow::Cow;

    // Execute shell-command using `/bin/sh`
    //
    // # Manual
    //
    // tmux >=3.6:
    // ```text
    // run-shell [-bCE] [-d delay] [-t target-pane] [shell-command]
    // (alias: run)
    // ```
    //
    // tmux >=3.2:
    // ```text
    // run-shell [-bC] [-d delay] [-t target-pane] [shell-command]
    // (alias: run)
    // ```
    //
    // tmux >=1.8:
    // ```text
    // run-shell [-b] [-t target-pane] shell-command
    // (alias: run)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // run-shell shell-command
    // (alias: run)
    // ```

    let run_shell = run_shell!();
    #[cfg(feature = "tmux_1_8")]
    let run_shell = run_shell!((run_shell), -b);
    #[cfg(feature = "tmux_3_2")]
    let run_shell = run_shell!((run_shell), -C);
    #[cfg(feature = "tmux_3_6")]
    let run_shell = run_shell!((run_shell), -E);
    #[cfg(feature = "tmux_3_2")]
    let run_shell = run_shell!((run_shell), -d "1");
    #[cfg(feature = "tmux_1_8")]
    let run_shell = run_shell!((run_shell), -t "2");
    #[cfg(feature = "tmux_1_5")]
    let run_shell = run_shell!((run_shell), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "run-shell";
    #[cfg(feature = "cmd_alias")]
    let cmd = "run";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    s.push("-b");
    #[cfg(feature = "tmux_3_2")]
    s.push("-C");
    #[cfg(feature = "tmux_3_6")]
    s.push("-E");
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-d", "1"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let run_shell = run_shell.build().to_vec();

    assert_eq!(run_shell, s);
}
