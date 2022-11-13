/// Structure for creating a new session
///
/// # Manual
///
/// tmux 3.2:
/// ```text
/// new-session [-AdDEPX] [-c start-directory] [-e environment] [-f flags] [-F format]
/// [-n window-name] [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 3.0:
/// ```text
/// new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 2.4:
/// ```text
/// new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 2.1:
/// ```text
/// new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.9:
/// ```text
/// new-session [-AdDP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.8:
/// ```text
/// new-session [-AdDP] [-F format] [-n window-name] [-s session-name] [-t target-session]
/// [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.6:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [-x width]
/// [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.2:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.1:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [command]
/// (alias: new)
/// ```
///
/// tmux ^0.8:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [command]
/// (alias: new)
/// ```
#[macro_export]
macro_rules! new_session {
    // `[-A]` - behave like `attach-session` if `session-name` already exists
    (@cmd ($cmd:expr) -A, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.attach()
        }) $($tail)*)
    }};
    // `[-d]` - new session is not attached to the current terminal
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    // `[-D]` - any other clients attached to the session are detached
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.detach_other()
        }) $($tail)*)
    }};
    // `[-E]` - `update-environment` option will not be applied
    (@cmd ($cmd:expr) -E, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.not_update_env()
        }) $($tail)*)
    }};
    // `[-P]` - print information about the new session after it has been created
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.print()
        }) $($tail)*)
    }};
    // `[-X]` - send SIGHUP to the parent process, detaching the client
    (@cmd ($cmd:expr) -X, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.parent_sighup()
        }) $($tail)*)
    }};
    // `[-c start-directory]` - specify starting directory
    (@cmd ($cmd:expr) -c $start_directory:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.start_directory($start_directory)
        }) $($tail)*)
    }};
    // `[-e environment]` - takes the form ‘VARIABLE=value’ and sets an environment variable for the
    // newly created session; it may be specified multiple times
    (@cmd ($cmd:expr) -e $environment:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.environment($environment)
        }) $($tail)*)
    }};
    // `[-f flags]` - comma-separated list of client flags
    (@cmd ($cmd:expr) -e $flags:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.flags($flags)
        }) $($tail)*)
    }};
    // `[-F format]` - specify different format
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    // `[-n window-name]` - window name of the initial window
    (@cmd ($cmd:expr) -n $window_name:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.window_name($window_name)
        }) $($tail)*)
    }};
    // `[-s session-name]` - specify a session name
    (@cmd ($cmd:expr) -s $session_name:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.session_name($session_name)
        }) $($tail)*)
    }};
    // `[-t group-name]` - specify a session group
    (@cmd ($cmd:expr) -t $group_name:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.group_name($group_name)
        }) $($tail)*)
    }};
    // `[-x width]` - specify a different width
    (@cmd ($cmd:expr) -x $width:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.width($width)
        }) $($tail)*)
    }};
    // `[-y height]` - specify a different height
    (@cmd ($cmd:expr) -y $height:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.height($height)
        }) $($tail)*)
    }};
    // `[shell-command]` - shell command to execute in the initial window
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
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
        $crate::NewSession::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::new_session!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::new_session!(@cmd ({ $crate::NewSession::new() }) $($tail)*,)
    }};
}

#[test]
fn new_session_macro() {
    let cmd = new_session!();
    dbg!(cmd);
    let cmd = new_session!(-d, -E, -c "1");
    dbg!(cmd);

    //let cmds = TmuxCommands::new().
}
