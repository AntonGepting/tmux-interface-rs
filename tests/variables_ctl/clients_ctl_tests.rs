#[cfg(feature = "tmux_1_6")]
#[test]
fn get_clients() {
    use tmux_interface::{ClientsCtl, KillSession, NewSession, Tmux};

    const TARGET_SESSION: &str = "get_clients_test";

    Tmux::with_command(NewSession::new().detached().session_name(TARGET_SESSION))
        .output()
        .unwrap();

    let _clients = ClientsCtl::new().get_all().unwrap();
    // let mut found = false;
    // for client in clients {
    //     if client.session == Some("0".to_string()) {
    // found = true;
    //     }
    // }
    // assert!(found);

    Tmux::with_command(KillSession::new().target_session(TARGET_SESSION))
        .output()
        .unwrap();
}
