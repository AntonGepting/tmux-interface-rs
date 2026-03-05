// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Send<'a> = SendKeys<'a>;

/// Send a key or keys to a window
///
/// # Manual
///
/// tmux >=3.4:
/// ```text
/// send-keys [-FHKlMRX] [-c target-client] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=3.1:
/// ```text
/// send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=3.0a:
/// ```text
/// send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=2.4:
/// ```text
/// send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=2.1:
/// ```text
/// send-keys [-lMR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=1.7:
/// ```text
/// send-keys [-lR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=1.6:
/// ```text
/// send-keys [-R] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=0.8:
/// ```text
/// send-keys [-t target-window] key ...
/// (alias: send)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SendKeys<'a> {
    /// `[-F]`
    #[cfg(feature = "tmux_3_1")]
    pub expand_formats: bool,

    /// `[-H]`
    #[cfg(feature = "tmux_3_0a")]
    pub hex: bool,

    /// `[-K]`
    #[cfg(feature = "tmux_3_4")]
    pub client: bool,

    /// `[-l]`
    #[cfg(feature = "tmux_1_7")]
    pub disable_lookup: bool,

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub mouse_event: bool,

    /// `[-R]`
    #[cfg(feature = "tmux_1_7")]
    pub copy_mode: bool,

    /// `[-X]`
    #[cfg(feature = "tmux_2_4")]
    pub reset: bool,

    /// `[-N repeat-count]`
    #[cfg(feature = "tmux_2_4")]
    pub repeat_count: Option<usize>,

    /// `[-c target-client]`
    #[cfg(feature = "tmux_3_4")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_2_4")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[key]`
    #[cfg(feature = "tmux_0_8")]
    pub keys: Vec<Cow<'a, str>>,
}

impl<'a> SendKeys<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F]`
    #[cfg(feature = "tmux_3_1")]
    pub fn expand_formats(mut self) -> Self {
        self.expand_formats = true;
        self
    }

    /// `[-H]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn hex(mut self) -> Self {
        self.hex = true;
        self
    }

    /// `[-K]`
    #[cfg(feature = "tmux_3_4")]
    pub fn client(mut self) -> Self {
        self.client = true;
        self
    }

    /// `[-l]`
    #[cfg(feature = "tmux_1_7")]
    pub fn disable_lookup(mut self) -> Self {
        self.disable_lookup = true;
        self
    }

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse_event(mut self) -> Self {
        self.mouse_event = true;
        self
    }

    /// `[-R]`
    #[cfg(feature = "tmux_1_7")]
    pub fn copy_mode(mut self) -> Self {
        self.copy_mode = true;
        self
    }

    /// `[-X]`
    #[cfg(feature = "tmux_2_4")]
    pub fn reset(mut self) -> Self {
        self.reset = true;
        self
    }

    /// `[-N repeat-count]`
    #[cfg(feature = "tmux_2_4")]
    pub fn repeat_count(mut self, repeat_count: usize) -> Self {
        self.repeat_count = Some(repeat_count);
        self
    }

    /// `[-c target-client]`
    #[cfg(feature = "tmux_3_4")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_2_4")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[key]`
    #[cfg(feature = "tmux_0_8")]
    pub fn key<S: Into<Cow<'a, str>>>(mut self, key: S) -> Self {
        self.keys.push(key.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SEND_KEYS);

        // `[-F]`
        #[cfg(feature = "tmux_3_1")]
        if self.expand_formats {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-H]`
        #[cfg(feature = "tmux_3_0a")]
        if self.hex {
            cmd.push_flag(H_UPPERCASE_KEY);
        }

        // `[-K]`
        #[cfg(feature = "tmux_3_4")]
        if self.client {
            cmd.push_flag(K_UPPERCASE_KEY);
        }

        // `[-l]`
        #[cfg(feature = "tmux_1_7")]
        if self.disable_lookup {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-M]`
        #[cfg(feature = "tmux_2_1")]
        if self.mouse_event {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-R]`
        #[cfg(feature = "tmux_1_7")]
        if self.copy_mode {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-X]`
        #[cfg(feature = "tmux_2_4")]
        if self.reset {
            cmd.push_flag(X_UPPERCASE_KEY);
        }

        // `[-N repeat-count]`
        #[cfg(feature = "tmux_2_4")]
        if let Some(repeat_count) = self.repeat_count {
            cmd.push_option(N_UPPERCASE_KEY, repeat_count.to_string());
        }

        // `[-c target-client]`
        #[cfg(feature = "tmux_3_4")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_2_4")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[key]`
        #[cfg(feature = "tmux_0_8")]
        for key in self.keys {
            cmd.push_param(key);
        }

        cmd
    }
}
