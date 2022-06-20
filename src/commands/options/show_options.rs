use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

//use crate::commands::tmux_commands::TmuxCommands;

/// Structure for showing options
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux show-options [-gsw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux show-options [-gsw] [-t target-session | target-window]
/// (alias: show)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux show-options [-t target-session]
/// (alias: show)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux show-options [-t target-session] option value
/// (alias: show)
/// ```
// XXX: better result type?
#[derive(Debug, Default, Clone)]
pub struct ShowOptions<'a> {
    /// `[-A]` - includes options inherited from a parent set of options
    #[cfg(feature = "tmux_3_0")]
    pub include_inherited: bool,

    /// `[-g]` - global session or window options are listed
    #[cfg(feature = "tmux_1_2")]
    pub global: bool,

    /// `[-H]` - includes hooks (omitted by default)
    #[cfg(feature = "tmux_3_0")]
    pub hooks: bool,

    /// `[-p]` - show window options
    #[cfg(feature = "tmux_3_0")]
    pub pane: bool,

    /// `[-q]` - no error will be returned if `option` is unset
    #[cfg(feature = "tmux_1_8")]
    pub quiet: bool,

    /// `[-s]` - show the server options
    #[cfg(feature = "tmux_1_2")]
    pub server: bool,

    /// `[-v]` - shows only the option value
    #[cfg(feature = "tmux_1_8")]
    pub value: bool,

    /// `[-w]` - show the window options
    #[cfg(feature = "tmux_1_2")]
    pub window: bool,

    /// `[-t target-pane]` - target session or window name
    //#[cfg(feature = "tmux_X_X")]
    pub target: Option<Cow<'a, str>>,

    /// `[option]` - specify option name
    #[cfg(feature = "tmux_1_7")]
    pub option: Option<Cow<'a, str>>,
}

impl<'a> ShowOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-A]` - includes options inherited from a parent set of options
    #[cfg(feature = "tmux_3_0")]
    pub fn include_inherited(mut self) -> Self {
        self.include_inherited = true;
        self
    }

    /// `[-g]` - global session or window options are listed
    #[cfg(feature = "tmux_1_2")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-H]` - includes hooks (omitted by default)
    #[cfg(feature = "tmux_3_0")]
    pub fn hooks(mut self) -> Self {
        self.hooks = true;
        self
    }

    /// `[-p]` - show window options
    #[cfg(feature = "tmux_3_0")]
    pub fn pane(mut self) -> Self {
        self.pane = true;
        self
    }

    /// `[-q]` - no error will be returned if `option` is unset
    #[cfg(feature = "tmux_1_8")]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// `[-s]` - show the server options
    #[cfg(feature = "tmux_1_2")]
    pub fn server(mut self) -> Self {
        self.server = true;
        self
    }

    /// `[-v]` - shows only the option value
    #[cfg(feature = "tmux_1_8")]
    pub fn value(mut self) -> Self {
        self.value = true;
        self
    }

    /// `[-w]` - show the window options
    #[cfg(feature = "tmux_1_2")]
    pub fn window(mut self) -> Self {
        self.window = true;
        self
    }

    /// `[-t target-pane]` - target session or window name
    //#[cfg(feature = "tmux_X_X")]
    pub fn target<S: Into<Cow<'a, str>>>(mut self, target: S) -> Self {
        self.target = Some(target.into());
        self
    }

    /// `[option]` - specify option name
    #[cfg(feature = "tmux_1_7")]
    pub fn option<S: Into<Cow<'a, str>>>(mut self, option: S) -> Self {
        self.option = Some(option.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SHOW_OPTIONS);

        // `[-A]` - includes options inherited from a parent set of options
        #[cfg(feature = "tmux_3_0")]
        if self.include_inherited {
            cmd.push_flag(A_UPPERCASE_KEY);
        }

        // `[-g]` - global session or window options are listed
        #[cfg(feature = "tmux_1_2")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-H]` - includes hooks (omitted by default)
        #[cfg(feature = "tmux_3_0")]
        if self.hooks {
            cmd.push_flag(H_UPPERCASE_KEY);
        }

        // `[-p]` - show window options
        #[cfg(feature = "tmux_3_0")]
        if self.pane {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-q]` - no error will be returned if `option` is unset
        #[cfg(feature = "tmux_1_8")]
        if self.quiet {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-s]` - show the server options
        #[cfg(feature = "tmux_1_2")]
        if self.server {
            cmd.push_flag(S_LOWERCASE_KEY);
        }

        // `[-v]` - shows only the option value
        #[cfg(feature = "tmux_1_8")]
        if self.value {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-w]` - show the window options
        #[cfg(feature = "tmux_1_2")]
        if self.window {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[-t target-pane]` - target session or window name
        //#[cfg(feature = "tmux_X_X")]
        if let Some(target) = self.target {
            cmd.push_option(T_LOWERCASE_KEY, target);
        }

        // `[option]` - specify option name
        #[cfg(feature = "tmux_1_7")]
        if let Some(option) = self.option {
            cmd.push_param(option);
        }

        cmd
    }
}
