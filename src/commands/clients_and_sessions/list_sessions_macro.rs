/// List all sessions managed by the server
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// list-sessions [-F format]
/// (alias: ls)
/// ```
///
/// tmux ^0.8:
/// ```text
/// list-sessions
/// (alias: ls)
/// ```
#[macro_export]
macro_rules! list_sessions {
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::list_sessions!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter");
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ListSessions::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::list_sessions!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::list_sessions!(@cmd ({ $crate::ListSessions::new() }) $($tail)*,)
    }};

}

#[test]
fn list_sessions_macro() {
    use std::borrow::Cow;

    // List all sessions managed by the server
    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // list-sessions [-F format]
    // (alias: ls)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // list-sessions
    // (alias: ls)
    // ```
    let list_sessions = list_sessions!();
    #[cfg(feature = "tmux_1_6")]
    let list_sessions = list_sessions!((list_sessions), -F "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-sessions";
    #[cfg(feature = "cmd_alias")]
    let cmd = "ls";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_sessions = list_sessions.build().to_vec();

    assert_eq!(list_sessions, s);
}
