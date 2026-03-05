// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type SuspendC<'a> = SuspendClient<'a>;

/// Suspend a client by sending SIGTSTP (tty stop)
///
/// # Manual
///
/// tmux >=1.5:
/// ```text
/// suspend-client [-t target-client]
/// (alias: suspendc)
/// ```
///
/// tmux >=0.8:
/// ```text
/// suspend-client [-c target-client]
/// (alias: suspendc)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SuspendClient<'a> {
    /// `[-c target-client]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-t target-client]`
    #[cfg(feature = "tmux_1_5")]
    pub target_client: Option<Cow<'a, str>>,
}

impl<'a> SuspendClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-c target-client]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SUSPEND_CLIENT);

        // `[-c target-client]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_client) = self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client);
        }

        // `[-t target-client]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client);
        }

        cmd
    }
}
