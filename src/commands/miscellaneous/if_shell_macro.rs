// XXX: solution for command [command]
// FIXME: solution for shell-command command [command]
/// Structure for conditional commands executing
///
/// # Manual
///
/// tmux ^2.0:
/// ```text
/// if-shell [-bF] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^1.8:
/// ```text
/// if-shell [-b] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^1.6:
/// ```text
/// if-shell shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^0.8:
/// ```text
/// if-shell shell-command command
/// (alias: if)
/// ```
#[macro_export]
macro_rules! if_shell {
    // `[-b]` - run in the background
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::if_shell!(@cmd ({
            $cmd.background()
        }) $($tail)*)
    }};
    // `[-F]` not execute but considered success if neither empty nor zero
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::if_shell!(@cmd ({
            $cmd.not_execute()
        }) $($tail)*)
    }};
    // `[-t target-pane]` specify the target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::if_shell!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[shell-command]`
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::if_shell!(@cmd ({
            $cmd.shell_command($shell_command)
        }) $($tail)*)
    }};
    // `[command]` - specify the second command
    (@cmd ($cmd:expr) $command:expr, $($tail:tt)*) => {{
        $crate::if_shell!(@cmd ({
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
        $crate::IfShell::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::if_shell!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::if_shell!(@cmd ({ $crate::IfShell::new() }) $($tail)*,)
    }};
}

#[test]
fn if_shell_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Structure for conditional commands executing
    //
    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // if-shell [-bF] [-t target-pane] shell-command command [command]
    // (alias: if)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // if-shell [-b] [-t target-pane] shell-command command [command]
    // (alias: if)
    // ```
    //
    // tmux ^1.6:
    // ```text
    // if-shell shell-command command [command]
    // (alias: if)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // if-shell shell-command command
    // (alias: if)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let if_shell = if_shell!();
    #[cfg(feature = "tmux_1_8")]
    let if_shell = if_shell!((if_shell), -b);
    #[cfg(feature = "tmux_2_0")]
    let if_shell = if_shell!((if_shell), -F);
    #[cfg(feature = "tmux_1_8")]
    let if_shell = if_shell!((if_shell), -t & target_pane);
    #[cfg(feature = "tmux_1_6")]
    let if_shell = if_shell!((if_shell), "2");
    #[cfg(feature = "tmux_0_8")]
    let if_shell = if_shell!((if_shell), "3");
    //#[cfg(feature = "tmux_1_6")]
    //if_shell.command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "if-shell";
    #[cfg(feature = "cmd_alias")]
    let cmd = "if";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    s.push("-b");
    #[cfg(feature = "tmux_2_0")]
    s.push("-F");
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    //#[cfg(feature = "tmux_1_6")]
    //s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let if_shell = if_shell.build().to_vec();

    assert_eq!(if_shell, s);
}
