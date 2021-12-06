#[test]
fn tmux_bin_command() {
    use crate::commands::tmux_bin_command::TmuxBinCommand;
    use crate::commands::tmux_bin_commands::TmuxBinCommands;
    use crate::commands::tmux_commands::TmuxCommands;
    use crate::{NewSession, SessionOptions, ShowOptions, StartServer, TmuxCommand};

    let cmd = TmuxBinCommand::default();
    let mut new_session = NewSession::new();
    new_session.session_name("asdf");
    new_session.detached();

    //let new_session.to_tmux_bin_command();
}
