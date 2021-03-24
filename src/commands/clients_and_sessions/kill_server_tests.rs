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

    assert_eq!(kill_server.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(kill_server.0.bin_args, None);
    assert_eq!(kill_server.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(kill_server.0.cmd_args, None);
}
