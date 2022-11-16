/// Structure for setting or unsetting hook `hook-name` to command.
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// set-hook [-agRu] [-t target-session] hook-name command
/// ```
///
/// tmux ^2.8:
/// ```text
/// set-hook [-gRu] [-t target-session] hook-name command
/// ```
///
/// tmux ^2.4:
/// ```text
/// set-hook [-gu] [-t target-session] hook-name command
/// ```
///
/// tmux ^2.2:
/// ```text
/// set-hook [-g] [-t target-session] hook-name command
/// ```
// FIXME: two parameters
#[macro_export]
macro_rules! set_hook {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::set_hook!(@cmd ({
            $cmd.append()
        }) $($tail)*)
    }};
    // `[-g]`
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::set_hook!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};
    // `[-R]`
    (@cmd ($cmd:expr) -R, $($tail:tt)*) => {{
        $crate::set_hook!(@cmd ({
            $cmd.run()
        }) $($tail)*)
    }};
    // `[-u]`
    (@cmd ($cmd:expr) -u, $($tail:tt)*) => {{
        $crate::set_hook!(@cmd ({
            $cmd.unset()
        }) $($tail)*)
    }};
    // `[-s target-session]` - specify the session, all clients currently attached
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::set_hook!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    // `hook-name`
    (@cmd ($cmd:expr) $hook_name:expr, $($tail:tt)*) => {{
        $crate::set_hook!(@cmd ({
            $cmd.hook_name($hook_name)
        }) $($tail)*)
    }};
    // `command`
    (@cmd ($cmd:expr) $command:expr, $($tail:tt)*) => {{
        $crate::set_hook!(@cmd ({
            $cmd.command($command)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter");
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SetHook::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::set_hook!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::set_hook!(@cmd ({ $crate::SetHook::new() }) $($tail)*,)
    }};
}

#[test]
fn set_hook_macro() {
    use crate::TargetSession;
    use std::borrow::Cow;

    // Structure for setting or unsetting hook `hook-name` to command.
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // set-hook [-agRu] [-t target-session] hook-name command
    // ```
    //
    // tmux ^2.8:
    // ```text
    // set-hook [-gRu] [-t target-session] hook-name command
    // ```
    //
    // tmux ^2.4:
    // ```text
    // set-hook [-gu] [-t target-session] hook-name command
    // ```
    //
    // tmux ^2.2:
    // ```text
    // set-hook [-g] [-t target-session] hook-name command
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let set_hook = set_hook!();
    #[cfg(feature = "tmux_3_0")]
    let set_hook = set_hook!((set_hook), -a);
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook!((set_hook), -g);
    #[cfg(feature = "tmux_2_8")]
    let set_hook = set_hook!((set_hook), -R);
    #[cfg(feature = "tmux_2_4")]
    let set_hook = set_hook!((set_hook), -u);
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook!((set_hook), -t & target_session);
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook!((set_hook), "2");
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook!((set_hook), "3");

    let cmd = "set-hook";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_0")]
    s.push("-a");
    #[cfg(feature = "tmux_2_2")]
    s.push("-g");
    #[cfg(feature = "tmux_2_8")]
    s.push("-R");
    #[cfg(feature = "tmux_2_4")]
    s.push("-u");
    #[cfg(feature = "tmux_2_2")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_2_2")]
    s.push("2");
    #[cfg(feature = "tmux_2_2")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let set_hook = set_hook.build().to_vec();

    assert_eq!(set_hook, s);
}
