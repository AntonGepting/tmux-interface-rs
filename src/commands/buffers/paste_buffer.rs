use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for inserting the contents of a paste buffer into the specified pane
///
/// # Manual
///
/// tmux ^1.7:
/// ```text
/// tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-window]
/// (alias: pasteb)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux paste-buffer [-dr] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux paste-buffer [-d] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
#[derive(Debug, Clone)]
pub struct PasteBuffer<'a>(pub TmuxCommand<'a>);

impl<'a> Default for PasteBuffer<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(PASTE_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> PasteBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]` - delete the paste buffer
    pub fn delete(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-p]` - paste bracket control codes are inserted around the buffer
    #[cfg(feature = "tmux_1_7")]
    pub fn bracket_codes(&mut self) -> &mut Self {
        self.0.push_flag(P_LOWERCASE_KEY);
        self
    }

    /// `[-r]` - do no replacement (equivalent to a separator of LF)
    #[cfg(feature = "tmux_1_0")]
    pub fn no_replacement(&mut self) -> &mut Self {
        self.0.push_flag(R_LOWERCASE_KEY);
        self
    }

    /// `[-b buffer-name]` - specify the buffer mode
    #[cfg(feature = "tmux_1_7")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(&mut self, buffer_name: S) -> &mut Self {
        self.0.push_option(B_LOWERCASE_KEY, buffer_name);
        self
    }

    /// `[-s separator]` - specify a separator
    #[cfg(feature = "tmux_1_3")]
    pub fn separator<S: Into<Cow<'a, str>>>(&mut self, separator: S) -> &mut Self {
        self.0.push_option(S_LOWERCASE_KEY, separator);
        self
    }

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_1_7")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-t target-window]` - specify the target window
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for PasteBuffer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(PASTE_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for PasteBuffer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(PASTE_BUFFER)),
            ..Default::default()
        })
    }
}
