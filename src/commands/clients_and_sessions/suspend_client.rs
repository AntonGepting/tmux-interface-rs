use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Suspend a client by sending SIGTSTP (tty stop)
///
/// # Manual
///
/// tmux ^1.5:
/// ```text
/// tmux suspend-client [-t target-client]
/// (alias: suspendc)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux suspend-client [-c target-client]
/// (alias: suspendc)
/// ```
#[derive(Debug, Clone)]
pub struct SuspendClient<'a>(TmuxCommand<'a>);

impl<'a> Default for SuspendClient<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SUSPEND_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> SuspendClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn target_client<S: Into<String>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(t_KEY, target_client);
        self
    }

    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SuspendClient<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SUSPEND_CLIENT)),
            ..Default::default()
        })
    }
}
