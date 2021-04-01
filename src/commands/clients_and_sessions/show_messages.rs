use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
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
#[derive(Debug, Clone)]
pub struct ShowMessages<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ShowMessages<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SHOW_MESSAGES)),
            ..Default::default()
        })
    }
}

impl<'a> ShowMessages<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-I]`
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    pub fn server(&mut self) -> &mut Self {
        self.0.push_flag(I_UPPERCASE_KEY);
        self
    }

    /// `[-J]`
    #[cfg(feature = "tmux_1_9")]
    pub fn jobs(&mut self) -> &mut Self {
        self.0.push_flag(J_UPPERCASE_KEY);
        self
    }

    /// `[-T]`
    #[cfg(feature = "tmux_1_9")]
    pub fn terminals(&mut self) -> &mut Self {
        self.0.push_flag(T_UPPERCASE_KEY);
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_1_2")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_client);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ShowMessages<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SHOW_MESSAGES)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ShowMessages<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SHOW_MESSAGES)),
            ..Default::default()
        })
    }
}
