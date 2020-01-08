//use std::process::{Command, Stdio};

//#[test]
//fn issue1_spawn() {
//let mut tmux = Command::new("tmux");
//let _output = tmux
//.args(&["new-session", "-s", "test_spawn", "-d"])
//.output()
//.unwrap();

//let mut tmux = Command::new("tmux");
//tmux.stdin(Stdio::inherit());
//tmux.stdout(Stdio::piped());
//let child = tmux
//.args(&["attach-session", "-t", "test_spawn"])
//.spawn()
//.unwrap();
//let output = child.wait_with_output().unwrap();
//println!("{:?}", output);
//assert!(output.status.success());

//let mut tmux = Command::new("tmux");
//let _output = tmux
//.args(&["kill-session", "-t", "test_spawn"])
//.output()
//.unwrap();
//}

//#[test]
//fn issue1_output() {
//let mut tmux = Command::new("tmux");
//let _output = tmux
//.args(&["new-session", "-s", "test_output", "-d"])
//.output()
//.unwrap();

//let mut tmux = Command::new("tmux");
//tmux.stdin(Stdio::inherit());
//tmux.stdout(Stdio::piped());
//let output = tmux
//.args(&["attach-session", "-t", "test_output"])
//.output()
//.unwrap();
//println!("{:?}", output);
//assert!(output.status.success());

//let mut tmux = Command::new("tmux");
//let _output = tmux
//.args(&["kill-session", "-t", "test_output"])
//.output()
//.unwrap();
//}

#[test]
fn issue1() {
    use tmux_interface::{AttachSession, NewSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();

    let new_session = NewSession {
        session_name: Some("test_ti"),
        detached: Some(true),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();

    // do not wait for user input, because test is running on Travis CI
    tmux.send_keys(None, &vec!["exit", "C-m"]).unwrap();

    let attach = AttachSession {
        target_session: Some("test_ti"),
        ..Default::default()
    };
    let output = tmux.attach_session(Some(&attach)).unwrap();
    assert!(output.status.success());

    tmux.kill_session(None, None, Some("test_ti")).unwrap();
}
