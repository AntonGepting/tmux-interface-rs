use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
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
#[derive(Debug, Clone)]
pub struct LockClient<'a>(TmuxCommand<'a>);

impl<'a> Default for LockClient<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LOCK_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> LockClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn target_session<S: Into<String>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(t_KEY, target_client);
        self
    }

    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for LockClient<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LOCK_CLIENT)),
            ..Default::default()
        })
    }
}
