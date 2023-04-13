#[cfg(feature = "tmux_1_6")]
#[test]
fn get() {
    use tmux_interface::{PanesCtl, SessionsCtl, WindowsCtl};

    let sessions = SessionsCtl::new().get_all().unwrap();
    dbg!(&sessions);

    let windows = WindowsCtl::new().get_all().unwrap();
    dbg!(&windows);

    let panes = PanesCtl::new().get_all().unwrap();
    dbg!(&panes);
}
