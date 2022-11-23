/// # Manual
///
/// tmux ^3.2:
/// ```text
/// run-shell [-bC] [-d delay] [-t target-pane] [shell-command]
/// (alias: run)
/// ```
///
/// tmux ^1.8:
/// ```text
/// run-shell [-b] [-t target-pane] shell-command
/// (alias: run)
/// ```
///
/// tmux ^1.2:
/// ```text
/// run-shell shell-command
/// (alias: run)
/// ```
///
/// tmux ^1.1:
/// ```text
/// run-shell command
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
    // `[-C]` - execute tmux command
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.tmux_command()
        }) $($tail)*)
    }};
    // `[-d delay]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.delay()
        }) $($tail)*)
    }};
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `shell-command`
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.shell_command($shell_command)
        }) $($tail)*)
    }};
    // `command`
    (@cmd ($cmd:expr) $command:expr, $($tail:tt)*) => {{
        $crate::run_shell!(@cmd ({
            $cmd.command($command)
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
    use crate::TargetPane;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // run-shell [-bC] [-d delay] [-t target-pane] [shell-command]
    // (alias: run)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // run-shell [-b] [-t target-pane] shell-command
    // (alias: run)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // run-shell shell-command
    // (alias: run)
    // ```
    //
    // tmux ^1.1:
    // ```text
    // run-shell command
    // (alias: run)
    // ```
    let target_pane = TargetPane::Raw("2").to_string();

    let run_shell = run_shell!();
    #[cfg(feature = "tmux_1_8")]
    let run_shell = run_shell!((run_shell), -b);
    #[cfg(feature = "tmux_3_2")]
    let run_shell = run_shell!((run_shell), -C);
    #[cfg(feature = "tmux_3_2")]
    let run_shell = run_shell!((run_shell), -1);
    #[cfg(feature = "tmux_1_8")]
    let run_shell = run_shell!((run_shell), -t & target_pane);
    #[cfg(feature = "tmux_1_2")]
    let run_shell = run_shell!((run_shell), "3");
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
    let run_shell = run_shell!((run_shell), "4");

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
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-d", "1"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("3");
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let run_shell = run_shell.build().to_vec();

    assert_eq!(run_shell, s);
}
