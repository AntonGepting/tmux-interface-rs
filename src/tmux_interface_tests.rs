use crate::TmuxInterface;
use crate::Version;


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
    assert_eq!(version.prog_name, "tmux");
    assert!(version.major >= 1);
}
