use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;
use std::marker::PhantomData;

pub type LsB<'a> = ListBuffers<'a>;

/// List the global buffers.
///
/// # Manual
///
/// tmux ^1.7:
/// ```text
/// list-buffers [-F format]
/// (alias: lsb)
/// ```
///
/// tmux ^1.5:
/// ```text
/// list-buffers
/// (alias: lsb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// list-buffers [-t target-session]
/// (alias: lsb)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListBuffers<'a> {
    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_session: Option<Cow<'a, str>>,
    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<Cow<'a, str>>,

    _phantom_data: PhantomData<&'a ()>,
}

impl<'a> ListBuffers<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LIST_BUFFERS);

        // `[-t target-session]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // `[-F format]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        cmd
    }
}
