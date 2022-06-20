use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;
use std::marker::PhantomData;

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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListSessions<'a> {
    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub format: Option<Cow<'a, str>>,

    _phantom_data: PhantomData<&'a ()>,
}

impl<'a> ListSessions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LIST_SESSIONS);

        // `[-F format]`
        #[cfg(feature = "tmux_1_6")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        cmd
    }
}
