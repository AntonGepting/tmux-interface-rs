// unnecessary console error output if session is not found:
// `can't find session: my_session`

// #[test]
// fn issue14_command() {
//     use std::process::{Command, Stdio};

//     // error reproduction
//     let output = Command::new("tmux")
//         .args(["has-session", "-t", "my_session"])
//         .status()
//         .unwrap()
//         .success();

//     // temporary solution
//     let output = Command::new("tmux")
//         .args(["has-session", "-t", "my_session"])
//         .stderr(Stdio::null())
//         .status()
//         .unwrap()
//         .success();

//     dbg!(output);
// }

// #[test]
// fn issue14_temporary_solution() {
//     use std::process::Stdio;
//     use tmux_interface::{HasSession, Tmux};

//     // error reproduction
//     let output = Tmux::with_command(HasSession::new().target_session("my_session"))
//         .status()
//         .unwrap()
//         .success();

//     dbg!(output);

//     // temporary solution
//     let output = Tmux::with_command(HasSession::new().target_session("my_session"))
//         .into_command()
//         .stderr(Stdio::null())
//         .status()
//         .unwrap()
//         .success();

//     dbg!(output);
// }

#[test]
fn issue14() {
    use tmux_interface::{HasSession, StdIO, Tmux};

    let output = Tmux::with_command(HasSession::new().target_session("my_session"))
        .stderr(Some(StdIO::Null))
        .status()
        .unwrap()
        .success();

    dbg!(output);
}
