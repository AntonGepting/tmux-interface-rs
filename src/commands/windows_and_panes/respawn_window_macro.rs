/// Reactivate a window in which the command has exited
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
/// [shell-command]
/// (alias: respawnw)
///
/// tmux ^2.6:
/// ```text
/// respawn-window [-k] [-c start-directory] [-t target-window]
/// [shell-command]
/// (alias: respawnw)
///
/// tmux ^1.2:
/// ```text
/// respawn-window [-k] [-t target-window] [shell-command]
/// (alias: respawnw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// respawn-window [-k] [-t target-window] [command]
/// (alias: respawnw)
/// ```
#[macro_export]
macro_rules! respawn_window {
    // `[-k]` - any existing command is killed
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::respawn_window!(@cmd ({
            $cmd.kill()
        }) $($tail)*)
    }};
    // `[-c start-directory]` - start-directory
    (@cmd ($cmd:expr) -c $start_directory:expr, $($tail:tt)*) => {{
        $crate::respawn_window!(@cmd ({
            $cmd.start_directory($start_directory)
        }) $($tail)*)
    }};
    // `[-e environment]` - environment
    // (@cmd ($cmd:expr) -e $target_session:expr, $($tail:tt)*) => {{
        // $crate::respawn_window!(@cmd ({
            // $cmd.target_session($target_session)
        // }) $($tail)*)
    // }};
    // `[-t target-window]` - target-window
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::respawn_window!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    // `[shell-command]` - shell-command
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::respawn_window!(@cmd ({
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
        $crate::RespawnWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::respawn_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::respawn_window!(@cmd ({ $crate::RespawnWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn respawn_window_tests() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Reactivate a window in which the command has exited
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
    // [shell-command]
    // (alias: respawnw)
    //
    // tmux ^2.6:
    // ```text
    // respawn-window [-k] [-c start-directory] [-t target-window]
    // [shell-command]
    // (alias: respawnw)
    //
    // tmux ^1.2:
    // ```text
    // respawn-window [-k] [-t target-window] [shell-command]
    // (alias: respawnw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // respawn-window [-k] [-t target-window] [command]
    // (alias: respawnw)
    // ```
    let target_window = TargetWindow::Raw("3").to_string();

    let respawn_window = respawn_window!();
    #[cfg(feature = "tmux_0_8")]
    let respawn_window = respawn_window!((respawn_window), -k);
    #[cfg(feature = "tmux_2_6")]
    let respawn_window = respawn_window!((respawn_window), -c "1");
    // #[cfg(feature = "tmux_3_0")]
    // let respawn_window = respawn_window!((respawn_window), -e "2");
    #[cfg(feature = "tmux_0_9")]
    let respawn_window = respawn_window!((respawn_window), -t & target_window);
    #[cfg(feature = "tmux_1_2")]
    let respawn_window = respawn_window!((respawn_window), "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "respawn-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "respawnw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-k");
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-c", "1"]);
    // #[cfg(feature = "tmux_3_0")]
    // s.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_0_9")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let respawn_window = respawn_window.build().to_vec();

    assert_eq!(respawn_window, s);
}
