use crate::tmux_interface::TmuxInterface;


#[test]
fn version() {
    let tmux = TmuxInterface::new(None);
    //assert_eq!(tmux.version().unwrap(), (2, 8));
}
