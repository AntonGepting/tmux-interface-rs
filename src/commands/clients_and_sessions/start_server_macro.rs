/// Start the tmux server, if not already running, without creating any sessions
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// start-server
/// (alias: start)
/// ```
#[macro_export]
macro_rules! start_server {
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::StartServer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::start_server!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::start_server!(@cmd ({ $crate::StartServer::new() }) $($tail)*,)
    }};
}

#[test]
fn start_server_macro() {
    use crate::StartServer;
    use std::borrow::Cow;

    // Start the tmux server, if not already running, without creating any sessions
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // start-server
    // (alias: start)
    // ```
    let start_server = start_server!();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "start-server";
    #[cfg(feature = "cmd_alias")]
    let cmd = "start";

    let mut s = Vec::new();
    s.push(cmd);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let start_server = start_server.build().to_vec();

    assert_eq!(start_server, s);
}
