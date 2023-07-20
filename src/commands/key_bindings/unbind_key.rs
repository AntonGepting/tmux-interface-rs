use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Unbind<'a> = UnbindKey<'a>;

/// # Manual
///
/// tmux ^3.2:
/// ```text
/// unbind-key [-anq] [-T key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^2.4:
/// ```text
/// unbind-key [-an] [-T key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^2.1:
/// ```text
/// unbind-key [-acn] [-t mode-table] [-T key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^2.0:
/// ```text
/// unbind-key [-acn] [-t mode-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^1.4:
/// ```text
/// unbind-key [-acn] [-t key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^1.0:
/// ```text
/// unbind-key [-cn] [-t key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^0.8:
/// ```text
/// unbind-key key
/// (alias: unbind)
/// ```
// FIXME: -t -T flags
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct UnbindKey<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_1_4")]
    pub all: bool,

    /// `[-c]`
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub command_mode: bool,

    /// `[-n]`
    #[cfg(feature = "tmux_1_0")]
    pub root: bool,

    /// `[-q]`
    #[cfg(feature = "tmux_3_2")]
    pub quiet: bool,

    /// `[-t mode-key]`
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    pub mode_key: Option<Cow<'a, str>>,

    /// `[-t key-table]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub key_table: Option<Cow<'a, str>>,

    /// `[-T key-table]`
    #[cfg(feature = "tmux_2_1")]
    pub key_table: Option<Cow<'a, str>>,

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub key: Option<Cow<'a, str>>,
}

impl<'a> UnbindKey<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_4")]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// `[-c]`
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub fn command_mode(mut self) -> Self {
        self.command_mode = true;
        self
    }

    /// `[-n]`
    #[cfg(feature = "tmux_1_0")]
    pub fn root(mut self) -> Self {
        self.root = true;
        self
    }

    /// `[-q]`
    #[cfg(feature = "tmux_3_2")]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// `[-t mode-key]`
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    pub fn mode_key<S: Into<Cow<'a, str>>>(mut self, key_table: S) -> Self {
        self.mode_key = Some(key_table.into());
        self
    }

    /// `[-t key-table]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub fn key_table<S: Into<Cow<'a, str>>>(mut self, key_table: S) -> Self {
        self.key_table = Some(key_table.into());
        self
    }

    /// `[-T key-table]`
    #[cfg(feature = "tmux_2_1")]
    pub fn key_table<S: Into<Cow<'a, str>>>(mut self, key_table: S) -> Self {
        self.key_table = Some(key_table.into());
        self
    }

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub fn key<S: Into<Cow<'a, str>>>(mut self, key: S) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(UNBIND_KEY);

        // `[-a]`
        #[cfg(feature = "tmux_1_4")]
        if self.all {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-c]`
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
        if self.command_mode {
            cmd.push_flag(C_LOWERCASE_KEY);
        }

        // `[-n]`
        #[cfg(feature = "tmux_1_0")]
        if self.root {
            cmd.push_flag(N_LOWERCASE_KEY);
        }

        // `[-q]`
        #[cfg(feature = "tmux_3_2")]
        if self.quiet {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-t mode-key]`
        #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
        if let Some(mode_key) = self.mode_key {
            cmd.push_option(T_LOWERCASE_KEY, mode_key);
        }

        // `[-t key-table]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
        if let Some(key_table) = self.key_table {
            cmd.push_option(T_LOWERCASE_KEY, key_table);
        }

        // `[-T key-table]`
        #[cfg(feature = "tmux_2_1")]
        if let Some(key_table) = self.key_table {
            cmd.push_option(T_UPPERCASE_KEY, key_table);
        }

        // `key`
        #[cfg(feature = "tmux_0_8")]
        if let Some(key) = self.key {
            cmd.push_param(key);
        }

        cmd
    }
}
