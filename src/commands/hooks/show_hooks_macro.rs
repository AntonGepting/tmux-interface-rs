/// # Manual
///
/// tmux ^2.2:
/// ```text
/// show-hooks [-g] [-t target-session]
/// ```
#[macro_export]
macro_rules! show_hooks {
    // `[-g]`
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};
    // `[-s target-session]` - specify the session, all clients currently attached
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ({
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
        $crate::ShowHooks::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::show_hooks!(@cmd ({ $crate::ShowHooks::new() }) $($tail)*,)
    }};
}

#[test]
fn show_hooks_macro() {
    use crate::TargetSession;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^2.2:
    // ```text
    // show-hooks [-g] [-t target-session]
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let show_hooks = show_hooks!();
    #[cfg(feature = "tmux_2_2")]
    let show_hooks = show_hooks!((show_hooks), -g);
    #[cfg(feature = "tmux_2_2")]
    let show_hooks = show_hooks!((show_hooks), -t & target_session);

    let cmd = "show-hooks";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_2")]
    s.push("-g");
    #[cfg(feature = "tmux_2_2")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let show_hooks = show_hooks.build().to_vec();

    assert_eq!(show_hooks, s);
}
