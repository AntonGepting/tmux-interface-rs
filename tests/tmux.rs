extern crate tmux_interface;
use crate::tmux_interface::TmuxInterface;


#[test]
fn nnnn() {
    let tmux = TmuxInterface::new(None);
    //let mut tmux = Tmux::new(Some("./tests/tmux.sh"));
    //custom.tmux.arg("ls").status().expect("fail");
    tmux.list_sessions(None).unwrap();
}

