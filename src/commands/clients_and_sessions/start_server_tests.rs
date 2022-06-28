#[test]
fn start_server() {
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
    let start_server = StartServer::new();

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
