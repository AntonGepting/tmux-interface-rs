use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type LsW<'a> = ListWindows<'a>;

// XXX: better return type
/// List windows on the server
///
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// list-windows [-a] [-F format] [-t target-session]
/// (alias: lsw)
/// ```
///
/// tmux ^1.5:
/// ```text
/// list-windows [-a] [-t target-session]
/// (alias: lsw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// list-windows [-t target-session]
/// (alias: lsw)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListWindows<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_1_5")]
    pub all: bool,

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub format: Option<Cow<'a, str>>,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> ListWindows<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_5")]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LIST_WINDOWS);

        // `[-a]`
        #[cfg(feature = "tmux_1_5")]
        if self.all {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-F format]`
        #[cfg(feature = "tmux_1_6")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
