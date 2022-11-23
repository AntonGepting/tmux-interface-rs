/// Pipe output sent by the program in target-pane to a shell command or vice versa
///
/// # Manual
///
/// tmux ^2.7:
/// ```text
/// pipe-pane [-IOo] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
///
/// tmux ^1.2:
/// ```text
/// pipe-pane [-o] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
///
/// tmux ^1.1:
/// ```text
/// pipe-pane [-o] [-t target-pane] [command]
/// (alias: pipep)
/// ```
#[macro_export]
macro_rules! pipe_pane {
    // `[-I]` - stdin is connected
    (@cmd ($cmd:expr) -I, $($tail:tt)*) => {{
        $crate::pipe_pane!(@cmd ({
            $cmd.stdout()
        }) $($tail)*)
    }};
    // `[-O]` - stdout is connected
    (@cmd ($cmd:expr) -O, $($tail:tt)*) => {{
        $crate::pipe_pane!(@cmd ({
            $cmd.stdin()
        }) $($tail)*)
    }};
    // `[-o]` - only open a new pipe if no previous pipe exists
    (@cmd ($cmd:expr) -o, $($tail:tt)*) => {{
        $crate::pipe_pane!(@cmd ({
            $cmd.open()
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::pipe_pane!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[shell-command]` - shell-command
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::pipe_pane!(@cmd ({
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
        $crate::PipePane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::pipe_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::pipe_pane!(@cmd ({ $crate::PipePane::new() }) $($tail)*,)
    }};
}

#[test]
fn pipe_pane_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Pipe output sent by the program in target-pane to a shell command or vice versa
    //
    // # Manual
    //
    // tmux ^2.7:
    // ```text
    // pipe-pane [-IOo] [-t target-pane] [shell-command]
    // (alias: pipep)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // pipe-pane [-o] [-t target-pane] [shell-command]
    // (alias: pipep)
    // ```
    //
    // tmux ^1.1:
    // ```text
    // pipe-pane [-o] [-t target-pane] [command]
    // (alias: pipep)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();
    let pipe_pane = pipe_pane!();
    #[cfg(feature = "tmux_2_7")]
    let pipe_pane = pipe_pane!((pipe_pane), -I);
    #[cfg(feature = "tmux_2_7")]
    let pipe_pane = pipe_pane!((pipe_pane), -O);
    #[cfg(feature = "tmux_1_1")]
    let pipe_pane = pipe_pane!((pipe_pane), -o);
    #[cfg(feature = "tmux_1_1")]
    let pipe_pane = pipe_pane!((pipe_pane), -t & target_pane);
    #[cfg(feature = "tmux_1_2")]
    let pipe_pane = pipe_pane!((pipe_pane), "2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "pipe-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "pipep";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_7")]
    s.push("-I");
    #[cfg(feature = "tmux_2_7")]
    s.push("-O");
    #[cfg(feature = "tmux_1_1")]
    s.push("-o");
    #[cfg(feature = "tmux_1_1")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("2");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let pipe_pane = pipe_pane.build().to_vec();

    assert_eq!(pipe_pane, s);
}
