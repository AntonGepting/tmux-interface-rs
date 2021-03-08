use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

#[derive(Default, Debug)]
pub struct HasSession<'a>(TmuxCommand<'a>);

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
impl<'a> HasSession<'a> {
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

    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(t_KEY, target_session);
        self
    }

    /// run command
    pub fn exec(&self) -> TmuxOutput {
        self.0.exec()
    }
}

impl<'a> From<HasSession<'a>> for TmuxCommand<'a> {
    fn from(item: HasSession<'a>) -> Self {
        item.0
    }
}

// XXX: ? mb implement too
//impl<'a> From<Tmux<'a>> for HasSession<'a> {
//fn from(item: Tmux<'a>) -> Self {
//let mut command: HasSession = item.into();
//command.cmd = Some(HasSession::HAS_SESSION.into());
//HasSession(command)
//}
//}

//impl<'a> From<&Tmux<'a>> for HasSession<'a> {
//fn from(item: &Tmux<'a>) -> Self {
//let mut command: TmuxCommand = item.into();
//command.cmd = Some(HasSession::HAS_SESSION.into());
//HasSession(command)
//}
//}

//impl<'a> From<TmuxCommand<'a>> for HasSession<'a> {
//fn from(item: TmuxCommand<'a>) -> Self {
//let mut command: TmuxCommand = item.into();
//command.cmd = Some(HasSession::HAS_SESSION.into());
//HasSession(command)
//}
//}

//impl<'a> From<&TmuxCommand<'a>> for HasSession<'a> {
//fn from(item: &TmuxCommand<'a>) -> Self {
//let mut command: TmuxCommand = item.into();
//command.cmd = Some(HasSession::HAS_SESSION.into());
//HasSession(command)
//}
//}
