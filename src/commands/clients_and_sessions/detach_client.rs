use crate::commands::constants::*;
use crate::TmuxCommand;
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DetachClient<'a> {
    /// `[-a]` - kill all but the client client given with `-t`
    #[cfg(feature = "tmux_2_2")]
    pub all: bool,

    /// `[-P]` - send SIGHUP to the parent process of the client, typically causing it to exit
    #[cfg(feature = "tmux_1_5")]
    pub parent_sighup: bool,

    /// `[-E shell-command]` - run shell-command to replace the client
    #[cfg(feature = "tmux_2_4")]
    pub shell_command: Option<Cow<'a, str>>,

    /// `[-s target-session]` - specify the session, all clients currently attached
    #[cfg(feature = "tmux_1_5")]
    pub target_session: Option<Cow<'a, str>>,

    /// `[-t target-client]` - specify the client
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<Cow<'a, str>>,
}

impl<'a> DetachClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - kill all but the client client given with `-t`
    #[cfg(feature = "tmux_2_2")]
    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }

    /// `[-P]` - send SIGHUP to the parent process of the client, typically causing it to exit
    #[cfg(feature = "tmux_1_5")]
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.parent_sighup = true;
        self
    }

    /// `[-E shell-command]` - run shell-command to replace the client
    #[cfg(feature = "tmux_2_4")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    /// `[-s target-session]` - specify the session, all clients currently attached
    #[cfg(feature = "tmux_1_5")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[-t target-client]` - specify the client
    #[cfg(feature = "tmux_0_8")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.target_client = Some(target_client.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(DETACH_CLIENT);

        // `[-a]` - kill all but the client client given with `-t`
        #[cfg(feature = "tmux_2_2")]
        if self.all {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-P]` - send SIGHUP to the parent process of the client, typically causing it to exit
        #[cfg(feature = "tmux_1_5")]
        if self.parent_sighup {
            cmd.push_flag(P_UPPERCASE_KEY);
        }

        // `[-E shell-command]` - run shell-command to replace the client
        #[cfg(feature = "tmux_2_4")]
        if let Some(shell_command) = &self.shell_command {
            cmd.push_option(E_UPPERCASE_KEY, shell_command.as_ref());
        }

        // `[-s target-session]` - specify the session, all clients currently attached
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_session) = &self.target_session {
            cmd.push_option(S_LOWERCASE_KEY, target_session.as_ref());
        }

        // `[-t target-client]` - specify the client
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_client) = &self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client.as_ref());
        }

        cmd
    }
}
