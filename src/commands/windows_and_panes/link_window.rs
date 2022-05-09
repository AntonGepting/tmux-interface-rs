use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Link the window at src-window to the specified dst-window
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux link-window [-adk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux link-window [-dk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
#[derive(Debug, Default, Clone)]
pub struct LinkWindow<'a> {
    /// `[-a]` - the window is moved to the next index up
    #[cfg(feature = "tmux_2_1")]
    pub add: bool,

    /// `[-d]` - the newly linked window is not selected
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,

    /// `[-k]` - if dst-window exists, it is killed, otherwise an error is generated
    #[cfg(feature = "tmux_0_8")]
    pub kill: bool,

    /// `[-s src-window]` - src-window
    #[cfg(feature = "tmux_0_8")]
    pub src_window: Option<Cow<'a, str>>,

    /// `[-t dst-window]` - dst-window
    #[cfg(feature = "tmux_0_8")]
    pub dst_window: Option<Cow<'a, str>>,
}

impl<'a> LinkWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - the window is moved to the next index up
    #[cfg(feature = "tmux_2_1")]
    pub fn add(&mut self) -> &mut Self {
        self.add = true;
        self
    }

    /// `[-d]` - the newly linked window is not selected
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = true;
        self
    }

    /// `[-k]` - if dst-window exists, it is killed, otherwise an error is generated
    #[cfg(feature = "tmux_0_8")]
    pub fn kill(&mut self) -> &mut Self {
        self.kill = true;
        self
    }

    /// `[-s src-window]` - src-window
    #[cfg(feature = "tmux_0_8")]
    pub fn src_window<S: Into<Cow<'a, str>>>(&mut self, src_window: S) -> &mut Self {
        self.src_window = Some(src_window.into());
        self
    }

    /// `[-t dst-window]` - dst-window
    #[cfg(feature = "tmux_0_8")]
    pub fn dst_window<S: Into<Cow<'a, str>>>(&mut self, dst_window: S) -> &mut Self {
        self.src_window = Some(dst_window.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(LINK_WINDOW);

        // `[-a]` - the window is moved to the next index up
        #[cfg(feature = "tmux_2_1")]
        if self.add {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-d]` - the newly linked window is not selected
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-k]` - if dst-window exists, it is killed, otherwise an error is generated
        #[cfg(feature = "tmux_0_8")]
        if self.kill {
            cmd.push_flag(K_LOWERCASE_KEY);
        }

        // `[-s src-window]` - src-window
        #[cfg(feature = "tmux_0_8")]
        if let Some(src_window) = &self.src_window {
            cmd.push_option(S_LOWERCASE_KEY, src_window.as_ref());
        }

        // `[-t dst-window]` - dst-window
        #[cfg(feature = "tmux_0_8")]
        if let Some(dst_window) = &self.dst_window {
            cmd.push_option(T_LOWERCASE_KEY, dst_window.as_ref());
        }

        cmd
    }
}
