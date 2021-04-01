use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for setting a pane/window/session/server option
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux set-option [-aFgopqsuw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux set-option [-agoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux set-option [-agqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux set-option [-agsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux set-option [-agu] [-t target-session] option value
/// (alias: set)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux set-option [-gu] [-t target-session] option value
/// (alias: set)
/// ```

#[derive(Debug, Clone)]
pub struct SetOption<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SetOption<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SET_OPTION)),
            ..Default::default()
        })
    }
}

impl<'a> SetOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - value is appended to the existing setting, if the option expects a string or a style
    #[cfg(feature = "tmux_1_0")]
    pub fn append(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-F]` - expand formats in the option value
    #[cfg(feature = "tmux_2_6")]
    pub fn format(&mut self) -> &mut Self {
        self.0.push_flag(F_UPPERCASE_KEY);
        self
    }

    /// `[-g]` - the global session or window option is set
    #[cfg(feature = "tmux_0_8")]
    pub fn global(&mut self) -> &mut Self {
        self.0.push_flag(G_LOWERCASE_KEY);
        self
    }

    /// `[-o]` - prevents setting an option that is already set
    #[cfg(feature = "tmux_1_8")]
    pub fn not_overwrite(&mut self) -> &mut Self {
        self.0.push_flag(O_LOWERCASE_KEY);
        self
    }

    /// `[-p]` - set a pane option
    #[cfg(feature = "tmux_3_0")]
    pub fn pane(&mut self) -> &mut Self {
        self.0.push_flag(P_LOWERCASE_KEY);
        self
    }

    /// `[-q]` - suppress errors about unknown or ambiguous options
    #[cfg(feature = "tmux_1_7")]
    pub fn quiet(&mut self) -> &mut Self {
        self.0.push_flag(Q_LOWERCASE_KEY);
        self
    }

    /// `[-s]` - set a server option
    #[cfg(feature = "tmux_1_2")]
    pub fn server(&mut self) -> &mut Self {
        self.0.push_flag(S_LOWERCASE_KEY);
        self
    }

    /// `[-u]` - unset an option, so a session inherits the option from the global options
    #[cfg(feature = "tmux_0_8")]
    pub fn unset(&mut self) -> &mut Self {
        self.0.push_flag(U_LOWERCASE_KEY);
        self
    }

    /// `[-w]` - set a window option
    #[cfg(feature = "tmux_1_2")]
    pub fn window(&mut self) -> &mut Self {
        self.0.push_flag(W_LOWERCASE_KEY);
        self
    }

    /// `[-t target-pane]` - specify the target-pane
    #[cfg(feature = "tmux_3_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-t target-session | target-window]`
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub fn target(&mut self, target: &'a str) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target);
        self
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    pub fn target_session(&mut self, target_session: &'a str) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    // FIXME: option valuer pair in one fn

    /// `option`
    pub fn option<S: Into<Cow<'a, str>>>(&mut self, option: S) -> &mut Self {
        self.0.push_param(option);
        self
    }

    /// `value`
    pub fn value<S: Into<Cow<'a, str>>>(&mut self, value: S) -> &mut Self {
        self.0.push_param(value);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SetOption<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SET_OPTION)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SetOption<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SET_OPTION)),
            ..Default::default()
        })
    }
}
