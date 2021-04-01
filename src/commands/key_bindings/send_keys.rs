use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
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

#[derive(Debug, Clone)]
pub struct SendKeys<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SendKeys<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SEND_KEYS)),
            ..Default::default()
        })
    }
}

// FIXME: repeat-count
impl<'a> SendKeys<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F]` - expand formats in arguments where appropriate
    #[cfg(feature = "tmux_3_1")]
    pub fn expand_formats(&mut self) -> &mut Self {
        self.0.push_flag(F_UPPERCASE_KEY);
        self
    }

    /// `[-H]` - expect each key to be a hexadecimal number for an ASCII character
    #[cfg(feature = "tmux_3_0")]
    pub fn hex(&mut self) -> &mut Self {
        self.0.push_flag(H_UPPERCASE_KEY);
        self
    }

    /// `[-l]` - disable key name lookup and processes the keys as literal UTF-8 characters
    #[cfg(feature = "tmux_1_7")]
    pub fn disable_lookup(&mut self) -> &mut Self {
        self.0.push_flag(L_LOWERCASE_KEY);
        self
    }

    /// `[-M]` - pass through a mouse event
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse_event(&mut self) -> &mut Self {
        self.0.push_flag(M_UPPERCASE_KEY);
        self
    }

    /// `[-R]` - cause the terminal state to be reset
    #[cfg(feature = "tmux_1_6")]
    pub fn copy_mode(&mut self) -> &mut Self {
        self.0.push_flag(R_UPPERCASE_KEY);
        self
    }

    /// `[-X]` - send a command into copy mode
    #[cfg(feature = "tmux_2_4")]
    pub fn reset(&mut self) -> &mut Self {
        self.0.push_flag(X_UPPERCASE_KEY);
        self
    }

    /// `[-N repeat-count]` - specify a repeat count
    #[cfg(feature = "tmux_2_4")]
    pub fn repeat_count(&mut self, repeat_count: usize) -> &mut Self {
        self.0
            .push_option(N_UPPERCASE_KEY, repeat_count.to_string());
        self
    }

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_1_6")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-t target-window]` - specify the target window
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub fn key<S: Into<Cow<'a, str>>>(&mut self, key: S) -> &mut Self {
        self.0.push_param(key);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SendKeys<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SEND_KEYS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SendKeys<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SEND_KEYS)),
            ..Default::default()
        })
    }
}
