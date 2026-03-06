// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Display<'a> = DisplayMessage<'a>;

/// Display a message
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// display-message [-aCIlNpv] [-c target-client] [-d delay] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=3.4:
/// ```text
/// display-message [-aIlNpv] [-c target-client] [-d delay] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=3.2:
/// ```text
/// display-message [-aINpv] [-c target-client] [-d delay] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=3.0:
/// ```text
/// display-message [-aIpv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=2.9:
/// ```text
/// display-message [-apv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=1.5:
/// ```text
/// display-message [-p] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DisplayMessage<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_2_9")]
    pub list_format_vars: bool,

    /// `[-C]`
    #[cfg(feature = "tmux_3_6")]
    pub keep_updated: bool,

    /// `[-I]`
    #[cfg(feature = "tmux_3_0")]
    pub forward_stdin: bool,

    /// `[-l]`
    #[cfg(feature = "tmux_3_4")]
    pub disable_format: bool,

    /// `[-N]`
    #[cfg(feature = "tmux_3_2")]
    pub ignore_keys: bool,

    /// `[-p]`
    #[cfg(feature = "tmux_1_5")]
    pub print: bool,

    /// `[-v]`
    #[cfg(feature = "tmux_2_9")]
    pub verbose: bool,

    /// `[-c target-client]`
    #[cfg(feature = "tmux_1_5")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-d delay]`
    #[cfg(feature = "tmux_3_2")]
    pub delay: Option<usize>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[message]`
    #[cfg(feature = "tmux_1_5")]
    pub message: Option<Cow<'a, str>>,
}

impl<'a> DisplayMessage<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_2_9")]
    pub fn list_format_vars(mut self) -> Self {
        self.list_format_vars = true;
        self
    }

    /// `[-C]`
    #[cfg(feature = "tmux_3_6")]
    pub fn keep_updated(mut self) -> Self {
        self.keep_updated = true;
        self
    }

    /// `[-I]`
    #[cfg(feature = "tmux_3_0")]
    pub fn forward_stdin(mut self) -> Self {
        self.forward_stdin = true;
        self
    }

    /// `[-l]`
    #[cfg(feature = "tmux_3_4")]
    pub fn disable_format(mut self) -> Self {
        self.disable_format = true;
        self
    }

    /// `[-N]`
    #[cfg(feature = "tmux_3_2")]
    pub fn ignore_keys(mut self) -> Self {
        self.ignore_keys = true;
        self
    }

    /// `[-p]`
    #[cfg(feature = "tmux_1_5")]
    pub fn print(mut self) -> Self {
        self.print = true;
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_2_9")]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// `[-c target-client]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-d delay]`
    #[cfg(feature = "tmux_3_2")]
    pub fn delay(mut self, delay: usize) -> Self {
        self.delay = Some(delay);
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[message]`
    #[cfg(feature = "tmux_1_5")]
    pub fn message<S: Into<Cow<'a, str>>>(mut self, message: S) -> Self {
        self.message = Some(message.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(DISPLAY_MESSAGE);

        // `[-a]`
        #[cfg(feature = "tmux_2_9")]
        if self.list_format_vars {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-C]`
        #[cfg(feature = "tmux_3_6")]
        if self.keep_updated {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-I]`
        #[cfg(feature = "tmux_3_0")]
        if self.forward_stdin {
            cmd.push_flag(I_UPPERCASE_KEY);
        }

        // `[-l]`
        #[cfg(feature = "tmux_3_4")]
        if self.disable_format {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-N]`
        #[cfg(feature = "tmux_3_2")]
        if self.ignore_keys {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-p]`
        #[cfg(feature = "tmux_1_5")]
        if self.print {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-v]`
        #[cfg(feature = "tmux_2_9")]
        if self.verbose {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-c target-client]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client);
        }

        // `[-d delay]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(delay) = self.delay {
            cmd.push_option(D_LOWERCASE_KEY, delay.to_string());
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[message]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(message) = self.message {
            cmd.push_param(message);
        }

        cmd
    }
}
