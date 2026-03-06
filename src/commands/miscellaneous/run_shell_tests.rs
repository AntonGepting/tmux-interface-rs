// auto-generated file
//

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
#[test]
fn run_shell() {
    use crate::RunShell;
    use std::borrow::Cow;

    let run_shell = RunShell::new();
    // `[-b]`
    #[cfg(feature = "tmux_1_8")]
    let run_shell = run_shell.background();

    // `[-C]`
    #[cfg(feature = "tmux_3_2")]
    let run_shell = run_shell.tmux_command();

    // `[-E]`
    #[cfg(feature = "tmux_3_6")]
    let run_shell = run_shell.redirect_stderr();

    // `[-d delay]`
    #[cfg(feature = "tmux_3_2")]
    let run_shell = run_shell.delay("1");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_8")]
    let run_shell = run_shell.target_pane("2");

    // `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    let run_shell = run_shell.shell_command("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "run-shell";
    #[cfg(feature = "cmd_alias")]
    let cmd = "run";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    v.push("-b");
    #[cfg(feature = "tmux_3_2")]
    v.push("-C");
    #[cfg(feature = "tmux_3_6")]
    v.push("-E");
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-d", "1"]);
    #[cfg(feature = "tmux_1_8")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let run_shell = run_shell.build().to_vec();

    assert_eq!(run_shell, v);
}
