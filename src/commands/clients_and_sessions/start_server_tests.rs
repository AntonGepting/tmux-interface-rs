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
    // tmux start-server
    // (alias: start)
    // ```
    let start_server = StartServer::new();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "start-server";
    #[cfg(feature = "cmd_alias")]
    let cmd = "start";

    assert_eq!(start_server.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(start_server.0.bin_args, None);
    assert_eq!(start_server.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(start_server.0.cmd_args, None);
}
