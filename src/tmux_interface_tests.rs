use crate::tmux_interface::TmuxInterface;


#[test]
fn new() {
    let tmux = TmuxInterface::new();
    assert_eq!(tmux.tmux, None);
    let mut tmux = TmuxInterface::new();
    tmux.tmux = Some("tmux_mock");
    assert_eq!(tmux.tmux, Some("tmux_mock"));
}


#[test]
fn subcommand() {
    let tmux = TmuxInterface::new();
    tmux.subcommand("has", &["-t", "session_name"]).unwrap();
}


#[test]
fn version() {
    let tmux = TmuxInterface::new();
    let version = tmux.version().unwrap();
    assert!(version.0 >= 1);
    //assert!(version.1 >= 0);
}
