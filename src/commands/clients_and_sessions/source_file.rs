use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
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

    #[cfg(feature = "tmux_3_0")]
    pub fn not_execute(&mut self) -> &mut Self {
        self.0.push_flag(n_KEY);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn quite(&mut self) -> &mut Self {
        self.0.push_flag(q_KEY);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn verbose(&mut self) -> &mut Self {
        self.0.push_flag(v_KEY);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn path<S: Into<Cow<'a, str>>>(&mut self, path: S) -> &mut Self {
        self.0.push_param(path);
        self
    }

    pub fn output(&self) -> TmuxOutput {
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
