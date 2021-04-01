use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure binding key `key` to command
///
/// # Manual
///
/// tmux 3.1:
/// ```text
/// tmux bind-key [-nr] [-N note] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux bind-key [-nr] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.3:
/// ```text
/// tmux bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux bind-key [-cnr] [-t mode-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux bind-key [-cnr] [-t key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux bind-key [-r] key command [arguments]
/// (alias: bind)
/// ```
#[derive(Debug, Clone)]
pub struct BindKey<'a>(pub TmuxCommand<'a>);

impl<'a> Default for BindKey<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(BIND_KEY)),
            ..Default::default()
        })
    }
}

impl<'a> BindKey<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-n]` - an alias for -T root
    #[cfg(feature = "tmux_1_0")]
    pub fn root(&mut self) -> &mut Self {
        self.0.push_flag(N_LOWERCASE_KEY);
        self
    }

    /// `[-r]` - this key may repeat
    #[cfg(feature = "tmux_0_8")]
    pub fn repeat(&mut self) -> &mut Self {
        self.0.push_flag(R_LOWERCASE_KEY);
        self
    }

    /// `[-N note]` - attaches note to the key
    #[cfg(feature = "tmux_3_1")]
    pub fn note<S: Into<Cow<'a, str>>>(&mut self, note: S) -> &mut Self {
        self.0.push_option(N_UPPERCASE_KEY, note);
        self
    }

    /// `[-T key-table]` - key-table
    #[cfg(feature = "tmux_2_1")]
    pub fn key_table<S: Into<Cow<'a, str>>>(&mut self, key_table: S) -> &mut Self {
        self.0.push_option(T_UPPERCASE_KEY, key_table);
        self
    }

    /// `[arguments]` - arguments
    #[cfg(feature = "tmux_0_8")]
    pub fn arguments<S: Into<Cow<'a, str>>>(&mut self, key_table: S) -> &mut Self {
        self.0.push_param(key_table);
        self
    }

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub fn key<S: Into<Cow<'a, str>>>(&mut self, key: S) -> &mut Self {
        self.0.push_param(key);
        self
    }

    /// `command`
    #[cfg(feature = "tmux_0_8")]
    pub fn command<S: Into<Cow<'a, str>>>(&mut self, command: S) -> &mut Self {
        self.0.push_param(command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for BindKey<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(BIND_KEY)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for BindKey<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(BIND_KEY)),
            ..Default::default()
        })
    }
}
