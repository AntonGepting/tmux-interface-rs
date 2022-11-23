/// # Manual
///
/// tmux ^0.8:
/// ```text
/// lock-server
/// (alias: lock)
/// ```
#[macro_export]
macro_rules! lock_server {
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::LockServer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::lock_server!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::lock_server!(@cmd ({ $crate::LockServer::new() }) $($tail)*,)
    }};
}

#[test]
fn lock_server_macro() {
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // lock-server
    // (alias: lock)
    // ```
    let lock_server = lock_server!();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-server";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lock";

    let mut s = Vec::new();
    s.push(cmd);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let lock_server = lock_server.build().to_vec();

    assert_eq!(lock_server, s);
}
