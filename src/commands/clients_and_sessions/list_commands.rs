// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type LsCm<'a> = ListCommands<'a>;

/// List the syntax of all commands supported by tmux
///
/// # Manual
///
/// tmux >=3.1a:
/// ```text
/// list-commands [-F format] [command]
/// (alias: lscm)
/// ```
///
/// tmux >=2.3:
/// ```text
/// list-commands [-F format]
/// (alias: lscm)
/// ```
///
/// tmux >=0.8:
/// ```text
/// list-commands
/// (alias: lscm)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListCommands<'a> {
    /// `[-F format]`
    #[cfg(feature = "tmux_2_3")]
    pub format: Option<Cow<'a, str>>,

    /// `[command]`
    #[cfg(feature = "tmux_3_1a")]
    pub command: Option<Cow<'a, str>>,
}

impl<'a> ListCommands<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_2_3")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[command]`
    #[cfg(feature = "tmux_3_1a")]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LIST_COMMANDS);

        // `[-F format]`
        #[cfg(feature = "tmux_2_3")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[command]`
        #[cfg(feature = "tmux_3_1a")]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        cmd
    }
}
