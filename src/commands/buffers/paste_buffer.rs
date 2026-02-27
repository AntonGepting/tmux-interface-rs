// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type PasteB<'a> = PasteBuffer<'a>;

/// Structure for inserting the contents of a paste buffer into the specified pane
///
/// # Manual
///
/// tmux >=1.9:
/// ```text
/// paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
///
/// tmux >=1.5:
/// ```text
/// paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
///
/// tmux >=0.8:
/// ```text
/// paste-buffer [-d] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PasteBuffer<'a> {
    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub delete: bool,

    /// `[-p]`
    #[cfg(feature = "tmux_1_7")]
    pub bracket_codes: bool,

    /// `[-r]`
    #[cfg(feature = "tmux_1_5")]
    pub no_replacement: bool,

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub buffer_index: Option<Cow<'a, str>>,

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub buffer_name: Option<Cow<'a, str>>,

    /// `[-s separator]`
    #[cfg(feature = "tmux_1_5")]
    pub separator: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,
}

impl<'a> PasteBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }

    /// `[-p]`
    #[cfg(feature = "tmux_1_7")]
    pub fn bracket_codes(mut self) -> Self {
        self.bracket_codes = true;
        self
    }

    /// `[-r]`
    #[cfg(feature = "tmux_1_5")]
    pub fn no_replacement(mut self) -> Self {
        self.no_replacement = true;
        self
    }

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub fn buffer_index<S: Into<Cow<'a, str>>>(mut self, buffer_index: S) -> Self {
        self.buffer_index = Some(buffer_index.into());
        self
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(mut self, buffer_name: S) -> Self {
        self.buffer_name = Some(buffer_name.into());
        self
    }

    /// `[-s separator]`
    #[cfg(feature = "tmux_1_5")]
    pub fn separator<S: Into<Cow<'a, str>>>(mut self, separator: S) -> Self {
        self.separator = Some(separator.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(PASTE_BUFFER);

        // `[-d]`
        #[cfg(feature = "tmux_0_8")]
        if self.delete {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-p]`
        #[cfg(feature = "tmux_1_7")]
        if self.bracket_codes {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-r]`
        #[cfg(feature = "tmux_1_5")]
        if self.no_replacement {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-b buffer-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
        if let Some(buffer_index) = self.buffer_index {
            cmd.push_option(B_LOWERCASE_KEY, buffer_index);
        }

        // `[-b buffer-name]`
        #[cfg(feature = "tmux_2_0")]
        if let Some(buffer_name) = self.buffer_name {
            cmd.push_option(B_LOWERCASE_KEY, buffer_name);
        }

        // `[-s separator]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(separator) = self.separator {
            cmd.push_option(S_LOWERCASE_KEY, separator);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        cmd
    }
}
