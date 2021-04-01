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
#[derive(Debug, Clone)]
pub struct LockClient<'a>(pub TmuxCommand<'a>);

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

    /// `[-t target-client]`
    #[cfg(feature = "tmux_1_1")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_client);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
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

impl<'a> From<&TmuxCommand<'a>> for LockClient<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LOCK_CLIENT)),
            ..Default::default()
        })
    }
}
