use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Lock all clients attached to `target-session`
/// # Manual
///
/// tmux ^1.1:
/// ```text
/// tmux lock-session [-t target-session]
/// (alias: locks)
/// ```
#[derive(Default, Debug)]
pub struct LockSession<'a>(TmuxCommand<'a>);

impl<'a> LockSession<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const LOCK_SESSION: &'static str = "lock-session";
    #[cfg(feature = "use_cmd_alias")]
    const LOCK_SESSION: &'static str = "locks";

    pub fn new() -> Self {
        LockSession({
            TmuxCommand {
                bin: None,
                bin_args: None,
                cmd: Some(LockSession::LOCK_SESSION.into()),
                cmd_args: None,
            }
        })
    }

    /// [-t target-session]
    pub fn target_session<T: Into<Cow<'a, str>>>(&mut self, target_session: T) -> &mut Self {
        self.0.push_option(t_KEY, target_session);
        self
    }

    /// run command
    pub fn exec(&self) -> TmuxOutput {
        self.0.exec()
    }
}

//impl<'a> From<LockSession<'a>> for TmuxCommand<'a> {
//fn from(item: LockSession<'a>) -> Self {
//item.0
//}
//}

//impl<'a> From<Tmux<'a>> for LockSession<'a> {
//fn from(item: Tmux<'a>) -> Self {
//let mut command: TmuxCommand = item.into();
//command.cmd = Some(LockSession::LOCK_SESSION.into());
//LockSession(command)
//}
//}

//impl<'a> From<TmuxCommand<'a>> for LockSession<'a> {
//fn from(item: TmuxCommand<'a>) -> Self {
//let mut command: TmuxCommand = item.into();
//command.cmd = Some(LockSession::LOCK_SESSION.into());
//LockSession(command)
//}
//}
