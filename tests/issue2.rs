use tmux_interface;
use tmux_interface::{TargetSession, SESSION_ALL, WINDOW_ALL};

#[test]
fn issue2() {
    let sessions = tmux_interface::Sessions::get(SESSION_ALL).unwrap();

    for s in sessions {
        let session_name = s.name.unwrap().to_string();
        let parent_session = TargetSession::new(&session_name);
        let ws = tmux_interface::Windows::get(&parent_session, WINDOW_ALL).unwrap();
        println!("Session {}", session_name);
        for w in ws {
            println!("{:?}", w);
        }
    }
}
