// XXX: better result return?
/// Report if the specified session exist
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// has-session [-t target-session]
/// (alias: has)
/// ```
#[macro_export]
macro_rules! has_session {
    // `[-t target-session]` - specify the session, all clients currently attached
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::has_session!(@cmd ({
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
        $crate::HasSession::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::has_session!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::has_session!(@cmd ({ $crate::HasSession::new() }) $($tail)*,)
    }};
}

#[test]
fn has_session_macro() {
    use crate::has_session;
    use crate::HasSession;
    use std::borrow::Cow;

    // Report if the specified session exist
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // has-session [-t target-session]
    // (alias: has)
    // ```
    let has_session = has_session!();
    #[cfg(feature = "tmux_0_8")]
    let has_session = has_session!((has_session), -t "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "has-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "has";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let has_session = has_session.build().to_vec();

    assert_eq!(has_session, s);
}
