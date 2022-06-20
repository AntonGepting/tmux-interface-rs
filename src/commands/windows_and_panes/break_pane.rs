use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux break-pane [-abdP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.2:
/// ```text
/// tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
/// (alias: breakp)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux break-pane [-dP] [-F format] [-t target-pane]
/// (alias: breakp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux break-pane [-d] [-t target-window]
/// (alias: breakp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux break-pane [-d] [-p pane-index] [-t target-window]
/// (alias: breakp)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BreakPane<'a> {
    /// `[-a]` - the window is moved to the next index after
    #[cfg(feature = "tmux_3_2")]
    pub after: bool,

    /// `[-b]` - the window is moved to the next index before
    #[cfg(feature = "tmux_3_2")]
    pub before: bool,

    /// `[-d]` - the new window does not become the current window
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,

    /// `[-P]` - option prints information about the new window after it has been created
    #[cfg(feature = "tmux_1_7")]
    pub print: bool,

    /// `[-F format]` - specify format
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<Cow<'a, str>>,

    /// `[-n window-name]` - window-name
    #[cfg(feature = "tmux_2_4")]
    pub window_name: Option<Cow<'a, str>>,

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_2_1")]
    pub src_pane: Option<Cow<'a, str>>,

    /// `[-t dst-pane]` - dst-pane
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub dst_pane: Option<Cow<'a, str>>,

    /// `[-t dst-window]` - dst-window
    #[cfg(feature = "tmux_2_2")]
    pub dst_window: Option<Cow<'a, str>>,

    /// `[-t target-window]` - target-window
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - target-pane
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub target_pane: Option<Cow<'a, str>>,
    // FIXME:
    // `[-p pane-index]` - pane-index
}

impl<'a> BreakPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - the window is moved to the next index after
    #[cfg(feature = "tmux_3_2")]
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

    /// `[-d]` - the new window does not become the current window
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-P]` - option prints information about the new window after it has been created
    #[cfg(feature = "tmux_1_7")]
    pub fn print(mut self) -> Self {
        self.print = true;
        self
    }

    /// `[-F format]` - specify format
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-n window-name]` - window-name
    #[cfg(feature = "tmux_2_4")]
    pub fn window_name<S: Into<Cow<'a, str>>>(mut self, window_name: S) -> Self {
        self.window_name = Some(window_name.into());
        self
    }

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_2_1")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(mut self, src_pane: S) -> Self {
        self.src_pane = Some(src_pane.into());
        self
    }

    /// `[-t dst-pane]` - dst-pane
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(mut self, dst_pane: S) -> Self {
        self.dst_pane = Some(dst_pane.into());
        self
    }

    /// `[-t dst-window]` - dst-window
    #[cfg(feature = "tmux_2_2")]
    pub fn dst_window<S: Into<Cow<'a, str>>>(mut self, dst_window: S) -> Self {
        self.dst_window = Some(dst_window.into());
        self
    }

    /// `[-t target-window]` - target-window
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    // FIXME:
    // `[-p pane-index]` - pane-index

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(BREAK_PANE);

        // `[-a]` - the window is moved to the next index after
        #[cfg(feature = "tmux_3_2")]
        if self.after {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-b]` - the window is moved to the next index before
        #[cfg(feature = "tmux_3_2")]
        if self.before {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-d]` - the new window does not become the current window
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-P]` - option prints information about the new window after it has been created
        #[cfg(feature = "tmux_1_7")]
        if self.print {
            cmd.push_flag(P_UPPERCASE_KEY);
        }

        // `[-F format]` - specify format
        #[cfg(feature = "tmux_1_7")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-n]` - window-name
        #[cfg(feature = "tmux_2_4")]
        if let Some(window_name) = self.window_name {
            cmd.push_option(N_LOWERCASE_KEY, window_name);
        }

        // `[-s src-pane]` - src-pane
        #[cfg(feature = "tmux_2_1")]
        if let Some(src_pane) = self.src_pane {
            cmd.push_option(S_LOWERCASE_KEY, src_pane);
        }

        // `[-t dst-pane]` - dst-pane
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        if let Some(dst_pane) = self.dst_pane {
            cmd.push_option(T_LOWERCASE_KEY, dst_pane);
        }

        // `[-t dst-window]` - dst-window
        #[cfg(feature = "tmux_2_2")]
        if let Some(dst_window) = self.dst_window {
            cmd.push_option(T_LOWERCASE_KEY, dst_window);
        }

        // `[-t target-window]` - target-window
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t target-pane]` - target-pane
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // FIXME:
        // `[-p pane-index]` - pane-index

        cmd
    }
}
