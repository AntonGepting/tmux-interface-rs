use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type PasteB<'a> = PasteBuffer<'a>;

/// Structure for inserting the contents of a paste buffer into the specified pane
///
/// # Manual
///
/// tmux ^1.7:
/// ```text
/// paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
///
/// tmux ^1.3:
/// ```text
/// paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-window]
/// (alias: pasteb)
/// ```
///
/// tmux ^1.0:
/// ```text
/// paste-buffer [-dr] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// paste-buffer [-d] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PasteBuffer<'a> {
    /// `[-d]` - delete the paste buffer
    #[cfg(feature = "tmux_0_8")]
    pub delete: bool,

    /// `[-p]` - paste bracket control codes are inserted around the buffer
    #[cfg(feature = "tmux_1_7")]
    pub bracket_codes: bool,

    /// `[-r]` - do no replacement (equivalent to a separator of LF)
    #[cfg(feature = "tmux_1_0")]
    pub no_replacement: bool,

    /// `[-b buffer-name]` - specify the buffer mode
    #[cfg(feature = "tmux_1_7")]
    pub buffer_name: Option<Cow<'a, str>>,

    /// `[-s separator]` - specify a separator
    #[cfg(feature = "tmux_1_3")]
    pub separator: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_1_7")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-t target-window]` - specify the target window
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub target_window: Option<Cow<'a, str>>,
    // buffer-index
}

impl<'a> PasteBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]` - delete the paste buffer
    #[cfg(feature = "tmux_0_8")]
    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }

    /// `[-p]` - paste bracket control codes are inserted around the buffer
    #[cfg(feature = "tmux_1_7")]
    pub fn bracket_codes(mut self) -> Self {
        self.bracket_codes = true;
        self
    }

    /// `[-r]` - do no replacement (equivalent to a separator of LF)
    #[cfg(feature = "tmux_1_0")]
    pub fn no_replacement(mut self) -> Self {
        self.no_replacement = true;
        self
    }

    /// `[-b buffer-name]` - specify the buffer mode
    #[cfg(feature = "tmux_1_7")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(mut self, buffer_name: S) -> Self {
        self.buffer_name = Some(buffer_name.into());
        self
    }

    /// `[-s separator]` - specify a separator
    #[cfg(feature = "tmux_1_3")]
    pub fn separator<S: Into<Cow<'a, str>>>(mut self, separator: S) -> Self {
        self.separator = Some(separator.into());
        self
    }

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_1_7")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-t target-window]` - specify the target window
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(PASTE_BUFFER);

        // `[-d]` - delete the paste buffer
        #[cfg(feature = "tmux_0_8")]
        if self.delete {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-p]` - paste bracket control codes are inserted around the buffer
        #[cfg(feature = "tmux_1_7")]
        if self.bracket_codes {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-r]` - do no replacement (equivalent to a separator of LF)
        #[cfg(feature = "tmux_1_0")]
        if self.no_replacement {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-b buffer-name]` - specify the buffer mode
        #[cfg(feature = "tmux_1_7")]
        if let Some(buffer_name) = self.buffer_name {
            cmd.push_option(B_LOWERCASE_KEY, buffer_name);
        }

        // `[-s separator]` - specify a separator
        #[cfg(feature = "tmux_1_3")]
        if let Some(separator) = self.separator {
            cmd.push_option(S_LOWERCASE_KEY, separator);
        }

        // `[-t target-pane]` - specify the target pane
        #[cfg(feature = "tmux_1_7")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-t target-window]` - specify the target window
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        cmd
    }
}
