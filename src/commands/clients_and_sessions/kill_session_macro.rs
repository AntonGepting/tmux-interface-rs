// auto-generated file
//

/// Destroy the given session
///
/// # Manual
///
/// tmux >=2.2:
/// ```text
/// kill-session [-aC] [-t target-session]
/// ```
///
/// tmux >=1.9:
/// ```text
/// kill-session [-a] [-t target-session]
/// ```
///
/// tmux >=0.8:
/// ```text
/// kill-session [-t target-session]
/// ```
#[macro_export]
macro_rules! kill_session {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::kill_session!(@cmd ({
            $cmd.all()
        }) $($tail)*)
    }};

    // `[-C]`
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::kill_session!(@cmd ({
            $cmd.clear_alerts()
        }) $($tail)*)
    }};

    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::kill_session!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::KillSession::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::kill_session!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::kill_session!(@cmd ({ $crate::KillSession::new() }) $($tail)*,)
    }};
}

#[test]
fn kill_session_macro() {
    use std::borrow::Cow;

    // Destroy the given session
    //
    // # Manual
    //
    // tmux >=2.2:
    // ```text
    // kill-session [-aC] [-t target-session]
    // ```
    //
    // tmux >=1.9:
    // ```text
    // kill-session [-a] [-t target-session]
    // ```
    //
    // tmux >=0.8:
    // ```text
    // kill-session [-t target-session]
    // ```

    let kill_session = kill_session!();
    #[cfg(feature = "tmux_1_9")]
    let kill_session = kill_session!((kill_session), -a);
    #[cfg(feature = "tmux_2_2")]
    let kill_session = kill_session!((kill_session), -C);
    #[cfg(feature = "tmux_0_8")]
    let kill_session = kill_session!((kill_session), -t "1");

    let cmd = "kill-session";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_9")]
    s.push("-a");
    #[cfg(feature = "tmux_2_2")]
    s.push("-C");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let kill_session = kill_session.build().to_vec();

    assert_eq!(kill_session, s);
}
