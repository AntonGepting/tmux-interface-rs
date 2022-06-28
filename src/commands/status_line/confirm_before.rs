use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// # Manual
///
/// tmux ^1.5:
/// ```text
/// confirm-before [-p prompt] [-t target-client] command
/// (alias: confirm)
/// ```
///
/// tmux ^0.9:
/// ```text
/// confirm-before [-t target-client] command
/// (alias: confirm)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ConfirmBefore<'a> {
    /// `[-p prompt]`
    #[cfg(feature = "tmux_1_5")]
    pub prompt: Option<Cow<'a, str>>,

    /// `[-t target-client]`
    #[cfg(feature = "tmux_0_9")]
    pub target_client: Option<Cow<'a, str>>,

    /// `command`
    #[cfg(feature = "tmux_0_9")]
    pub command: Option<Cow<'a, str>>,
}

impl<'a> ConfirmBefore<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-p prompt]`
    #[cfg(feature = "tmux_1_5")]
    pub fn prompt<S: Into<Cow<'a, str>>>(mut self, prompt: S) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_0_9")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `command`
    #[cfg(feature = "tmux_0_9")]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CONFIRM_BEFORE);

        // `[-p prompt]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(prompt) = self.prompt {
            cmd.push_option(P_LOWERCASE_KEY, prompt);
        }

        // `[-t target-client]`
        #[cfg(feature = "tmux_0_9")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client);
        }

        // `command`
        #[cfg(feature = "tmux_0_9")]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        cmd
    }
}
