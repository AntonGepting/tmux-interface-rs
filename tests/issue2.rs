// Window ID / Window Title / Pane number missing

use tmux_interface::{Sessions, TargetSession, Windows};

#[test]
fn issue2() {
    let sessions = Sessions::get().unwrap();

    for s in sessions {
        let session_name = s.name.unwrap().to_string();
        let parent_session = TargetSession::new(&session_name);
        let ws = Windows::get(&parent_session).unwrap();
        println!("Session {}", session_name);
        for w in ws {
            println!("{:?}", w);
        }
    }
}
