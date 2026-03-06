// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Set<'a> = SetOption<'a>;

/// Set a pane/window/session/server option
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// set-option [-aFgopqsuUw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux >=3.0a:
/// ```text
/// set-option [-aFgopqsuw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux >=2.6:
/// ```text
/// set-option [-aFgoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux >=1.8:
/// ```text
/// set-option [-agoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux >=1.7:
/// ```text
/// set-option [-agqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux >=1.5:
/// ```text
/// set-option [-agsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux >=0.8:
/// ```text
/// set-option [-gu] [-t target-session] option value
/// (alias: set)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SetOption<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_1_5")]
    pub append: bool,

    /// `[-F]`
    #[cfg(feature = "tmux_2_6")]
    pub format: bool,

    /// `[-g]`
    #[cfg(feature = "tmux_0_8")]
    pub global: bool,

    /// `[-o]`
    #[cfg(feature = "tmux_1_8")]
    pub not_overwrite: bool,

    /// `[-p]`
    #[cfg(feature = "tmux_3_0a")]
    pub pane: bool,

    /// `[-q]`
    #[cfg(feature = "tmux_1_7")]
    pub quiet: bool,

    /// `[-s]`
    #[cfg(feature = "tmux_1_5")]
    pub server: bool,

    /// `[-u]`
    #[cfg(feature = "tmux_0_8")]
    pub unset: bool,

    /// `[-U]`
    #[cfg(feature = "tmux_3_2")]
    pub unset_on_all: bool,

    /// `[-w]`
    #[cfg(feature = "tmux_1_5")]
    pub window: bool,

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_session: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_0a")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_0a")]
    pub target_pane: Option<Cow<'a, str>>,

    // FIXME: option value pair in one fn
    /// `[option]`
    #[cfg(feature = "tmux_0_8")]
    pub option: Option<Cow<'a, str>>,

    /// `[value]`
    #[cfg(feature = "tmux_0_8")]
    pub value: Option<Cow<'a, str>>,
}

impl<'a> SetOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_5")]
    pub fn append(mut self) -> Self {
        self.append = true;
        self
    }

    /// `[-F]`
    #[cfg(feature = "tmux_2_6")]
    pub fn format(mut self) -> Self {
        self.format = true;
        self
    }

    /// `[-g]`
    #[cfg(feature = "tmux_0_8")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-o]`
    #[cfg(feature = "tmux_1_8")]
    pub fn not_overwrite(mut self) -> Self {
        self.not_overwrite = true;
        self
    }

    /// `[-p]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn pane(mut self) -> Self {
        self.pane = true;
        self
    }

    /// `[-q]`
    #[cfg(feature = "tmux_1_7")]
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

    /// `[-u]`
    #[cfg(feature = "tmux_0_8")]
    pub fn unset(mut self) -> Self {
        self.unset = true;
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_3_2")]
    pub fn unset_on_all(mut self) -> Self {
        self.unset_on_all = true;
        self
    }

    /// `[-w]`
    #[cfg(feature = "tmux_1_5")]
    pub fn window(mut self) -> Self {
        self.window = true;
        self
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_0a")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
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

    /// `[value]`
    #[cfg(feature = "tmux_0_8")]
    pub fn value<S: Into<Cow<'a, str>>>(mut self, value: S) -> Self {
        self.value = Some(value.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SET_OPTION);

        // `[-a]`
        #[cfg(feature = "tmux_1_5")]
        if self.append {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-F]`
        #[cfg(feature = "tmux_2_6")]
        if self.format {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-g]`
        #[cfg(feature = "tmux_0_8")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-o]`
        #[cfg(feature = "tmux_1_8")]
        if self.not_overwrite {
            cmd.push_flag(O_LOWERCASE_KEY);
        }

        // `[-p]`
        #[cfg(feature = "tmux_3_0a")]
        if self.pane {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-q]`
        #[cfg(feature = "tmux_1_7")]
        if self.quiet {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-s]`
        #[cfg(feature = "tmux_1_5")]
        if self.server {
            cmd.push_flag(S_LOWERCASE_KEY);
        }

        // `[-u]`
        #[cfg(feature = "tmux_0_8")]
        if self.unset {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-U]`
        #[cfg(feature = "tmux_3_2")]
        if self.unset_on_all {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-w]`
        #[cfg(feature = "tmux_1_5")]
        if self.window {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_0a")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
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

        // `[value]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(value) = self.value {
            cmd.push_param(value);
        }

        cmd
    }
}
