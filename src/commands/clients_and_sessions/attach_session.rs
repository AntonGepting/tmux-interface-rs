use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for attaching client to already existing session
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux attach-session [-dErx] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux attach-session [-dr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux attach-session [-dr] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux attach-session [-d] [-t target-session]
/// (alias: attach)
/// ```
#[derive(Clone, Debug)]
pub struct AttachSession<'a>(TmuxCommand<'a>);

impl<'a> AttachSession<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const ATTACH_SESSION: &'static str = "attach-session";
    #[cfg(feature = "use_cmd_alias")]
    const ATTACH_SESSION: &'static str = "attach";

    pub fn new() -> Self {
        AttachSession({
            TmuxCommand {
                cmd: Some(AttachSession::ATTACH_SESSION.into()),
                ..Default::default()
            }
        })
    }

    /// [-d] - any other clients attached to the session are detached
    #[cfg(feature = "tmux_0_8")]
    pub fn detach_other(&mut self) -> &mut Self {
        self.0.push_flag(d_KEY);
        self
    }

    /// [-E] - `update-environment` option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(&mut self) -> &mut Self {
        self.0.push_flag(E_KEY);
        self
    }

    /// [-r] - signifies the client is read-only
    #[cfg(feature = "tmux_1_2")]
    pub fn read_only(&mut self) -> &mut Self {
        self.0.push_flag(r_KEY);
        self
    }

    /// [-x] - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.0.insert_flag(x_KEY);
        self
    }

    /// [-c working-directory] - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    pub fn working_directory<S: Into<Cow<'a, str>>>(&mut self, working_directory: S) -> &mut Self {
        self.0.push_option(c_KEY, working_directory);
        self
    }

    /// [-t target-session] - specify target session name
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(t_KEY, target_session);
        self
    }

    /// run command
    pub fn exec(&self) -> TmuxOutput {
        self.0.exec()
    }
}

//impl<'a> From<Tmux<'a>> for AttachSession<'a> {
//fn from(item: Tmux<'a>) -> Self {
//AttachSession(TmuxCommand {
//bin: item.0.bin,
//cmd: Some(AttachSession::ATTACH_SESSION.into()),
//..Default::default()
//})
//}
//}

//impl<'a> From<&Tmux<'a>> for AttachSession<'a> {
//fn from(item: &Tmux<'a>) -> Self {
//AttachSession(TmuxCommand {
//bin: item.0.bin.clone(),
//cmd: Some(AttachSession::ATTACH_SESSION.into()),
//..Default::default()
//})
//}
//}

//impl<'a> From<&mut Tmux<'a>> for AttachSession<'a> {
//fn from(item: &mut Tmux<'a>) -> Self {
//AttachSession(TmuxCommand {
//bin: item.0.bin.clone(),
//cmd: Some(AttachSession::ATTACH_SESSION.into()),
//..Default::default()
//})
//}
//}
