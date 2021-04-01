use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// ```text
/// tmux ^2.4:
/// tmux unbind-key [-an] [-T key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux unbind-key [-acn] [-t mode-table] [-T key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux unbind-key [-acn] [-t mode-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^1.4:
/// ```text
/// tmux unbind-key [-acn] [-t key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux unbind-key [-cn] [-t key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux unbind-key key
/// (alias: unbind)
/// ```
#[derive(Debug, Clone)]
pub struct UnbindKey<'a>(pub TmuxCommand<'a>);

impl<'a> Default for UnbindKey<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(UNBIND_KEY)),
            ..Default::default()
        })
    }
}

impl<'a> UnbindKey<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_4")]
    pub fn all(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-c]`
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub fn command_mode(&mut self) -> &mut Self {
        self.0.push_flag(C_LOWERCASE_KEY);
        self
    }

    /// `[-n]`
    #[cfg(feature = "tmux_1_0")]
    pub fn root(&mut self) -> &mut Self {
        self.0.push_flag(N_LOWERCASE_KEY);
        self
    }

    /// `[-t mode-key]`
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    pub fn mode_key<S: Into<Cow<'a, str>>>(&mut self, key_table: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, key_table);
        self
    }

    /// `[-t key-table]`
    /// `[-T key-table]`
    #[cfg(feature = "tmux_1_0")]
    pub fn key_table<S: Into<Cow<'a, str>>>(&mut self, key_table: S) -> &mut Self {
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
        self.0.push_option(T_LOWERCASE_KEY, key_table);
        #[cfg(feature = "tmux_2_4")]
        self.0.push_option(T_UPPERCASE_KEY, key_table);
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

impl<'a> From<TmuxCommand<'a>> for UnbindKey<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(UNBIND_KEY)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for UnbindKey<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(UNBIND_KEY)),
            ..Default::default()
        })
    }
}
