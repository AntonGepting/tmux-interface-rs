#[cfg(feature = "tmux_1_6")]
#[test]
fn get() {
    use tmux_interface::{
        GlobalSessionOptionsCtl, GlobalWindowOptionsCtl, LocalSessionOptionsCtl,
        LocalWindowOptionsCtl, ServerOptionsCtl, SessionOptionsCtl, WindowOptionsCtl,
    };

    #[cfg(feature = "tmux_3_1")]
    use tmux_interface::PaneOptionsCtl;

    let server_options = ServerOptionsCtl::default().get_all().unwrap();
    dbg!(server_options);
    let global_session_options = GlobalSessionOptionsCtl::default().get_all().unwrap();
    dbg!(global_session_options);
    let local_session_options = LocalSessionOptionsCtl::default().get_all().unwrap();
    dbg!(local_session_options);
    let global_window_options = GlobalWindowOptionsCtl::default().get_all().unwrap();
    dbg!(global_window_options);
    let local_window_options = LocalWindowOptionsCtl::default().get_all().unwrap();
    dbg!(local_window_options);
    #[cfg(feature = "tmux_3_1")]
    {
        let pane_options = PaneOptionsCtl::default().get_all().unwrap();
        dbg!(pane_options);
    }
}
