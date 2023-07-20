use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type SuspendC<'a> = SuspendClient<'a>;

/// Suspend a client by sending SIGTSTP (tty stop)
///
/// # Manual
///
/// tmux ^1.5:
/// ```text
/// suspend-client [-t target-client]
/// (alias: suspendc)
/// ```
///
/// tmux ^0.8:
/// ```text
/// suspend-client [-c target-client]
/// (alias: suspendc)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SuspendClient<'a> {
    /// `[-t target-client]`
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<Cow<'a, str>>,
}

impl<'a> SuspendClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SUSPEND_CLIENT);

        // `[-t target-client]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_client) = self.target_client {
            #[cfg(feature = "tmux_1_5")]
            cmd.push_option(T_LOWERCASE_KEY, target_client);
            #[cfg(not(feature = "tmux_1_5"))]
            cmd.push_option(C_LOWERCASE_KEY, target_client);
        }

        cmd
    }
}
