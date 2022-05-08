use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Structure for displaying a message
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux display-message [-aINpv] [-c target-client] [-d delay] [-t target-pane] [message]
///  (alias: display)
/// ```
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
#[derive(Debug, Default, Clone)]
pub struct DisplayMessage<'a> {
    /// `[-a]` - list the format variables and their values
    #[cfg(feature = "tmux_2_9a")]
    pub list_format_vars: bool,

    /// `[-I]` - forward any input read from stdin to the empty pane given by target-pane
    #[cfg(feature = "tmux_3_0")]
    pub forward_stdin: bool,

    /// `[-N]` - ignores key presses and closes only after the delay expires
    #[cfg(feature = "tmux_3_2")]
    pub ignore_keys: bool,

    /// `[-p]` - the output is printed to stdout
    #[cfg(feature = "tmux_2_9a")]
    pub print: bool,

    /// `[-v]` - print verbose logging as the format is parsed
    #[cfg(feature = "tmux_2_9a")]
    pub verbose: bool,

    /// `[-c target-client]` - target-client
    #[cfg(feature = "tmux_1_0")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-d delay]` - delay
    #[cfg(feature = "tmux_3_2")]
    pub delay: Option<usize>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[message]` - message
    #[cfg(feature = "tmux_1_0")]
    pub message: Option<Cow<'a, str>>,
}

impl<'a> DisplayMessage<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - list the format variables and their values
    #[cfg(feature = "tmux_2_9a")]
    pub fn list_format_vars(&mut self) -> &mut Self {
        self.list_format_vars = true;
        self
    }

    /// `[-I]` - forward any input read from stdin to the empty pane given by target-pane
    #[cfg(feature = "tmux_3_0")]
    pub fn forward_stdin(&mut self) -> &mut Self {
        self.forward_stdin = true;
        self
    }

    /// `[-N]` - ignores key presses and closes only after the delay expires
    #[cfg(feature = "tmux_3_2")]
    pub fn ignore_keys(&mut self) -> &mut Self {
        self.ignore_keys = true;
        self
    }

    /// `[-p]` - the output is printed to stdout
    #[cfg(feature = "tmux_2_9a")]
    pub fn print(&mut self) -> &mut Self {
        self.print = true;
        self
    }

    /// `[-v]` - print verbose logging as the format is parsed
    #[cfg(feature = "tmux_2_9a")]
    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = true;
        self
    }

    /// `[-c target-client]` - target-client
    #[cfg(feature = "tmux_1_0")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-d delay]` - delay
    #[cfg(feature = "tmux_3_2")]
    pub fn delay(&mut self, delay: usize) -> &mut Self {
        self.delay = Some(delay);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[message]` - message
    #[cfg(feature = "tmux_1_0")]
    pub fn message<S: Into<Cow<'a, str>>>(&mut self, message: S) -> &mut Self {
        self.message = Some(message.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(DISPLAY_MESSAGE);

        // `[-a]` - list the format variables and their values
        #[cfg(feature = "tmux_2_9a")]
        if self.list_format_vars {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-I]` - forward any input read from stdin to the empty pane given by target-pane
        #[cfg(feature = "tmux_3_0")]
        if self.forward_stdin {
            cmd.push_flag(I_UPPERCASE_KEY);
        }

        // `[-N]` - ignores key presses and closes only after the delay expires
        #[cfg(feature = "tmux_3_2")]
        if self.ignore_keys {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-p]` - the output is printed to stdout
        #[cfg(feature = "tmux_2_9a")]
        if self.print {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-v]` - print verbose logging as the format is parsed
        #[cfg(feature = "tmux_2_9a")]
        if self.verbose {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-c target-client]` - target-client
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_client) = &self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client.as_ref());
        }

        // `[-d delay]` - delay
        #[cfg(feature = "tmux_3_2")]
        if let Some(delay) = self.delay {
            cmd.push_option(D_LOWERCASE_KEY, delay.to_string());
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_pane) = &self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane.as_ref());
        }

        // `[message]` - message
        #[cfg(feature = "tmux_1_0")]
        if let Some(message) = &self.message {
            cmd.push_param(message.as_ref());
        }

        cmd
    }
}
