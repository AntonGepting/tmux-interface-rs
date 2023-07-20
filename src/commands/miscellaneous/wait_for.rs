use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Wait<'a> = WaitFor<'a>;

// TODO: enum for arg
// FIXME: not multiple, only one choice
/// # Manual
///
/// tmux ^1.9:
/// ```text
/// wait-for [-L | -S | -U] channel
/// (alias: wait)
/// ```
///
/// tmux ^1.8:
/// ```text
/// wait-for -LSU channel
/// (alias: wait)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct WaitFor<'a> {
    /// `[-L]`
    #[cfg(feature = "tmux_1_8")]
    pub locked: bool,

    /// `[-S]`
    #[cfg(feature = "tmux_1_8")]
    pub woken: bool,

    /// `[-U]`
    #[cfg(feature = "tmux_1_8")]
    pub unlocked: bool,

    /// `channel`
    #[cfg(feature = "tmux_1_8")]
    pub channel: Option<Cow<'a, str>>,
}

impl<'a> WaitFor<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-L]`
    #[cfg(feature = "tmux_1_8")]
    pub fn locked(mut self) -> Self {
        self.locked = true;
        self
    }

    /// `[-S]`
    #[cfg(feature = "tmux_1_8")]
    pub fn woken(mut self) -> Self {
        self.woken = true;
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_1_8")]
    pub fn unlocked(mut self) -> Self {
        self.unlocked = true;
        self
    }

    /// `channel`
    #[cfg(feature = "tmux_1_8")]
    pub fn channel<S: Into<Cow<'a, str>>>(mut self, channel: S) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(WAIT_FOR);

        // `[-L]`
        #[cfg(feature = "tmux_1_8")]
        if self.locked {
            cmd.push_flag(L_UPPERCASE_KEY);
        }

        // `[-S]`
        #[cfg(feature = "tmux_1_8")]
        if self.woken {
            cmd.push_flag(S_UPPERCASE_KEY);
        }

        // `[-U]`
        #[cfg(feature = "tmux_1_8")]
        if self.unlocked {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `channel`
        #[cfg(feature = "tmux_1_8")]
        if let Some(channel) = self.channel {
            cmd.push_param(channel);
        }
        cmd
    }
}
