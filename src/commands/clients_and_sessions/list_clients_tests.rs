// auto-generated file
//

// List all clients attached to the server
//
// # Manual
//
// tmux >=3.4:
// ```text
// list-clients [-F format] [-f filter] [-t target-session]
// (alias: lsc)
// ```
//
// tmux >=1.6:
// ```text
// list-clients [-F format] [-t target-session]
// (alias: lsc)
// ```
//
// tmux >=1.5:
// ```text
// list-clients [-t target-session]
// (alias: lsc)
// ```
#[test]
fn list_clients() {
    use crate::ListClients;
    use std::borrow::Cow;

    let list_clients = ListClients::new();
    // `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    let list_clients = list_clients.format("1");

    // `[-f filter]`
    #[cfg(feature = "tmux_3_4")]
    let list_clients = list_clients.filter("2");

    // `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    let list_clients = list_clients.target_session("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-clients";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsc";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_6")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_3_4")]
    v.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let list_clients = list_clients.build().to_vec();

    assert_eq!(list_clients, v);
}
