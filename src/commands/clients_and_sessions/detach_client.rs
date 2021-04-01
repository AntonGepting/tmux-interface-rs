use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
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
#[derive(Clone, Debug)]
pub struct DetachClient<'a>(pub TmuxCommand<'a>);

impl<'a> Default for DetachClient<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(DETACH_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> DetachClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - kill all but the client client given with `-t`
    #[cfg(feature = "tmux_2_2")]
    pub fn all(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-P]` - send SIGHUP to the parent process of the client, typically causing it to exit
    #[cfg(feature = "tmux_1_5")]
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.0.push_flag(P_UPPERCASE_KEY);
        self
    }

    /// `[-E shell-command]` - run shell-command to replace the client
    #[cfg(feature = "tmux_2_4")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.0.push_option(E_UPPERCASE_KEY, shell_command);
        self
    }

    /// `[-s target-session]` - specify the session, all clients currently attached
    #[cfg(feature = "tmux_1_5")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(S_LOWERCASE_KEY, target_session);
        self
    }

    /// `[-t target-client]` - specify the client
    #[cfg(feature = "tmux_0_8")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_client);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for DetachClient<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(DETACH_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for DetachClient<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(DETACH_CLIENT)),
            ..Default::default()
        })
    }
}
