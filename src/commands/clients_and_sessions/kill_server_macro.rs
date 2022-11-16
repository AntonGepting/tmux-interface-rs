/// Kill the tmux server and clients and destroy all sessions
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// kill-server
/// ```
#[macro_export]
macro_rules! kill_server {
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter");
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::KillServer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::kill_server!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::has_session!(@cmd ({ $crate::KillServer::new() }) $($tail)*,)
    }};
}

#[test]
fn kill_server_macro() {
    use std::borrow::Cow;

    // Kill the tmux server and clients and destroy all sessions
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // kill-server
    // ```
    let kill_server = kill_server!();

    let cmd = "kill-server";

    let mut s = Vec::new();
    s.push(cmd);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let kill_server = kill_server.build().to_vec();

    assert_eq!(kill_server, s);
}
