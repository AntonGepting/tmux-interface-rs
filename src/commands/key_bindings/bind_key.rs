use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

// FIXME: -c flag support
/// Structure binding key `key` to command
///
/// # Manual
///
/// tmux 3.1:
/// ```text
/// bind-key [-nr] [-N note] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.4:
/// ```text
/// bind-key [-nr] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.3:
/// ```text
/// bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.1:
/// ```text
/// bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.0:
/// ```text
/// bind-key [-cnr] [-t mode-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^1.0:
/// ```text
/// bind-key [-cnr] [-t key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^0.8:
/// ```text
/// bind-key [-r] key command [arguments]
/// (alias: bind)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BindKey<'a> {
    /// `[-n]` - an alias for -T root
    #[cfg(feature = "tmux_1_0")]
    pub root: bool,

    /// `[-r]` - this key may repeat
    #[cfg(feature = "tmux_0_8")]
    pub repeat: bool,

    /// `[-N note]` - attaches note to the key
    #[cfg(feature = "tmux_3_1")]
    pub note: Option<Cow<'a, str>>,

    /// `[-T key-table]` - key-table
    #[cfg(feature = "tmux_2_1")]
    pub key_table: Option<Cow<'a, str>>,

    /// `[arguments]` - arguments
    #[cfg(feature = "tmux_0_8")]
    pub arguments: Option<Cow<'a, str>>,

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub key: Option<Cow<'a, str>>,

    /// `command`
    #[cfg(feature = "tmux_0_8")]
    pub command: Option<Cow<'a, str>>,
}

impl<'a> BindKey<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-n]` - an alias for -T root
    #[cfg(feature = "tmux_1_0")]
    pub fn root(mut self) -> Self {
        self.root = true;
        self
    }

    /// `[-r]` - this key may repeat
    #[cfg(feature = "tmux_0_8")]
    pub fn repeat(mut self) -> Self {
        self.repeat = true;
        self
    }

    /// `[-N note]` - attaches note to the key
    #[cfg(feature = "tmux_3_1")]
    pub fn note<S: Into<Cow<'a, str>>>(mut self, note: S) -> Self {
        self.note = Some(note.into());
        self
    }

    /// `[-T key-table]` - key-table
    #[cfg(feature = "tmux_2_1")]
    pub fn key_table<S: Into<Cow<'a, str>>>(mut self, key_table: S) -> Self {
        self.key_table = Some(key_table.into());
        self
    }

    /// `[arguments]` - arguments
    #[cfg(feature = "tmux_0_8")]
    pub fn arguments<S: Into<Cow<'a, str>>>(mut self, key_table: S) -> Self {
        self.arguments = Some(key_table.into());
        self
    }

    /// `key`
    #[cfg(feature = "tmux_0_8")]
    pub fn key<S: Into<Cow<'a, str>>>(mut self, key: S) -> Self {
        self.key = Some(key.into());
        self
    }

    // XXX: command?
    /// `command`
    #[cfg(feature = "tmux_0_8")]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(BIND_KEY);

        // `[-n]` - an alias for -T root
        #[cfg(feature = "tmux_1_0")]
        if self.root {
            cmd.push_flag(N_LOWERCASE_KEY);
        }

        // `[-r]` - this key may repeat
        #[cfg(feature = "tmux_0_8")]
        if self.repeat {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-N note]` - attaches note to the key
        #[cfg(feature = "tmux_3_1")]
        if let Some(note) = self.note {
            cmd.push_option(N_UPPERCASE_KEY, note);
        }

        // `[-T key-table]` - key-table
        #[cfg(feature = "tmux_2_1")]
        if let Some(key_table) = self.key_table {
            cmd.push_option(T_UPPERCASE_KEY, key_table);
        }

        // `[arguments]` - arguments
        #[cfg(feature = "tmux_0_8")]
        if let Some(arguments) = self.arguments {
            cmd.push_param(arguments);
        }

        // `key`
        #[cfg(feature = "tmux_0_8")]
        if let Some(key) = self.key {
            cmd.push_param(key);
        }

        // `command`
        #[cfg(feature = "tmux_0_8")]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        cmd
    }
}
