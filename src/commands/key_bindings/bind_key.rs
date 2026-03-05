// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Bind<'a> = BindKey<'a>;

// FIXME: -c flag support
/// Bind key `key` to command
///
/// # Manual
///
/// tmux >=3.1:
/// ```text
/// bind-key [-nr] [-N note] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=2.4:
/// ```text
/// bind-key [-nr] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=2.3:
/// ```text
/// bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=2.1:
/// ```text
/// bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=2.0:
/// ```text
/// bind-key [-cnr] [-t mode-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=1.5:
/// ```text
/// bind-key [-cnr] [-t key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=0.8:
/// ```text
/// bind-key [-r] key command [arguments]
/// (alias: bind)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BindKey<'a> {
    /// `[-c]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    pub command_mode: bool,

    /// `[-n]`
    #[cfg(feature = "tmux_1_5")]
    pub root: bool,

    /// `[-r]`
    #[cfg(feature = "tmux_1_5")]
    pub repeat: bool,

    /// `[-N note]`
    #[cfg(feature = "tmux_3_1")]
    pub note: Option<Cow<'a, str>>,

    /// `[-R repeat-count]`
    #[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
    pub repeat_count: Option<Cow<'a, str>>,

    /// `[-t key-table]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    pub key_table: Option<Cow<'a, str>>,

    /// `[-t mode-table]`
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    pub mode_table: Option<Cow<'a, str>>,

    /// `[-T key-table]`
    #[cfg(feature = "tmux_2_1")]
    pub key_table: Option<Cow<'a, str>>,

    /// `[key]`
    #[cfg(feature = "tmux_0_8")]
    pub key: Option<Cow<'a, str>>,

    /// `[command]`
    #[cfg(feature = "tmux_0_8")]
    pub command: Option<Cow<'a, str>>,

    /// `[arguments]`
    #[cfg(feature = "tmux_0_8")]
    pub arguments: Option<Cow<'a, str>>,
}

impl<'a> BindKey<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-c]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    pub fn command_mode(mut self) -> Self {
        self.command_mode = true;
        self
    }

    /// `[-n]`
    #[cfg(feature = "tmux_1_5")]
    pub fn root(mut self) -> Self {
        self.root = true;
        self
    }

    /// `[-r]`
    #[cfg(feature = "tmux_1_5")]
    pub fn repeat(mut self) -> Self {
        self.repeat = true;
        self
    }

    /// `[-N note]`
    #[cfg(feature = "tmux_3_1")]
    pub fn note<S: Into<Cow<'a, str>>>(mut self, note: S) -> Self {
        self.note = Some(note.into());
        self
    }

    /// `[-R repeat-count]`
    #[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
    pub fn repeat_count<S: Into<Cow<'a, str>>>(mut self, repeat_count: S) -> Self {
        self.repeat_count = Some(repeat_count.into());
        self
    }

    /// `[-t key-table]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    pub fn key_table<S: Into<Cow<'a, str>>>(mut self, key_table: S) -> Self {
        self.key_table = Some(key_table.into());
        self
    }

    /// `[-t mode-table]`
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
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
    #[cfg(feature = "tmux_0_8")]
    pub fn key<S: Into<Cow<'a, str>>>(mut self, key: S) -> Self {
        self.key = Some(key.into());
        self
    }

    /// `[command]`
    #[cfg(feature = "tmux_0_8")]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    /// `[arguments]`
    #[cfg(feature = "tmux_0_8")]
    pub fn arguments<S: Into<Cow<'a, str>>>(mut self, arguments: S) -> Self {
        self.arguments = Some(arguments.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(BIND_KEY);

        // `[-c]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
        if self.command_mode {
            cmd.push_flag(C_LOWERCASE_KEY);
        }

        // `[-n]`
        #[cfg(feature = "tmux_1_5")]
        if self.root {
            cmd.push_flag(N_LOWERCASE_KEY);
        }

        // `[-r]`
        #[cfg(feature = "tmux_1_5")]
        if self.repeat {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-N note]`
        #[cfg(feature = "tmux_3_1")]
        if let Some(note) = self.note {
            cmd.push_option(N_UPPERCASE_KEY, note);
        }

        // `[-R repeat-count]`
        #[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
        if let Some(repeat_count) = self.repeat_count {
            cmd.push_option(R_UPPERCASE_KEY, repeat_count);
        }

        // `[-t key-table]`
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
        if let Some(key_table) = self.key_table {
            cmd.push_option(T_LOWERCASE_KEY, key_table);
        }

        // `[-t mode-table]`
        #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
        if let Some(mode_table) = self.mode_table {
            cmd.push_option(T_LOWERCASE_KEY, mode_table);
        }

        // `[-T key-table]`
        #[cfg(feature = "tmux_2_1")]
        if let Some(key_table) = self.key_table {
            cmd.push_option(T_UPPERCASE_KEY, key_table);
        }

        // `[key]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(key) = self.key {
            cmd.push_param(key);
        }

        // `[command]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        // `[arguments]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(arguments) = self.arguments {
            cmd.push_param(arguments);
        }

        cmd
    }
}
