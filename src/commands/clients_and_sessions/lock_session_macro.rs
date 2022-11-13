/// Lock all clients attached to `target-session`
/// # Manual
///
/// tmux ^1.1:
/// ```text
/// lock-session [-t target-session]
/// (alias: locks)
/// ```
#[macro_export]
macro_rules! lock_session {
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::lock_session!(@cmd ({
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
        $crate::LockSession::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::lock_session!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::lock_session!(@cmd ({ $crate::LockSession::new() }) $($tail)*,)
    }};

}

#[test]
fn lock_session_macro() {
    use crate::LockSession;
    use std::borrow::Cow;

    // Lock all clients attached to `target-session`
    // # Manual
    //
    // tmux ^1.1:
    // ```text
    // lock-session [-t target-session]
    // (alias: locks)
    // ```
    let lock_session = lock_session!();
    #[cfg(feature = "tmux_1_1")]
    let lock_session = lock_session!((lock_session), -t "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "locks";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_1")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let lock_session = lock_session.build().to_vec();

    assert_eq!(lock_session, s);
}
