use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

// XXX: better result return?
/// List all sessions managed by the server
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// tmux list-sessions [-F format]
/// (alias: ls)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-sessions
/// (alias: ls)
/// ```
#[derive(Debug, Default, Clone)]
pub struct ListSessions<'a> {
    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub format: Option<Cow<'a, str>>,
}

impl<'a> ListSessions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.format = Some(format.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(LIST_SESSIONS);

        // `[-F format]`
        #[cfg(feature = "tmux_1_6")]
        if let Some(format) = &self.format {
            cmd.push_option(F_UPPERCASE_KEY, format.as_ref());
        }

        cmd
    }
}
