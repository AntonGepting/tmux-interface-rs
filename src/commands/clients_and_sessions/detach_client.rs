use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxCommandTrait};
use std::borrow::Cow;

/// Structure for detaching the current client
///
/// # Manual
///
/// tmux ^2.4:
/// ```text
/// tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
///
/// tmux ^2.2:
/// ```text
/// tmux detach-client [-aP] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux detach-client [-P] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux detach-client [-t target-client]
/// (alias: detach)
/// ```
impl<'a> DetachClient<'a> for TmuxCommand<'a> {}

pub trait DetachClient<'a>: TmuxCommandTrait<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const DETACH_CLIENT: &'static str = "detach-client";
    #[cfg(feature = "use_cmd_alias")]
    const DETACH_CLIENT: &'static str = "detach";

    fn new() -> TmuxCommand<'a> {
        TmuxCommand {
            cmd: Some(<TmuxCommand as DetachClient>::DETACH_CLIENT.into()),
            ..Default::default()
        }
    }

    /// [-a] - kill all but the client client given with `-t`
    #[cfg(feature = "tmux_2_2")]
    fn all(&mut self) -> &mut Self {
        self.push_flag(a_KEY);
        self
    }

    /// [-P] - send SIGHUP to the parent process of the client, typically causing it to exit
    #[cfg(feature = "tmux_1_5")]
    fn parent_sighup(&mut self) -> &mut Self {
        self.push_flag(P_KEY);
        self
    }

    /// [-E shell-command] - run shell-command to replace the client
    #[cfg(feature = "tmux_2_4")]
    fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.push_option(E_KEY, shell_command);
        self
    }

    /// [-s target-session] - specify the session, all clients currently attached
    #[cfg(feature = "tmux_1_5")]
    fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.push_option(s_KEY, target_session);
        self
    }

    /// [-t target-client] - specify the client
    #[cfg(feature = "tmux_0_8")]
    fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.push_option(t_KEY, target_client);
        self
    }

    // run command
    //fn exec(&self) -> TmuxOutput {
    //self.exec()
    //}
}
