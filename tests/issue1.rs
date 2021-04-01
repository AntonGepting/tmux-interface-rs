// Issue #1: How to attach to a session from outside tmux?
// Solution: Command::stdin(Stdio::inherit());
//
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
    use tmux_interface::{TargetSession, TmuxCommand};

    let tmux = TmuxCommand::new();

    let target_session = TargetSession::Raw("test_ti").to_string();

    tmux.new_session()
        .detached()
        .session_name(&target_session)
        .output()
        .unwrap();
    // do not wait for user input, because test is running on Travis CI
    tmux.send_keys().key("exit").key("C-m").output().unwrap();
    tmux.attach_session()
        .target_session(&target_session)
        .output()
        .unwrap();
    //assert!(output.status.success());
    tmux.kill_session()
        .target_session(&target_session)
        .output()
        .unwrap();
}
