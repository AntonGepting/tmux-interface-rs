use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Structure for setting a pane/window/session/server option
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// set-option [-aFgopqsuUw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux ^3.0:
/// ```text
/// set-option [-aFgopqsuw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux ^2.6:
/// ```text
/// set-option [-aFgoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.8:
/// ```text
/// set-option [-agoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.7:
/// ```text
/// set-option [-agqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.2:
/// ```text
/// set-option [-agsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.0:
/// ```text
/// set-option [-agu] [-t target-session] option value
/// (alias: set)
/// ```
///
/// tmux ^0.8:
/// ```text
/// set-option [-gu] [-t target-session] option value
/// (alias: set)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SetOption<'a> {
    /// `[-a]` - value is appended to the existing setting, if the option expects a string or a style
    #[cfg(feature = "tmux_1_0")]
    pub append: bool,

    /// `[-F]` - expand formats in the option value
    #[cfg(feature = "tmux_2_6")]
    pub format: bool,

    /// `[-g]` - the global session or window option is set
    #[cfg(feature = "tmux_0_8")]
    pub global: bool,

    /// `[-o]` - prevents setting an option that is already set
    #[cfg(feature = "tmux_1_8")]
    pub not_overwrite: bool,

    /// `[-p]` - set a pane option
    #[cfg(feature = "tmux_3_0")]
    pub pane: bool,

    /// `[-q]` - suppress errors about unknown or ambiguous options
    #[cfg(feature = "tmux_1_7")]
    pub quiet: bool,

    /// `[-s]` - set a server option
    #[cfg(feature = "tmux_1_2")]
    pub server: bool,

    /// `[-u]` - unset an option, so a session inherits the option from the global options
    #[cfg(feature = "tmux_0_8")]
    pub unset: bool,

    /// `[-U]` - unsets an option (like -u) but if the option is a pane option also unsets the option on any panes in the window
    #[cfg(feature = "tmux_3_2")]
    pub unset_on_all: bool,

    /// `[-w]` - set a window option
    #[cfg(feature = "tmux_1_2")]
    pub window: bool,

    /// `[-t target-pane]` - specify the target-pane
    #[cfg(feature = "tmux_3_0")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-t target-session | target-window]`
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub target: Option<Cow<'a, str>>,

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    pub target_session: Option<Cow<'a, str>>,

    // FIXME: option valuer pair in one fn
    /// `option`
    pub option: Option<Cow<'a, str>>,

    /// `value`
    pub value: Option<Cow<'a, str>>,
}

impl<'a> SetOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - value is appended to the existing setting, if the option expects a string or a style
    #[cfg(feature = "tmux_1_0")]
    pub fn append(mut self) -> Self {
        self.append = true;
        self
    }

    /// `[-F]` - expand formats in the option value
    #[cfg(feature = "tmux_2_6")]
    pub fn format(mut self) -> Self {
        self.format = true;
        self
    }

    /// `[-g]` - the global session or window option is set
    #[cfg(feature = "tmux_0_8")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-o]` - prevents setting an option that is already set
    #[cfg(feature = "tmux_1_8")]
    pub fn not_overwrite(mut self) -> Self {
        self.not_overwrite = true;
        self
    }

    /// `[-p]` - set a pane option
    #[cfg(feature = "tmux_3_0")]
    pub fn pane(mut self) -> Self {
        self.pane = true;
        self
    }

    /// `[-q]` - suppress errors about unknown or ambiguous options
    #[cfg(feature = "tmux_1_7")]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// `[-s]` - set a server option
    #[cfg(feature = "tmux_1_2")]
    pub fn server(mut self) -> Self {
        self.server = true;
        self
    }

    /// `[-u]` - unset an option, so a session inherits the option from the global options
    #[cfg(feature = "tmux_0_8")]
    pub fn unset(mut self) -> Self {
        self.unset = true;
        self
    }

    /// `[-U]` - unsets an option (like -u) but if the option is a pane option also unsets the option on any panes in the window
    #[cfg(feature = "tmux_3_2")]
    pub fn unset_on_all(mut self) -> Self {
        self.unset_on_all = true;
        self
    }

    /// `[-w]` - set a window option
    #[cfg(feature = "tmux_1_2")]
    pub fn window(mut self) -> Self {
        self.window = true;
        self
    }

    /// `[-t target-pane]` - specify the target-pane
    #[cfg(feature = "tmux_3_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-t target-session | target-window]`
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub fn target(mut self, target: &'a str) -> Self {
        self.target = Some(target.into());
        self
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    pub fn target_session(mut self, target_session: &'a str) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    // FIXME: option valuer pair in one fn

    /// `option`
    pub fn option<S: Into<Cow<'a, str>>>(mut self, option: S) -> Self {
        self.option = Some(option.into());
        self
    }

    /// `value`
    pub fn value<S: Into<Cow<'a, str>>>(mut self, value: S) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SET_OPTION);

        // `[-a]` - value is appended to the existing setting, if the option expects a string or a style
        #[cfg(feature = "tmux_1_0")]
        if self.append {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-F]` - expand formats in the option value
        #[cfg(feature = "tmux_2_6")]
        if self.format {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-g]` - the global session or window option is set
        #[cfg(feature = "tmux_0_8")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-o]` - prevents setting an option that is already set
        #[cfg(feature = "tmux_1_8")]
        if self.not_overwrite {
            cmd.push_flag(O_LOWERCASE_KEY);
        }

        // `[-p]` - set a pane option
        #[cfg(feature = "tmux_3_0")]
        if self.pane {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-q]` - suppress errors about unknown or ambiguous options
        #[cfg(feature = "tmux_1_7")]
        if self.quiet {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-s]` - set a server option
        #[cfg(feature = "tmux_1_2")]
        if self.server {
            cmd.push_flag(S_LOWERCASE_KEY);
        }

        // `[-u]` - unset an option, so a session inherits the option from the global options
        #[cfg(feature = "tmux_0_8")]
        if self.unset {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-U]` - unsets an option (like -u) but if the option is a pane option also unsets the option on any panes in the window
        #[cfg(feature = "tmux_3_2")]
        if self.unset_on_all {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-w]` - set a window option
        #[cfg(feature = "tmux_1_2")]
        if self.window {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[-t target-pane]` - specify the target-pane
        #[cfg(feature = "tmux_3_0")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-t target-session | target-window]`
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
        if let Some(target) = self.target {
            cmd.push_option(T_LOWERCASE_KEY, target);
        }

        // `[-t target-session]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // FIXME: option valuer pair in one fn

        // `option`
        if let Some(option) = self.option {
            cmd.push_param(option);
        }

        // `value`
        if let Some(value) = self.value {
            cmd.push_param(value);
        }

        cmd
    }
}
