use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for displaying a message
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux ^2.9a:
/// ```text
/// tmux display-message [-apv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux display-message [-p] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux display-message [-p] [-t target-client] [message]
///  (alias: display)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux display-message [-t target-client] [message]
///  (alias: display)
/// ```
#[derive(Debug, Clone)]
pub struct DisplayMessage<'a>(pub TmuxCommand<'a>);

impl<'a> Default for DisplayMessage<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(DISPLAY_MESSAGE)),
            ..Default::default()
        })
    }
}

impl<'a> DisplayMessage<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - list the format variables and their values
    #[cfg(feature = "tmux_2_9a")]
    pub fn list_format_vars(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-I]` - forward any input read from stdin to the empty pane given by target-pane
    #[cfg(feature = "tmux_3_0")]
    pub fn forward_stdin(&mut self) -> &mut Self {
        self.0.push_flag(I_UPPERCASE_KEY);
        self
    }

    /// `[-p]` - the output is printed to stdout
    #[cfg(feature = "tmux_2_9a")]
    pub fn print(&mut self) -> &mut Self {
        self.0.push_flag(P_LOWERCASE_KEY);
        self
    }

    /// `[-v]` - print verbose logging as the format is parsed
    #[cfg(feature = "tmux_2_9a")]
    pub fn verbose(&mut self) -> &mut Self {
        self.0.push_flag(V_LOWERCASE_KEY);
        self
    }

    /// `[-c target-client]` - target-client
    #[cfg(feature = "tmux_1_0")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(C_LOWERCASE_KEY, target_client);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[message]` - message
    #[cfg(feature = "tmux_1_0")]
    pub fn message<S: Into<Cow<'a, str>>>(&mut self, message: S) -> &mut Self {
        self.0.push_param(message);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for DisplayMessage<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(DISPLAY_MESSAGE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for DisplayMessage<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(DISPLAY_MESSAGE)),
            ..Default::default()
        })
    }
}
