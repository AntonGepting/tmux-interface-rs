// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Show<'a> = ShowOptions<'a>;

/// Show options
///
/// # Manual
///
/// tmux >=3.0a:
/// ```text
/// show-options [-AgHpqsvw] [-t target-pane] [option]
/// (alias: show)
/// ```
///
/// tmux >=1.8:
/// ```text
/// show-options [-gqsvw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux >=1.5:
/// ```text
/// show-options [-gsw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux >=0.8:
/// ```text
/// show-options [-t target-session] option value
/// (alias: show)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ShowOptions<'a> {
    /// `[-A]`
    #[cfg(feature = "tmux_3_0a")]
    pub include_inherited: bool,

    /// `[-g]`
    #[cfg(feature = "tmux_1_5")]
    pub global: bool,

    /// `[-H]`
    #[cfg(feature = "tmux_3_0a")]
    pub hooks: bool,

    /// `[-p]`
    #[cfg(feature = "tmux_3_0a")]
    pub pane: bool,

    /// `[-q]`
    #[cfg(feature = "tmux_1_8")]
    pub quiet: bool,

    /// `[-s]`
    #[cfg(feature = "tmux_1_5")]
    pub server: bool,

    /// `[-v]`
    #[cfg(feature = "tmux_1_8")]
    pub value: bool,

    /// `[-w]`
    #[cfg(feature = "tmux_1_5")]
    pub window: bool,

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_0a")))]
    pub target_session: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_0a")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[option]`
    #[cfg(feature = "tmux_0_8")]
    pub option: Option<Cow<'a, str>>,
}

impl<'a> ShowOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-A]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn include_inherited(mut self) -> Self {
        self.include_inherited = true;
        self
    }

    /// `[-g]`
    #[cfg(feature = "tmux_1_5")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-H]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn hooks(mut self) -> Self {
        self.hooks = true;
        self
    }

    /// `[-p]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn pane(mut self) -> Self {
        self.pane = true;
        self
    }

    /// `[-q]`
    #[cfg(feature = "tmux_1_8")]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// `[-s]`
    #[cfg(feature = "tmux_1_5")]
    pub fn server(mut self) -> Self {
        self.server = true;
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_1_8")]
    pub fn value(mut self) -> Self {
        self.value = true;
        self
    }

    /// `[-w]`
    #[cfg(feature = "tmux_1_5")]
    pub fn window(mut self) -> Self {
        self.window = true;
        self
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_0a")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[option]`
    #[cfg(feature = "tmux_0_8")]
    pub fn option<S: Into<Cow<'a, str>>>(mut self, option: S) -> Self {
        self.option = Some(option.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SHOW_OPTIONS);

        // `[-A]`
        #[cfg(feature = "tmux_3_0a")]
        if self.include_inherited {
            cmd.push_flag(A_UPPERCASE_KEY);
        }

        // `[-g]`
        #[cfg(feature = "tmux_1_5")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-H]`
        #[cfg(feature = "tmux_3_0a")]
        if self.hooks {
            cmd.push_flag(H_UPPERCASE_KEY);
        }

        // `[-p]`
        #[cfg(feature = "tmux_3_0a")]
        if self.pane {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-q]`
        #[cfg(feature = "tmux_1_8")]
        if self.quiet {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-s]`
        #[cfg(feature = "tmux_1_5")]
        if self.server {
            cmd.push_flag(S_LOWERCASE_KEY);
        }

        // `[-v]`
        #[cfg(feature = "tmux_1_8")]
        if self.value {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-w]`
        #[cfg(feature = "tmux_1_5")]
        if self.window {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_0a")))]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_3_0a")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[option]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(option) = self.option {
            cmd.push_param(option);
        }

        cmd
    }
}
