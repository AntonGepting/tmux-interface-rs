/// Structure for creating a new session
///
/// # Manual
///
/// tmux 3.3:
/// ```text
/// server-access [-adlrw] [user]
/// ```
#[macro_export]
macro_rules! server_access {
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::server_access!(@cmd ({
            $cmd.attach()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::server_access!(@cmd ({
            $cmd.detached()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::server_access!(@cmd ({
            $cmd.detach_other()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::server_access!(@cmd ({
            $cmd.not_update_env()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::server_access!(@cmd ({
            $cmd.print()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) $user:expr, $($tail:tt)*) => {{
        $crate::server_access!(@cmd ({
            $cmd.shell_command($user)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ServerAccess::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::server_access!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::server_access!(@cmd ({ $crate::ServerAccess::new() }) $($tail)*,)
    }};
}

#[test]
fn server_access_macro() {
    use crate::ServerAccess;
    use std::borrow::Cow;

    // Execute commands from path
    //
    // # Manual
    //
    // tmux ^3.3:
    // ```text
    // server-access [-adlrw] [user]
    // (alias: source)
    // ```
    let server_access = server_access!();
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access!((server_access), -a);
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access!((server_access), -d);
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access!((server_access), -l);
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access!((server_access), -r);
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access!((server_access), -w);
    #[cfg(feature = "tmux_3_3")]
    let source_file = server_access!((server_access), "1");

    let cmd = "server-access";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    s.push("-a");
    #[cfg(feature = "tmux_3_3")]
    s.push("-d");
    #[cfg(feature = "tmux_3_3")]
    s.push("-l");
    #[cfg(feature = "tmux_3_3")]
    s.push("-r");
    #[cfg(feature = "tmux_3_3")]
    s.push("-w");
    #[cfg(feature = "tmux_3_3")]
    s.push("1");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let server_access = server_access.build().to_vec();

    assert_eq!(server_access, s);
}
