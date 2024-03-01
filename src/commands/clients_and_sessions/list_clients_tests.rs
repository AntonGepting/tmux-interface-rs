#[test]
fn list_clients() {
    use crate::{ListClients, TargetSession};
    use std::borrow::Cow;

    // List all clients attached to the server
    //
    // # Manual
    //
    // tmux ^3.4:
    // ```text
    // list-clients [-F format] [-f filter] [-t target-session]
    // (alias: lsc)
    // ```
    //
    // tmux ^1.6:
    // ```text
    // list-clients [-F format] [-t target-session]
    // (alias: lsc)
    // ```
    //
    // ```
    // tmux ^1.5:
    // ```text
    // list-clients [-t target-session]
    // (alias: lsc)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // list-clients
    // (alias: lsc)
    // ```
    let target_session = TargetSession::Raw("3").to_string();

    let list_clients = ListClients::new();
    #[cfg(feature = "tmux_1_6")]
    let list_clients = list_clients.format("1");
    #[cfg(feature = "tmux_3_4")]
    let list_clients = list_clients.filter("2");
    #[cfg(feature = "tmux_1_5")]
    let list_clients = list_clients.target_session(&target_session);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-clients";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsc";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_clients = list_clients.build().to_vec();

    assert_eq!(list_clients, s);
}
