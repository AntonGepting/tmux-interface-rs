use crate::tmux_interface::TmuxInterface;


#[test]
fn new() {
    let tmux = TmuxInterface::new(None);
    assert_eq!(tmux.tmux, "tmux");
    let tmux = TmuxInterface::new(Some("tmux_mock"));
    assert_eq!(tmux.tmux, "tmux_mock");
}


#[test]
fn subcommand() {
    let tmux = TmuxInterface::new(None);
    tmux.subcommand("has", &["-t", "session_name"]).unwrap();
}


#[test]
fn version() {
    let tmux = TmuxInterface::new(None);
    let version = tmux.version().unwrap();
    assert!(version.0 > 1);
    assert!(version.1 > 0);
}
