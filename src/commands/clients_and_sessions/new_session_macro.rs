// auto-generated file
//

/// Create new session
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// new-session [-AdDEPX] [-c start-directory] [-e environment] [-f flags] [-F format]
/// [-n window-name] [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=3.0:
/// ```text
/// new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=2.4:
/// ```text
/// new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=2.1:
/// ```text
/// new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.9:
/// ```text
/// new-session [-AdDP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.8:
/// ```text
/// new-session [-AdDP] [-F format] [-n window-name] [-s session-name] [-t target-session]
/// [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.6:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [-x width]
/// [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.2:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.1:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [command]
/// (alias: new)
/// ```
///
/// tmux >=0.8:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [command]
/// (alias: new)
/// ```
#[macro_export]
macro_rules! new_session {
    // `[-A]`
    (@cmd ($cmd:expr) -A, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.attach()
        }) $($tail)*)
    }};

    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};

    // `[-D]`
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.detach_other()
        }) $($tail)*)
    }};

    // `[-E]`
    (@cmd ($cmd:expr) -E, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.not_update_env()
        }) $($tail)*)
    }};

    // `[-P]`
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.print()
        }) $($tail)*)
    }};

    // `[-X]`
    (@cmd ($cmd:expr) -X, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.parent_sighup()
        }) $($tail)*)
    }};

    // `[-c start-directory]`
    (@cmd ($cmd:expr) -c $start_directory:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.start_directory($start_directory)
        }) $($tail)*)
    }};

    // TODO:
    // `[-e environment]` - takes the form ‘VARIABLE=value’ and sets an environment variable for the
    // newly created session; it may be specified multiple times
    // (@cmd ($cmd:expr) -e $environment:expr, $($tail:tt)*) => {{
        // $crate::new_session!(@cmd ({
            // $cmd.environment($environment)
        // }) $($tail)*)
    // }};

    // `[-f flags]`
    (@cmd ($cmd:expr) -f $flags:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.flags($flags)
        }) $($tail)*)
    }};

    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-n window-name]`
    (@cmd ($cmd:expr) -n $window_name:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.window_name($window_name)
        }) $($tail)*)
    }};

    // `[-s session-name]`
    (@cmd ($cmd:expr) -s $session_name:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.session_name($session_name)
        }) $($tail)*)
    }};

    // `[-t target-session]`
    // `[-t group-name]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
            {
                $cmd.target_session($target)
            }
            #[cfg(feature = "tmux_2_4")]
            {
                $cmd.group_name($target)
            }
        }) $($tail)*)
    }};

    // `[-x width]`
    (@cmd ($cmd:expr) -x $width:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.width($width)
        }) $($tail)*)
    }};

    // `[-y height]`
    (@cmd ($cmd:expr) -y $height:expr, $($tail:tt)*) => {{
        $crate::new_session!(@cmd ({
            $cmd.height($height)
        }) $($tail)*)
    }};

    // `[shell-command]`
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
    #[cfg(feature = "tmux_3_2")]
    use crate::ClientFlags;
    use std::borrow::Cow;

    // Create new session
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // new-session [-AdDEPX] [-c start-directory] [-e environment] [-f flags] [-F format]
    // [-n window-name] [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux >=3.0:
    // ```text
    // new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t group-name] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux >=2.4:
    // ```text
    // new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t group-name] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux >=2.1:
    // ```text
    // new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t target-session] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux >=1.9:
    // ```text
    // new-session [-AdDP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t target-session] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux >=1.8:
    // ```text
    // new-session [-AdDP] [-F format] [-n window-name] [-s session-name] [-t target-session]
    // [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux >=1.6:
    // ```text
    // new-session [-d] [-n window-name] [-s session-name] [-t target-session] [-x width]
    // [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux >=1.2:
    // ```text
    // new-session [-d] [-n window-name] [-s session-name] [-t target-session] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux >=1.1:
    // ```text
    // new-session [-d] [-n window-name] [-s session-name] [-t target-session] [command]
    // (alias: new)
    // ```
    //
    // tmux >=0.8:
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
    #[cfg(feature = "tmux_3_0a")]
    let new_session = new_session!((new_session), -X);
    #[cfg(feature = "tmux_1_9")]
    let new_session = new_session!((new_session), -c "1");
    // #[cfg(feature = "tmux_3_2")]
    // let new_session = new_session!((new_session), -e "2=3");
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
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
    let new_session = new_session!((new_session), -t "7");
    #[cfg(feature = "tmux_2_4")]
    let new_session = new_session!((new_session), -t "8");
    #[cfg(feature = "tmux_1_5")]
    let new_session = new_session!((new_session), -x 9);
    #[cfg(feature = "tmux_1_5")]
    let new_session = new_session!((new_session), -y 10);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let new_session = new_session!((new_session), "11");
    #[cfg(feature = "tmux_1_5")]
    let new_session = new_session!((new_session), "12");

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
    #[cfg(feature = "tmux_3_0a")]
    s.push("-X");
    #[cfg(feature = "tmux_1_9")]
    s.extend_from_slice(&["-c", "1"]);
    //#[cfg(feature = "tmux_3_2")]
    //s.extend_from_slice(&["-e", "2=3"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-f", "active-pane"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-F", "4"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-n", "5"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "6"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "7"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-t", "8"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-x", "9"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-y", "10"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.push("11");
    #[cfg(feature = "tmux_1_5")]
    s.push("12");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let new_session = new_session.build().to_vec();

    assert_eq!(new_session, s);
}
