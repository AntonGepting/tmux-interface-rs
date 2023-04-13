#[cfg(feature = "tmux_1_6")]
#[test]
fn direct() {
    use tmux_interface::commands::tmux_commands::TmuxCommands;
    use tmux_interface::{HasSession, KillSession, NewSession};

    let session_name = "session";

    let cmds = TmuxCommands::new()
        .cmd(
            NewSession::new()
                .detached()
                .session_name(session_name)
                .build(),
        )
        .cmd(HasSession::new().target_session(session_name).build())
        .cmd(KillSession::new().target_session(session_name).build())
        .to_string();

    dbg!(cmds);
}
