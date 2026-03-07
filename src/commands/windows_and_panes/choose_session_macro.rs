// auto-generated file
//

/// Put a window into session choice mode
///
/// # Manual
///
/// tmux =1.7:
/// ```text
/// choose-session [-F format] [-t target-window] [template]
/// ```
///
/// tmux >=1.5:
/// ```text
/// choose-session [-t target-window] [template]
/// ```
///
/// tmux >=0.8:
/// ```text
/// choose-session [-t target-window]
/// ```
#[macro_export]
macro_rules! choose_session {
    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::choose_session!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::choose_session!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};

    // `[template]`
    (@cmd ($cmd:expr) $template:expr, $($tail:tt)*) => {{
        $crate::choose_session!(@cmd ({
            $cmd.template($template)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ChooseSession::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::choose_session!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::choose_session!(@cmd ({ $crate::ChooseSession::new() }) $($tail)*,)
    }};
}

#[test]
fn choose_session_macro() {
    use std::borrow::Cow;

    // Put a window into session choice mode
    //
    // # Manual
    //
    // tmux =1.7:
    // ```text
    // choose-session [-F format] [-t target-window] [template]
    // ```
    //
    // tmux >=1.5:
    // ```text
    // choose-session [-t target-window] [template]
    // ```
    //
    // tmux >=0.8:
    // ```text
    // choose-session [-t target-window]
    // ```

    let choose_session = choose_session!();
    #[cfg(feature = "tmux_1_7")]
    let choose_session = choose_session!((choose_session), -F "1");
    #[cfg(feature = "tmux_0_8")]
    let choose_session = choose_session!((choose_session), -t "2");
    #[cfg(feature = "tmux_1_5")]
    let choose_session = choose_session!((choose_session), "3");

    let cmd = "choose-session";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let choose_session = choose_session.build().to_vec();

    assert_eq!(choose_session, s);
}
