use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type ShowMsgs<'a> = ShowMessages<'a>;

/// Show client messages or server information
///
/// # Manual
///
/// tmux ^2.2:
/// ```text
/// show-messages [-JT] [-t target-client]
/// (alias: showmsgs)
/// ```
///
/// tmux ^1.9:
/// ```text
/// show-messages [-IJT] [-t target-client]
/// (alias: showmsgs)
/// ```
///
/// tmux ^1.2:
/// ```text
/// show-messages [-t target-client]
/// (alias: showmsgs)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ShowMessages<'a> {
    /// `[-I]`
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    pub server: bool,
    /// `[-J]`
    #[cfg(feature = "tmux_1_9")]
    pub jobs: bool,
    /// `[-T]`
    #[cfg(feature = "tmux_1_9")]
    pub terminals: bool,
    /// `[-t target-client]`
    #[cfg(feature = "tmux_1_2")]
    pub target_client: Option<Cow<'a, str>>,
}

impl<'a> ShowMessages<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-I]`
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    pub fn server(mut self) -> Self {
        self.server = true;
        self
    }

    /// `[-J]`
    #[cfg(feature = "tmux_1_9")]
    pub fn jobs(mut self) -> Self {
        self.jobs = true;
        self
    }

    /// `[-T]`
    #[cfg(feature = "tmux_1_9")]
    pub fn terminals(mut self) -> Self {
        self.terminals = true;
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_1_2")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SHOW_MESSAGES);

        // `[-I]`
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
        if self.server {
            cmd.push_flag(I_UPPERCASE_KEY);
        }

        // `[-J]`
        #[cfg(feature = "tmux_1_9")]
        if self.jobs {
            cmd.push_flag(J_UPPERCASE_KEY);
        }

        // `[-T]`
        #[cfg(feature = "tmux_1_9")]
        if self.terminals {
            cmd.push_flag(T_UPPERCASE_KEY);
        }

        // `[-t target-client]`
        #[cfg(feature = "tmux_1_2")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client);
        }

        cmd
    }
}
