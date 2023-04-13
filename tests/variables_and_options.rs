#[cfg(feature = "tmux_1_6")]
#[test]
fn get() {
    use tmux_interface::{
        GlobalSessionOptionsCtl, GlobalWindowOptionsCtl, LocalSessionOptionsCtl,
        LocalWindowOptionsCtl, PaneOptionsCtl, PanesCtl, ServerOptionsCtl, SessionOptionsCtl,
        SessionsCtl, TargetPane, TargetSession, TargetWindow, WindowOptionsCtl, WindowsCtl,
    };

    println!("global: ");
    let global_server_options = ServerOptionsCtl::default().get_all().unwrap();
    println!(" server options: ");
    println!("  user options: {:?}", global_server_options.user_options);
    // dbg!(global_server_options);

    let global_session_options = GlobalSessionOptionsCtl::default().get_all().unwrap();
    println!(" session options: ");
    println!("  user options: {:?}", global_session_options.user_options);
    // dbg!(global_session_options);

    let global_window_options = GlobalWindowOptionsCtl::default().get_all().unwrap();
    println!(" window options: ");
    println!("  user options: {:?}", global_window_options.user_options);
    // dbg!(global_window_options);

    let sessions = SessionsCtl::default().get_all().unwrap();
    for session in sessions {
        let target_session = TargetSession::Id(session.id.unwrap());
        println!("session: {} ({})", target_session, session.name.unwrap());
        // dbg!(&target_session_str);

        let session_options = LocalSessionOptionsCtl::with_target(Some(target_session.to_string()))
            .get_all()
            .unwrap();

        // dbg!(session_options);
        println!(" user options: {:?}", session_options.user_options);

        let windows = WindowsCtl::default()
            .get(Some(target_session.to_string()))
            .unwrap();
        for window in windows {
            let target_window = TargetWindow::Id(window.id.unwrap());
            println!("  window: {} ({})", target_window, window.name.unwrap());
            // dbg!(&target_window_str);

            let window_options =
                LocalWindowOptionsCtl::with_target(Some(target_window.to_string()))
                    .get_all()
                    .unwrap();

            // dbg!(window_options);
            println!("   user options: {:?}", window_options.user_options);

            let panes = PanesCtl::default()
                .get(Some(target_window.to_string()))
                .unwrap();
            for pane in panes {
                let target_pane = TargetPane::Id(pane.id.unwrap());
                // dbg!(&target_pane_str);
                println!("   pane: {}:", target_pane);

                let pane_options = PaneOptionsCtl::with_target(Some(target_pane.to_string()))
                    .get_all()
                    .unwrap();
                // dbg!(pane_options);
                println!("    user options: {:?}", pane_options.user_options);
            }
        }
    }
}
