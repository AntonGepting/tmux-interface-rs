use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Confirm<'a> = ConfirmBefore<'a>;

/// # Manual
///
/// tmux ^3.4:
/// ```text
/// confirm-before [-by] [-c confirm-key] [-p prompt] [-t target-client] command
/// (alias: confirm)
/// ```
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
    /// `[-b]`
    #[cfg(feature = "tmux_3_4")]
    pub background: bool,

    /// `[-y]`
    #[cfg(feature = "tmux_3_4")]
    pub change_default: bool,

    /// `[-c confirm-key]`
    #[cfg(feature = "tmux_3_4")]
    pub confirm_key: Option<Cow<'a, str>>,

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

    /// `[-b]`
    #[cfg(feature = "tmux_3_4")]
    pub fn background(mut self) -> Self {
        self.background = true;
        self
    }

    /// `[-y]`
    #[cfg(feature = "tmux_3_4")]
    pub fn change_default(mut self) -> Self {
        self.change_default = true;
        self
    }

    /// `[-c confirm-key]`
    #[cfg(feature = "tmux_3_4")]
    pub fn confirm_key<S: Into<Cow<'a, str>>>(mut self, confirm_key: S) -> Self {
        self.confirm_key = Some(confirm_key.into());
        self
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

        // `[-b]`
        #[cfg(feature = "tmux_3_4")]
        if self.background {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-y]`
        #[cfg(feature = "tmux_3_4")]
        if self.change_default {
            cmd.push_flag(Y_LOWERCASE_KEY);
        }

        // `[-c confirm-key]`
        #[cfg(feature = "tmux_3_4")]
        if let Some(confirm_key) = self.confirm_key {
            cmd.push_option(C_LOWERCASE_KEY, confirm_key);
        }

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
