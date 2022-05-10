#[test]
fn kill_server() {
    use crate::KillServer;
    use std::borrow::Cow;

    // Kill the tmux server and clients and destroy all sessions
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // tmux kill-server
    // ```
    let kill_server = KillServer::new();

    let cmd = "kill-server";

    let mut s = Vec::new();
    s.push(cmd);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let kill_server = kill_server.build().to_vec();

    assert_eq!(kill_server, s);
}
