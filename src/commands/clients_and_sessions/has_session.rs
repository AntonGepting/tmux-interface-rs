use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxOutput};

#[derive(Default, Debug)]
pub struct HasSession(TmuxCommand);

// XXX: better result return?
/// Report if the specified session exist
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux has-session [-t target-session]
/// (alias: has)
/// ```
impl HasSession {
    #[cfg(not(feature = "use_cmd_alias"))]
    const HAS_SESSION: &'static str = "has-session";
    #[cfg(feature = "use_cmd_alias")]
    const HAS_SESSION: &'static str = "has";

    pub fn new() -> Self {
        HasSession({
            TmuxCommand {
                bin: None,
                bin_args: None,
                cmd: Some(HasSession::HAS_SESSION.into()),
                cmd_args: None,
            }
        })
    }

    pub fn target_session<S: Into<String>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(t_KEY, target_session);
        self
    }

    /// run command
    pub fn exec(&self) -> TmuxOutput {
        self.0.exec()
    }
}

impl From<HasSession> for TmuxCommand {
    fn from(item: HasSession) -> Self {
        item.0
    }
}

// XXX: ? mb implement too
//impl From<Tmux> for HasSession {
//fn from(item: Tmux) -> Self {
//let mut command: HasSession = item.into();
//command.cmd = Some(HasSession::HAS_SESSION.into());
//HasSession(command)
//}
//}

//impl From<&Tmux> for HasSession {
//fn from(item: &Tmux) -> Self {
//let mut command: TmuxCommand = item.into();
//command.cmd = Some(HasSession::HAS_SESSION.into());
//HasSession(command)
//}
//}

//impl From<TmuxCommand> for HasSession {
//fn from(item: TmuxCommand) -> Self {
//let mut command: TmuxCommand = item.into();
//command.cmd = Some(HasSession::HAS_SESSION.into());
//HasSession(command)
//}
//}

//impl From<&TmuxCommand> for HasSession {
//fn from(item: &TmuxCommand) -> Self {
//let mut command: TmuxCommand = item.into();
//command.cmd = Some(HasSession::HAS_SESSION.into());
//HasSession(command)
//}
//}
