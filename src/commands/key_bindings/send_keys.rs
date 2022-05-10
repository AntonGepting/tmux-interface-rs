use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Structure
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^3.0:
/// ```text
/// tmux send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux send-keys [-lMR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux send-keys [-lR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^1.6:
/// ```text
/// tmux send-keys [-R] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux send-keys [-t target-window] key ...
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
    pub fn expand_formats(&mut self) -> &mut Self {
        self.expand_formats = true;
        self
    }

    /// `[-H]` - expect each key to be a hexadecimal number for an ASCII character
    #[cfg(feature = "tmux_3_0")]
    pub fn hex(&mut self) -> &mut Self {
        self.hex = true;
        self
    }

    /// `[-l]` - disable key name lookup and processes the keys as literal UTF-8 characters
    #[cfg(feature = "tmux_1_7")]
    pub fn disable_lookup(&mut self) -> &mut Self {
        self.disable_lookup = true;
        self
    }

    /// `[-M]` - pass through a mouse event
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse_event(&mut self) -> &mut Self {
        self.mouse_event = true;
        self
    }

    /// `[-R]` - cause the terminal state to be reset
    #[cfg(feature = "tmux_1_6")]
    pub fn copy_mode(&mut self) -> &mut Self {
        self.copy_mode = true;
        self
    }

    /// `[-X]` - send a command into copy mode
    #[cfg(feature = "tmux_2_4")]
    pub fn reset(&mut self) -> &mut Self {
        self.reset = true;
        self
    }

    /// `[-N repeat-count]` - specify a repeat count
    #[cfg(feature = "tmux_2_4")]
    pub fn repeat_count(&mut self, repeat_count: usize) -> &mut Self {
        self.repeat_count = Some(repeat_count);
        self
    }

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_1_6")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-t target-window]` - specify the target window
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.target_window = Some(target_pane.into());
        self
    }

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub fn key<S: Into<Cow<'a, str>>>(&mut self, key: S) -> &mut Self {
        self.key = Some(key.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SEND_KEYS);

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
        if let Some(repeat_count) = &self.repeat_count {
            cmd.push_option(N_UPPERCASE_KEY, repeat_count.to_string());
        }

        // `[-t target-pane]` - specify the target pane
        #[cfg(feature = "tmux_1_6")]
        if let Some(target_pane) = &self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane.as_ref());
        }

        // `[-t target-window]` - specify the target window
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
        if let Some(target_window) = &self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window.as_ref());
        }

        // `key`
        #[cfg(feature = "tmux_0_8")]
        if let Some(key) = &self.key {
            cmd.push_param(key.as_ref());
        }

        cmd
    }
}
