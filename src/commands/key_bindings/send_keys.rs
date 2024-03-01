use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Send<'a> = SendKeys<'a>;

/// Structure
///
///
/// # Manual
///
/// tmux ^3.4:
/// ```text
/// send-keys [-FHKlMRX] [-c target-client] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^3.1:
/// ```text
/// send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^3.0:
/// ```text
/// send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^2.4:
/// ```text
/// send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^2.1:
/// ```text
/// send-keys [-lMR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^1.7:
/// ```text
/// send-keys [-lR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^1.6:
/// ```text
/// send-keys [-R] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^0.8:
/// ```text
/// send-keys [-t target-window] key ...
/// (alias: send)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SendKeys<'a> {
    /// `[-F]` - expand formats in arguments where appropriate
    #[cfg(feature = "tmux_3_1")]
    pub expand_formats: bool,

    /// `[-H]` - expect each key to be a hexadecimal number for an ASCII character
    #[cfg(feature = "tmux_3_0")]
    pub hex: bool,

    /// `[-K]` - keys are sent to target-client, so they are looked up in the client's key table
    #[cfg(feature = "tmux_3_4")]
    pub client: bool,

    /// `[-l]` - disable key name lookup and processes the keys as literal UTF-8 characters
    #[cfg(feature = "tmux_1_7")]
    pub disable_lookup: bool,

    /// `[-M]` - pass through a mouse event
    #[cfg(feature = "tmux_2_1")]
    pub mouse_event: bool,

    /// `[-R]` - cause the terminal state to be reset
    #[cfg(feature = "tmux_1_6")]
    pub copy_mode: bool,

    /// `[-X]` - send a command into copy mode
    #[cfg(feature = "tmux_2_4")]
    pub reset: bool,

    /// `[-c target-client]` - specify the target client
    #[cfg(feature = "tmux_3_4")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-N repeat-count]` - specify a repeat count
    #[cfg(feature = "tmux_2_4")]
    pub repeat_count: Option<usize>,

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_1_6")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-t target-window]` - specify the target window
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub key: Option<Cow<'a, str>>,
}

// FIXME: repeat-count
impl<'a> SendKeys<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F]` - expand formats in arguments where appropriate
    #[cfg(feature = "tmux_3_1")]
    pub fn expand_formats(mut self) -> Self {
        self.expand_formats = true;
        self
    }

    /// `[-H]` - expect each key to be a hexadecimal number for an ASCII character
    #[cfg(feature = "tmux_3_0")]
    pub fn hex(mut self) -> Self {
        self.hex = true;
        self
    }

    /// `[-K]` - keys are sent to target-client, so they are looked up in the client's key table
    #[cfg(feature = "tmux_3_4")]
    pub fn client(mut self) -> Self {
        self.client = true;
        self
    }

    /// `[-l]` - disable key name lookup and processes the keys as literal UTF-8 characters
    #[cfg(feature = "tmux_1_7")]
    pub fn disable_lookup(mut self) -> Self {
        self.disable_lookup = true;
        self
    }

    /// `[-M]` - pass through a mouse event
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse_event(mut self) -> Self {
        self.mouse_event = true;
        self
    }

    /// `[-R]` - cause the terminal state to be reset
    #[cfg(feature = "tmux_1_6")]
    pub fn copy_mode(mut self) -> Self {
        self.copy_mode = true;
        self
    }

    /// `[-X]` - send a command into copy mode
    #[cfg(feature = "tmux_2_4")]
    pub fn reset(mut self) -> Self {
        self.reset = true;
        self
    }

    /// `[-N repeat-count]` - specify a repeat count
    #[cfg(feature = "tmux_2_4")]
    pub fn repeat_count(mut self, repeat_count: usize) -> Self {
        self.repeat_count = Some(repeat_count);
        self
    }

    /// `[-c target-client]` - specify the target client
    #[cfg(feature = "tmux_3_4")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_1_6")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-t target-window]` - specify the target window
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub fn key<S: Into<Cow<'a, str>>>(mut self, key: S) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SEND_KEYS);

        // `[-F]` - expand formats in arguments where appropriate
        #[cfg(feature = "tmux_3_1")]
        if self.expand_formats {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-H]` - expect each key to be a hexadecimal number for an ASCII character
        #[cfg(feature = "tmux_3_0")]
        if self.hex {
            cmd.push_flag(H_UPPERCASE_KEY);
        }

        // `[-K]` - keys are sent to target-client, so they are looked up in the client's key table
        #[cfg(feature = "tmux_3_4")]
        if self.hex {
            cmd.push_flag(K_UPPERCASE_KEY);
        }

        // `[-l]` - disable key name lookup and processes the keys as literal UTF-8 characters
        #[cfg(feature = "tmux_1_7")]
        if self.disable_lookup {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-M]` - pass through a mouse event
        #[cfg(feature = "tmux_2_1")]
        if self.mouse_event {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-R]` - cause the terminal state to be reset
        #[cfg(feature = "tmux_1_6")]
        if self.copy_mode {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-X]` - send a command into copy mode
        #[cfg(feature = "tmux_2_4")]
        if self.reset {
            cmd.push_flag(X_UPPERCASE_KEY);
        }

        // `[-N repeat-count]` - specify a repeat count
        #[cfg(feature = "tmux_2_4")]
        if let Some(repeat_count) = self.repeat_count {
            cmd.push_option(N_UPPERCASE_KEY, repeat_count.to_string());
        }

        // `[-c target-client]` - specify the target client
        #[cfg(feature = "tmux_3_4")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client);
        }

        // `[-t target-pane]` - specify the target pane
        #[cfg(feature = "tmux_1_6")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-t target-window]` - specify the target window
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `key`
        #[cfg(feature = "tmux_0_8")]
        if let Some(key) = self.key {
            cmd.push_param(key);
        }

        cmd
    }
}
