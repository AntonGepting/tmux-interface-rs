// command sequence, Tmux struct as parent, commands structs (NewSession, HasSession...) as children
#[test]
fn example1_0() {
    use tmux_interface::{HasSession, KillSession, NewSession, Tmux};

    let target_session = "example_1";

    Tmux::new()
        .command(NewSession::new().detached().session_name(target_session))
        .command(HasSession::new().target_session(target_session))
        .command(KillSession::new().target_session(target_session))
        //.envs()
        .output()
        //.status()
        .unwrap();

    //assert!(status.success());
}

// command sequence, Tmux struct as parent, commands structs (NewSession, HasSession...) as children
#[test]
fn example1() {
    use tmux_interface::{HasSession, KillSession, NewSession, Tmux};

    let target_session = "example_1";

    Tmux::new()
        .command(NewSession::new().detached().session_name(target_session))
        .command(HasSession::new().target_session(target_session))
        .command(KillSession::new().target_session(target_session))
        //.envs()
        .output()
        .unwrap();
}

// command sequence, Tmux struct as parent, TmuxCommand as children
#[test]
fn example1_1() {
    use tmux_interface::{Tmux, TmuxCommand};

    let target_session = "example_1";

    Tmux::new()
        .command(
            TmuxCommand::new_session()
                .detached()
                .session_name(target_session),
        )
        .command(TmuxCommand::has_session().target_session(target_session))
        .command(TmuxCommand::kill_session().target_session(target_session))
        //.envs()
        .output()
        .unwrap();
}

// separate commands
#[test]
fn example2() {
    use tmux_interface::{HasSession, KillSession, NewSession, Tmux};

    let target_session = "example_2";

    Tmux::with_command(NewSession::new().detached().session_name(target_session))
        .output()
        .unwrap();

    Tmux::with_command(HasSession::new().target_session(target_session))
        .output()
        .unwrap();

    Tmux::with_command(KillSession::new().target_session(target_session))
        .output()
        .unwrap();
}

// separate commands
#[test]
fn example2_commands() {
    use tmux_interface::{HasSession, KillSession, NewSession, TmuxCommands};

    let target_session = "example2";

    let mut cmds = TmuxCommands::new();
    cmds.push(NewSession::new().detached().session_name(target_session));
    cmds.push(HasSession::new().target_session(target_session));
    cmds.push(KillSession::new().target_session(target_session));
    //cmds.build().output().unwrap();
}

//#[test]
//fn example3() {
//use tmux_interface::{TargetSession, TmuxInterface};

//let mut tmux = TmuxInterface::new();
//let id = tmux.new_session(None).unwrap();
//tmux.kill_session(None, None, Some(&TargetSession::Id(id)))
//.unwrap();
//}
