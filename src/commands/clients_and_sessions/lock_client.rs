use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Lock `target-client`
///
/// # Manual
///
/// tmux ^1.1:
/// ```text
/// tmux lock-client [-t target-client]
/// (alias: lockc)
/// ```
#[derive(Debug, Default, Clone)]
pub struct LockClient<'a> {
    /// `[-t target-client]`
    #[cfg(feature = "tmux_1_1")]
    pub target_client: Option<Cow<'a, str>>,
}

impl<'a> LockClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_1_1")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.target_client = Some(target_client.into());
        self
    }

    pub fn output(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(LOCK_CLIENT);

        /// `[-t target-client]`
        #[cfg(feature = "tmux_1_1")]
        if let Some(target_client) = &self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client.as_ref());
        }

        cmd
    }
}
