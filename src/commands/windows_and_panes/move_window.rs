use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux move-window [-abrdk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux move-window [-ardk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux move-window [-rdk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux move-window [-dk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux move-window [-d] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MoveWindow<'a> {
    /// `[-a]` - the window is moved to the next index up
    #[cfg(feature = "tmux_2_1")]
    pub after: bool,

    /// `[-b]` - the window is moved to the next index before
    #[cfg(feature = "tmux_3_2")]
    pub before: bool,

    /// `[-r]` - all windows in the session are renumbered in sequential order
    #[cfg(feature = "tmux_1_7")]
    pub renumber: bool,

    /// `[-d]` - the newly linked window is not selected
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,

    /// `[-k]` - if dst-window exists, it is killed, otherwise an error is generated
    #[cfg(feature = "tmux_1_3")]
    pub kill: bool,

    /// `[-s src-window]` - src-window
    #[cfg(feature = "tmux_0_8")]
    pub src_window: Option<Cow<'a, str>>,

    /// `[-t dst-window]` - dst-window
    #[cfg(feature = "tmux_0_8")]
    pub dst_window: Option<Cow<'a, str>>,
}

impl<'a> MoveWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - the window is moved to the next index up
    #[cfg(feature = "tmux_2_1")]
    pub fn after(mut self) -> Self {
        self.after = true;
        self
    }

    /// `[-b]` - the window is moved to the next index before
    #[cfg(feature = "tmux_3_2")]
    pub fn before(mut self) -> Self {
        self.before = true;
        self
    }

    /// `[-r]` - all windows in the session are renumbered in sequential order
    #[cfg(feature = "tmux_1_7")]
    pub fn renumber(mut self) -> Self {
        self.renumber = true;
        self
    }

    /// `[-d]` - the newly linked window is not selected
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-k]` - if dst-window exists, it is killed, otherwise an error is generated
    #[cfg(feature = "tmux_1_3")]
    pub fn kill(mut self) -> Self {
        self.kill = true;
        self
    }

    /// `[-s src-window]` - src-window
    #[cfg(feature = "tmux_0_8")]
    pub fn src_window<S: Into<Cow<'a, str>>>(mut self, src_window: S) -> Self {
        self.src_window = Some(src_window.into());
        self
    }

    /// `[-t dst-window]` - dst-window
    #[cfg(feature = "tmux_0_8")]
    pub fn dst_window<S: Into<Cow<'a, str>>>(mut self, dst_window: S) -> Self {
        self.dst_window = Some(dst_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(MOVE_WINDOW);

        // `[-a]` - the window is moved to the next index up
        #[cfg(feature = "tmux_2_1")]
        if self.after {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-b]` - the window is moved to the next index before
        #[cfg(feature = "tmux_3_2")]
        if self.before {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-r]` - all windows in the session are renumbered in sequential order
        #[cfg(feature = "tmux_1_7")]
        if self.renumber {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-d]` - the newly linked window is not selected
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-k]` - if dst-window exists, it is killed, otherwise an error is generated
        #[cfg(feature = "tmux_1_3")]
        if self.kill {
            cmd.push_flag(K_LOWERCASE_KEY);
        }

        // `[-s src-window]` - src-window
        #[cfg(feature = "tmux_0_8")]
        if let Some(src_window) = self.src_window {
            cmd.push_option(S_LOWERCASE_KEY, src_window);
        }

        // `[-t dst-window]` - dst-window
        #[cfg(feature = "tmux_0_8")]
        if let Some(dst_window) = self.dst_window {
            cmd.push_option(T_LOWERCASE_KEY, dst_window);
        }

        cmd
    }
}
