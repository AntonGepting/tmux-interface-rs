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
fn new_session() {
    #[cfg(feature = "tmux_3_2")]
    use crate::ClientFlags;
    use std::borrow::Cow;

    // Structure for creating a new session
    //
    // # Manual
    //
    // tmux 3.2:
    // ```text
    // new-session [-AdDEPX] [-c start-directory] [-e environment] [-f flags] [-F format]
    // [-n window-name] [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 3.0:
    // ```text
    // new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t group-name] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 2.4:
    // ```text
    // new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t group-name] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 2.1:
    // ```text
    // new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t target-session] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.9:
    // ```text
    // new-session [-AdDP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t target-session] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.8:
    // ```text
    // new-session [-AdDP] [-F format] [-n window-name] [-s session-name] [-t target-session]
    // [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.6:
    // ```text
    // new-session [-d] [-n window-name] [-s session-name] [-t target-session] [-x width]
    // [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.2:
    // ```text
    // new-session [-d] [-n window-name] [-s session-name] [-t target-session] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.1:
    // ```text
    // new-session [-d] [-n window-name] [-s session-name] [-t target-session] [command]
    // (alias: new)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // new-session [-d] [-n window-name] [-s session-name] [command]
    // (alias: new)
    // ```
    let new_session = new_session!();
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session!((new_session), -A);
    #[cfg(feature = "tmux_0_8")]
    let new_session = new_session!((new_session), -d);
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session!((new_session), -D);
    #[cfg(feature = "tmux_2_1")]
    let new_session = new_session!((new_session), -E);
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session!((new_session), -P);
    #[cfg(feature = "tmux_3_0")]
    let new_session = new_session!((new_session), -X);
    #[cfg(feature = "tmux_1_9")]
    let new_session = new_session!((new_session), -c "1");
    #[cfg(feature = "tmux_3_2")]
    let new_session = new_session!((new_session), -e "2=3");
    #[cfg(feature = "tmux_3_2")]
    let flags = ClientFlags {
        active_pane: Some(true),
        ..Default::default()
    };
    #[cfg(feature = "tmux_3_2")]
    let new_session = new_session!((new_session), -f flags);
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session!((new_session), -F "4");
    #[cfg(feature = "tmux_0_8")]
    let new_session = new_session!((new_session), -n "5");
    #[cfg(feature = "tmux_0_8")]
    let new_session = new_session!((new_session), -s "6");
    #[cfg(feature = "tmux_2_4")]
    let new_session = new_session!((new_session), -t "7");
    #[cfg(feature = "tmux_1_6")]
    let new_session = new_session!((new_session), -x 8);
    #[cfg(feature = "tmux_1_6")]
    let new_session = new_session!((new_session), -y 9);
    #[cfg(feature = "tmux_1_2")]
    let new_session = new_session!((new_session), "10");

    //let new = new_session.to_tmux_bin_command();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "new-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "new";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    s.push("-A");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_8")]
    s.push("-D");
    #[cfg(feature = "tmux_2_1")]
    s.push("-E");
    #[cfg(feature = "tmux_1_8")]
    s.push("-P");
    #[cfg(feature = "tmux_3_0")]
    s.push("-X");
    #[cfg(feature = "tmux_1_9")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-e", "2=3"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-f", "active-pane"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-F", "4"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-n", "5"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "6"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-t", "7"]);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-x", "8"]);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-y", "9"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("10");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let new_session = new_session.build().to_vec();

    assert_eq!(new_session, s);
}
