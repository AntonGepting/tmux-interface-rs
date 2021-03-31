use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux list-keys [-1aN] [-P prefix-string -T key-table]
/// (alias: lsk)
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux list-keys [-T key-table]
/// (alias: lsk)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux list-keys [-t mode-table] [-T key-table]
/// (alias: lsk)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-keys [-t key-table]
/// (alias: lsk)
/// ```
#[derive(Debug, Clone)]
pub struct ListKeys<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ListKeys<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LIST_KEYS)),
            ..Default::default()
        })
    }
}

impl<'a> ListKeys<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn first(&mut self) -> &mut Self {
        self.0.push_flag(_1_KEY);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn command(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn with_notes(&mut self) -> &mut Self {
        self.0.push_flag(N_UPPERCASE_KEY);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn prefix_string<S: Into<Cow<'a, str>>>(&mut self, prefix_string: S) -> &mut Self {
        self.0.push_option(P_KEY, prefix_string);
        self
    }

    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    pub fn mode_table<S: Into<Cow<'a, str>>>(&mut self, mode_table: S) -> &mut Self {
        self.0.push_option(t_KEY, mode_table);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn key_table<S: Into<Cow<'a, str>>>(&mut self, key_table: S) -> &mut Self {
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
        self.0.push_option(t_KEY, key_table);
        #[cfg(feature = "tmux_2_1")]
        self.0.push_option(T_UPPERCASE_KEY, key_table);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ListKeys<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LIST_KEYS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ListKeys<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LIST_KEYS)),
            ..Default::default()
        })
    }
}
