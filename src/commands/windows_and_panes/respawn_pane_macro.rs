/// Reactivate a pane in which the command has exited
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
///
/// tmux ^2.6:
/// ```text
/// respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
///
/// tmux ^1.5:
/// ```text
/// respawn-pane [-k] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
#[macro_export]
macro_rules! respawn_pane {
    // `[-k]` - any existing command is killed
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::respawn_pane!(@cmd ({
            $cmd.kill()
        }) $($tail)*)
    }};
    // `[-c start-directory]` - start-directory
    (@cmd ($cmd:expr) -c $start_directory:expr, $($tail:tt)*) => {{
        $crate::respawn_pane!(@cmd ({
            $cmd.start_directory($start_directory)
        }) $($tail)*)
    }};
    // `[-e environment]` - environment
    (@cmd ($cmd:expr) -e $environment:expr, $($tail:tt)*) => {{
        $crate::respawn_pane!(@cmd ({
            $cmd.environment($environment)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::respawn_pane!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[shell-command]` - shell-command
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::respawn_pane!(@cmd ({
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
        $crate::RespawnPane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::respawn_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::respawn_pane!(@cmd ({ $crate::RespawnPane::new() }) $($tail)*,)
    }};
}

#[test]
fn respawn_pane_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Reactivate a pane in which the command has exited
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
    // (alias: respawnp)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
    // (alias: respawnp)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // respawn-pane [-k] [-t target-pane] [shell-command]
    // (alias: respawnp)
    // ```
    let target_pane = TargetPane::Raw("3").to_string();

    let respawn_pane = respawn_pane!();
    #[cfg(feature = "tmux_1_5")]
    let respawn_pane = respawn_pane!((respawn_pane), -k);
    #[cfg(feature = "tmux_2_6")]
    let respawn_pane = respawn_pane!((respawn_pane), -c "1");
    #[cfg(feature = "tmux_3_0")]
    let respawn_pane = respawn_pane!((respawn_pane), -e "2");
    #[cfg(feature = "tmux_1_5")]
    let respawn_pane = respawn_pane!((respawn_pane), -t & target_pane);
    #[cfg(feature = "tmux_2_6")]
    let respawn_pane = respawn_pane!((respawn_pane), "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "respawn-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "respawnp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-k");
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_2_6")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let respawn_pane = respawn_pane.build().to_vec();

    assert_eq!(respawn_pane, s);
}
