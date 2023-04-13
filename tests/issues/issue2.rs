// Window ID / Window Title / Pane number missing

use tmux_interface::{SessionsCtl, TargetSession, WindowsCtl};

#[test]
fn issue2() {
    let sessions = SessionsCtl::new().get_all().unwrap();
    for s in sessions {
        let session_name = s.name.unwrap().to_string();
        let parent_session = TargetSession::new(&session_name);
        let windows = WindowsCtl::new()
            .get(Some(&parent_session.to_string()))
            .unwrap();
        println!("Session {}", session_name);
        for window in windows {
            println!("{:?}", window);
        }
    }
}
