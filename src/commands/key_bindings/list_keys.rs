// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type LsK<'a> = ListKeys<'a>;

/// List key bindings
///
/// # Manual
///
/// tmux >=3.1a:
/// ```text
/// list-keys [-1aN] [-P prefix-string -T key-table]
/// (alias: lsk)
/// ```
///
/// tmux >=3.1:
/// ```text
/// list-keys [-1N] [-P prefix-string -T key-table] [key]
/// (alias: lsk)
/// ```
///
/// tmux >=2.4:
/// ```text
/// list-keys [-T key-table]
/// (alias: lsk)
/// ```
///
/// tmux >=2.1:
/// ```text
/// list-keys [-t mode-table] [-T key-table]
/// (alias: lsk)
/// ```
///
/// tmux >=0.8:
/// ```text
/// list-keys [-t key-table]
/// (alias: lsk)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListKeys<'a> {
    /// `[-1]`
    #[cfg(feature = "tmux_3_1")]
    pub first: bool,

    /// `[-a]`
    #[cfg(feature = "tmux_3_1a")]
    pub command: bool,

    /// `[-N]`
    #[cfg(feature = "tmux_3_1")]
    pub with_notes: bool,

    /// `[-P prefix-string]`
    #[cfg(feature = "tmux_3_1")]
    pub prefix_string: Option<Cow<'a, str>>,

    /// `[-t key-table]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    pub key_table: Option<Cow<'a, str>>,

    /// `[-t mode-table]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    pub mode_table: Option<Cow<'a, str>>,

    /// `[-T key-table]`
    #[cfg(feature = "tmux_2_1")]
    pub key_table: Option<Cow<'a, str>>,

    /// `[key]`
    #[cfg(feature = "tmux_3_1")]
    pub key: Option<Cow<'a, str>>,
}

impl<'a> ListKeys<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-1]`
    #[cfg(feature = "tmux_3_1")]
    pub fn first(mut self) -> Self {
        self.first = true;
        self
    }

    /// `[-a]`
    #[cfg(feature = "tmux_3_1a")]
    pub fn command(mut self) -> Self {
        self.command = true;
        self
    }

    /// `[-N]`
    #[cfg(feature = "tmux_3_1")]
    pub fn with_notes(mut self) -> Self {
        self.with_notes = true;
        self
    }

    /// `[-P prefix-string]`
    #[cfg(feature = "tmux_3_1")]
    pub fn prefix_string<S: Into<Cow<'a, str>>>(mut self, prefix_string: S) -> Self {
        self.prefix_string = Some(prefix_string.into());
        self
    }

    /// `[-t key-table]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    pub fn key_table<S: Into<Cow<'a, str>>>(mut self, key_table: S) -> Self {
        self.key_table = Some(key_table.into());
        self
    }

    /// `[-t mode-table]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    pub fn mode_table<S: Into<Cow<'a, str>>>(mut self, mode_table: S) -> Self {
        self.mode_table = Some(mode_table.into());
        self
    }

    /// `[-T key-table]`
    #[cfg(feature = "tmux_2_1")]
    pub fn key_table<S: Into<Cow<'a, str>>>(mut self, key_table: S) -> Self {
        self.key_table = Some(key_table.into());
        self
    }

    /// `[key]`
    #[cfg(feature = "tmux_3_1")]
    pub fn key<S: Into<Cow<'a, str>>>(mut self, key: S) -> Self {
        self.key = Some(key.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LIST_KEYS);

        // `[-1]`
        #[cfg(feature = "tmux_3_1")]
        if self.first {
            cmd.push_flag(_1_KEY);
        }

        // `[-a]`
        #[cfg(feature = "tmux_3_1a")]
        if self.command {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-N]`
        #[cfg(feature = "tmux_3_1")]
        if self.with_notes {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-P prefix-string]`
        #[cfg(feature = "tmux_3_1")]
        if let Some(prefix_string) = self.prefix_string {
            cmd.push_option(P_UPPERCASE_KEY, prefix_string);
        }

        // `[-t key-table]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
        if let Some(key_table) = self.key_table {
            cmd.push_option(T_LOWERCASE_KEY, key_table);
        }

        // `[-t mode-table]`
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
        if let Some(mode_table) = self.mode_table {
            cmd.push_option(T_LOWERCASE_KEY, mode_table);
        }

        // `[-T key-table]`
        #[cfg(feature = "tmux_2_1")]
        if let Some(key_table) = self.key_table {
            cmd.push_option(T_UPPERCASE_KEY, key_table);
        }

        // `[key]`
        #[cfg(feature = "tmux_3_1")]
        if let Some(key) = self.key {
            cmd.push_param(key);
        }

        cmd
    }
}
