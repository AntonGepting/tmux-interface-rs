// NOTE: conditionals in macro problem, solution partial macro with condition and main macro
// ```
// #[doc(hidden)]
// #[macro_export]
// macro_rules! attach_session_pt1_ {}
// ```
// no need for conditionals, error in generated code controls
// error[E0599]: no method named `parent_sighup` found for struct `attach_session::AttachSession` in the current scope
//
/// Generate command using flags from TMUX manual
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// attach-session [-dErx] [-c working-directory] [-f flags] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^3.0:
/// ```text
/// attach-session [-dErx] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^2.1:
/// ```text
/// attach-session [-dEr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^1.9:
/// ```text
/// attach-session [-dr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^1.2:
/// ```text
/// attach-session [-dr] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^0.8:
/// ```text
/// attach-session [-d] [-t target-session]
/// (alias: attach)
/// ```
#[macro_export]
macro_rules! attach_session {
    // `[-d]` - any other clients attached to the session are detached
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({
            $cmd.detach_other()
        }) $($tail)*)
    }};
    // `[-E]` - `update-environment` option will not be applied
    (@cmd ($cmd:expr) -E, $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({
            $cmd.not_update_env()
        }) $($tail)*)
    }};
    // `[-r]` - signifies the client is read-only
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({
            $cmd.read_only()
        }) $($tail)*)
    }};
    // `[-x]` - send SIGHUP to the parent process, detaching the client
    (@cmd ($cmd:expr) -x, $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({
            $cmd.parent_sighup()
        }) $($tail)*)
    }};
    // `[-c working-directory]` - specify starting directory
    (@cmd ($cmd:expr) -c $working_directory:expr, $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({
            $cmd.working_directory($working_directory)
        }) $($tail)*)
    }};
    // `[-f flags]` - sets a comma-separated list of client flags
    (@cmd ($cmd:expr) -f $flags:expr, $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({
            $cmd.flags($flags)
        }) $($tail)*)
    }};
    // `[-t target-session]` - specify target session name
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter");
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::AttachSession::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({ $crate::AttachSession::new() }) $($tail)*,)
    }};
}

#[test]
fn attach_session_macro() {
    use crate::attach_session;
    #[cfg(feature = "tmux_3_2")]
    use crate::ClientFlags;
    use crate::{AttachSession, TargetSession};
    use std::borrow::Cow;

    // Structure for attaching client to already existing session
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // attach-session [-dErx] [-c working-directory] [-f flags] [-t target-session]
    // (alias: attach)
    // ```
    //
    // tmux ^3.0:
    // ```text
    // attach-session [-dErx] [-c working-directory] [-t target-session]
    // (alias: attach)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // attach-session [-dEr] [-c working-directory] [-t target-session]
    // (alias: attach)
    // ```
    //
    // tmux ^1.9:
    // ```text
    // attach-session [-dr] [-c working-directory] [-t target-session]
    // (alias: attach)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // attach-session [-dr] [-t target-session]
    // (alias: attach)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // attach-session [-d] [-t target-session]
    // (alias: attach)
    // ```
    let target_session = TargetSession::Raw("2").to_string();

    let attach_session = attach_session!();
    #[cfg(feature = "tmux_0_8")]
    let attach_session = attach_session!((attach_session), -d);
    #[cfg(feature = "tmux_2_1")]
    let attach_session = attach_session!((attach_session), -E);
    #[cfg(feature = "tmux_1_2")]
    let attach_session = attach_session!((attach_session), -r);
    #[cfg(feature = "tmux_3_0")]
    let attach_session = attach_session!((attach_session), -x);
    #[cfg(feature = "tmux_1_9")]
    let attach_session = attach_session!((attach_session), -c "1");
    #[cfg(feature = "tmux_3_2")]
    let flags = ClientFlags {
        active_pane: Some(true),
        ..Default::default()
    };
    #[cfg(feature = "tmux_3_2")]
    let attach_session = attach_session!((attach_session), -f flags);
    #[cfg(feature = "tmux_0_8")]
    let attach_session = attach_session!((attach_session), -t & target_session);
    #[cfg(feature = "tmux_0_8")]
    let attach_session = attach_session!((attach_session), -t target_session.to_string());

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "attach-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "attach";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_2_1")]
    s.push("-E");
    #[cfg(feature = "tmux_1_2")]
    s.push("-r");
    #[cfg(feature = "tmux_3_0")]
    s.push("-x");
    #[cfg(feature = "tmux_1_9")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-f", "active-pane"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let attach_session = attach_session.build().to_vec();

    assert_eq!(attach_session, s);
}
