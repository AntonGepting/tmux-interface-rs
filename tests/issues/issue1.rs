// Issue #1: How to attach to a session from outside tmux?
// Solution: Command::stdin(Stdio::inherit());
//
#[test]
fn issue1_spawn_command() {
    use std::process::{Command, Stdio};

    let mut tmux = Command::new("tmux");
    tmux.args(&["new-session", "-d", "-s", "issue1_spawn_command"])
        .output()
        .unwrap();

    let mut tmux = Command::new("tmux");
    tmux.stdin(Stdio::inherit());
    tmux.stdout(Stdio::piped());
    let child = tmux
        .args(&[
            "attach-session",
            "-t",
            "issue1_spawn_command",
            ";",
            "detach-client",
            "-s",
            "issue1_spawn_command",
        ])
        .spawn()
        .unwrap();
    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());

    let mut tmux = Command::new("tmux");
    tmux.args(&["kill-session", "-t", "issue1_spawn_command"])
        .output()
        .unwrap();
}

#[test]
fn issue1_output_command() {
    use std::process::{Command, Stdio};

    let mut tmux = Command::new("tmux");
    tmux.args(&["new-session", "-d", "-s", "issue1_output_command"])
        .output()
        .unwrap();

    let mut tmux = Command::new("tmux");
    tmux.stdin(Stdio::inherit());
    tmux.stdout(Stdio::piped());
    let output = tmux
        .args(&[
            "attach-session",
            "-t",
            "issue1_output_command",
            ";",
            "detach-client",
            "-s",
            "issue1_output_command",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());

    let mut tmux = Command::new("tmux");
    tmux.args(&["kill-session", "-t", "issue1_output_command"])
        .output()
        .unwrap();
}

#[test]
fn issue1() {
    use tmux_interface::{AttachSession, DetachClient, KillSession, NewSession, Tmux};

    let target_session = "issue1";

    Tmux::new()
        .command(NewSession::new().detached().session_name(target_session))
        .output()
        .unwrap();

    let cmd = Tmux::new()
        .command(AttachSession::new().target_session(target_session))
        // do not wait for user input, because test is running on CI server
        .command(DetachClient::new().target_session(target_session));
    let child = cmd.spawn().unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());

    Tmux::new()
        .command(KillSession::new().target_session(target_session))
        .output()
        .unwrap();
}
