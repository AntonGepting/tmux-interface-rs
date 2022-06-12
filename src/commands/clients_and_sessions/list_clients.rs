use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;
use std::marker::PhantomData;

/// List all clients attached to the server
///
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// tmux list-clients [-F format] [-t target-session]
/// (alias: lsc)
///
/// ```
/// tmux ^1.5:
/// ```text
/// tmux list-clients [-t target-session]
/// (alias: lsc)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-clients
/// (alias: lsc)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListClients<'a> {
    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub format: Option<Cow<'a, str>>,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    pub target_session: Option<Cow<'a, str>>,

    _phantom_data: PhantomData<&'a ()>,
}

impl<'a> ListClients<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(LIST_CLIENTS);

        // `[-F format]`
        #[cfg(feature = "tmux_1_6")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
