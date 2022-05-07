use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Show client messages or server information
///
/// # Manual
///
/// tmux ^2.2:
/// ```text
/// tmux show-messages [-JT] [-t target-client]
/// (alias: showmsgs)
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux show-messages [-IJT] [-t target-client]
/// (alias: showmsgs)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux show-messages [-t target-client]
/// (alias: showmsgs)
/// ```
#[derive(Debug, Default, Clone)]
pub struct ShowMessages<'a> {
    /// `[-I]`
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    pub server: Option<bool>,
    /// `[-J]`
    #[cfg(feature = "tmux_1_9")]
    pub jobs: Option<bool>,
    /// `[-T]`
    #[cfg(feature = "tmux_1_9")]
    pub terminals: Option<bool>,
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
    pub fn server(&mut self) -> &mut Self {
        self.server = Some(true);
        self
    }

    /// `[-J]`
    #[cfg(feature = "tmux_1_9")]
    pub fn jobs(&mut self) -> &mut Self {
        self.jobs = Some(true);
        self
    }

    /// `[-T]`
    #[cfg(feature = "tmux_1_9")]
    pub fn terminals(&mut self) -> &mut Self {
        self.terminals = Some(true);
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_1_2")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.target_client = Some(target_client.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SHOW_MESSAGES);

        // `[-I]`
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
        if self.server.is_some() {
            cmd.push_flag(I_UPPERCASE_KEY);
        }

        // `[-J]`
        #[cfg(feature = "tmux_1_9")]
        if self.jobs.is_some() {
            cmd.push_flag(J_UPPERCASE_KEY);
        }

        // `[-T]`
        #[cfg(feature = "tmux_1_9")]
        if self.terminals.is_some() {
            cmd.push_flag(T_UPPERCASE_KEY);
        }

        // `[-t target-client]`
        #[cfg(feature = "tmux_1_2")]
        if let Some(target_client) = &self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client.as_ref());
        }

        cmd
    }
}
