use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux unbind-key [-anq] [-T key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^2.4:
/// ```text
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
    pub quite: bool,

    /// `[-t mode-key]`
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    pub mode_key: Option<Cow<'a, str>>,

    /// `[-t key-table]`
    /// `[-T key-table]`
    #[cfg(feature = "tmux_1_0")]
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
    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }

    /// `[-c]`
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub fn command_mode(&mut self) -> &mut Self {
        self.command_mode = true;
        self
    }

    /// `[-n]`
    #[cfg(feature = "tmux_1_0")]
    pub fn root(&mut self) -> &mut Self {
        self.root = true;
        self
    }

    /// `[-q]`
    #[cfg(feature = "tmux_3_2")]
    pub fn quite(&mut self) -> &mut Self {
        self.quiet = true;
        self
    }

    /// `[-t mode-key]`
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    pub fn mode_key<S: Into<Cow<'a, str>>>(&mut self, key_table: S) -> &mut Self {
        self.mode_key = Some(key_table.into());
        self
    }

    /// `[-t key-table]`
    /// `[-T key-table]`
    #[cfg(feature = "tmux_1_0")]
    pub fn key_table<S: Into<Cow<'a, str>>>(&mut self, key_table: S) -> &mut Self {
        self.key_table = Some(key_table.into());
        self
    }

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub fn key<S: Into<Cow<'a, str>>>(&mut self, key: S) -> &mut Self {
        self.key = Some(key.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(UNBIND_KEY);

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
        if let Some(mode_key) = &self.mode_key {
            cmd.push_option(T_LOWERCASE_KEY, mode_key.as_ref());
        }

        // `[-t key-table]`
        // `[-T key-table]`
        #[cfg(feature = "tmux_1_0")]
        if let Some(key_table) = &self.key_table {
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
            cmd.push_option(T_LOWERCASE_KEY, key_table.as_ref());
            #[cfg(feature = "tmux_2_4")]
            cmd.push_option(T_UPPERCASE_KEY, key_table.as_ref());
        }

        // `key`
        #[cfg(feature = "tmux_0_8")]
        if let Some(key) = &self.key {
            cmd.push_param(key.as_ref());
        }

        cmd
    }
}
