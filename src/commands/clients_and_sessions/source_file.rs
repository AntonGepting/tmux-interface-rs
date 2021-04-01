use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Execute commands from path
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux source-file [-nqv] path
/// (alias: source)
/// ```
///
/// tmux ^2.3:
/// ```text
/// tmux source-file path
/// (alias: source)
///
/// ```
/// tmux ^0.8:
/// ```text
/// tmux source-file path
/// (alias: source)
/// ```
#[derive(Debug, Clone)]
pub struct SourceFile<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SourceFile<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SOURCE_FILE)),
            ..Default::default()
        })
    }
}

impl<'a> SourceFile<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-n]`
    #[cfg(feature = "tmux_3_0")]
    pub fn not_execute(&mut self) -> &mut Self {
        self.0.push_flag(N_LOWERCASE_KEY);
        self
    }

    /// `[-q]`
    #[cfg(feature = "tmux_3_0")]
    pub fn quite(&mut self) -> &mut Self {
        self.0.push_flag(Q_LOWERCASE_KEY);
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_3_0")]
    pub fn verbose(&mut self) -> &mut Self {
        self.0.push_flag(V_LOWERCASE_KEY);
        self
    }

    /// `path`
    #[cfg(feature = "tmux_0_8")]
    pub fn path<S: Into<Cow<'a, str>>>(&mut self, path: S) -> &mut Self {
        self.0.push_param(path);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SourceFile<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SOURCE_FILE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SourceFile<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SOURCE_FILE)),
            ..Default::default()
        })
    }
}
