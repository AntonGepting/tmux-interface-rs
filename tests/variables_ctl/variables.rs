#[cfg(feature = "tmux_1_6")]
#[test]
fn variables_ctl_tests() {
    use tmux_interface::{PanesCtl, SessionsCtl, WindowsCtl};

    let sessions = SessionsCtl::new().get_all().unwrap();
    dbg!(&sessions);

    let windows = WindowsCtl::new().get_all().unwrap();
    dbg!(&windows);

    let panes = PanesCtl::new().get_all().unwrap();
    dbg!(&panes);
}
