#[test]
fn list_clients() {
    use crate::{ListClients, TargetSession};
    use std::borrow::Cow;

    // List all clients attached to the server
    //
    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // tmux list-clients [-F format] [-t target-session]
    // (alias: lsc)
    //
    // ```
    // tmux ^1.5:
    // ```text
    // tmux list-clients [-t target-session]
    // (alias: lsc)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux list-clients
    // (alias: lsc)
    // ```
    let target_session = TargetSession::Raw("2").to_string();

    let mut list_clients = ListClients::new();
    #[cfg(feature = "tmux_1_6")]
    list_clients.format("1");
    #[cfg(feature = "tmux_1_5")]
    list_clients.target_session(&target_session);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-clients";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsc";

    let mut s = Vec::new();
    s.extend_from_slice(&["-F", "1"]);
    s.extend_from_slice(&["-t", "2"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(list_clients.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(list_clients.0.bin_args, None);
    assert_eq!(list_clients.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(list_clients.0.cmd_args, Some(s));
}
