// auto-generated file
//

// Kill the tmux server and clients and destroy all sessions
//
// # Manual
//
// tmux >=0.8:
// ```text
// kill-server
// ```
#[test]
fn kill_server() {
    use crate::KillServer;
    use std::borrow::Cow;

    let kill_server = KillServer::new();

    let cmd = "kill-server";

    let mut v = Vec::new();
    v.push(cmd);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let kill_server = kill_server.build().to_vec();

    assert_eq!(kill_server, v);
}
