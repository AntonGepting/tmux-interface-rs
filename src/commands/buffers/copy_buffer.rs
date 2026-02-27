// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type CopyB<'a> = CopyBuffer<'a>;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CopyBuffer<'a> {
    /// `[-a src-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub src_index: Option<Cow<'a, str>>,

    /// `[-b dst-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub dst_index: Option<Cow<'a, str>>,

    /// `[-s src-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub src_session: Option<Cow<'a, str>>,

    /// `[-t dst-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub dst_session: Option<Cow<'a, str>>,
}

impl<'a> CopyBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a src-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn src_index<S: Into<Cow<'a, str>>>(mut self, src_index: S) -> Self {
        self.src_index = Some(src_index.into());
        self
    }

    /// `[-b dst-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn dst_index<S: Into<Cow<'a, str>>>(mut self, dst_index: S) -> Self {
        self.dst_index = Some(dst_index.into());
        self
    }

    /// `[-s src-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn src_session<S: Into<Cow<'a, str>>>(mut self, src_session: S) -> Self {
        self.src_session = Some(src_session.into());
        self
    }

    /// `[-t dst-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn dst_session<S: Into<Cow<'a, str>>>(mut self, dst_session: S) -> Self {
        self.dst_session = Some(dst_session.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(COPY_BUFFER);

        // `[-a src-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(src_index) = self.src_index {
            cmd.push_option(A_LOWERCASE_KEY, src_index);
        }

        // `[-b dst-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(dst_index) = self.dst_index {
            cmd.push_option(B_LOWERCASE_KEY, dst_index);
        }

        // `[-s src-session]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(src_session) = self.src_session {
            cmd.push_option(S_LOWERCASE_KEY, src_session);
        }

        // `[-t dst-session]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(dst_session) = self.dst_session {
            cmd.push_option(T_LOWERCASE_KEY, dst_session);
        }

        cmd
    }
}
