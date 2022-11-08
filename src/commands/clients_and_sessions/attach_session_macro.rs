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
    (@cmd ($cmd:expr) -c $working_directory:tt, $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({
            $cmd.working_directory($working_directory)
        }) $($tail)*)
    }};
    // `[-f flags]` - sets a comma-separated list of client flags
    (@cmd ($cmd:expr) -f $flags:tt, $($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({
            $cmd.flags($flags)
        }) $($tail)*)
    }};
    // `[-t target-session]` - specify target session name
    (@cmd ($cmd:expr) -t $target_session:tt, $($tail:tt)*) => {{
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
    ($($tail:tt)*) => {{
        $crate::attach_session!(@cmd ({ $crate::AttachSession::new() }) $($tail)*,)
    }};
}

#[test]
fn attach_session_macro() {
    let cmd = attach_session!();
    dbg!(cmd);
    let cmd = attach_session!(-d, -E, -r, -c "1", -t "2");
    dbg!(cmd);
    //let cmd = attach_session!(-x, -c "2");
    //dbg!(cmd);
}
