use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// # Manual
///
/// tmux ^3.0:
/// ```text
/// (removed)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux set-window-option [-aFgoqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux set-window-option [-agoqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux set-window-option [-agqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux set-window-option [-agu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux set-window-option [-gu] [-t target-window] option value
/// (alias: setw)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SetWindowOption<'a> {
    /// `[-a]` -
    #[cfg(feature = "tmux_1_0")]
    pub append: bool,

    /// `[-F]` -
    #[cfg(feature = "tmux_2_6")]
    pub format: bool,

    /// `[-g]` -
    #[cfg(feature = "tmux_0_8")]
    pub global: bool,

    /// `[-o]` -
    #[cfg(feature = "tmux_1_9")]
    pub not_overwrite: bool,

    /// `[-q]` -
    #[cfg(feature = "tmux_1_7")]
    pub quiet: bool,

    /// `[-u]` -
    #[cfg(feature = "tmux_0_8")]
    pub unset: bool,

    /// `[-t target-window]` -
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,

    /// `option`
    pub option: Option<Cow<'a, str>>,

    /// `value`
    pub value: Option<Cow<'a, str>>,
}

impl<'a> SetWindowOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` -
    #[cfg(feature = "tmux_1_0")]
    pub fn append(&mut self) -> &mut Self {
        self.append = true;
        self
    }

    /// `[-F]` -
    #[cfg(feature = "tmux_2_6")]
    pub fn format(&mut self) -> &mut Self {
        self.format = true;
        self
    }

    /// `[-g]` -
    #[cfg(feature = "tmux_0_8")]
    pub fn global(&mut self) -> &mut Self {
        self.global = true;
        self
    }

    /// `[-o]` -
    #[cfg(feature = "tmux_1_9")]
    pub fn not_overwrite(&mut self) -> &mut Self {
        self.not_overwrite = true;
        self
    }

    /// `[-q]` -
    #[cfg(feature = "tmux_1_7")]
    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = true;
        self
    }

    /// `[-u]` -
    #[cfg(feature = "tmux_0_8")]
    pub fn unset(&mut self) -> &mut Self {
        self.unset = true;
        self
    }

    /// `[-t target-window]` -
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `option`
    pub fn option<S: Into<Cow<'a, str>>>(&mut self, option: S) -> &mut Self {
        self.option = Some(option.into());
        self
    }

    /// `value`
    pub fn value<S: Into<Cow<'a, str>>>(&mut self, value: S) -> &mut Self {
        self.value = Some(value.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SET_WINDOW_OPTION);

        // `[-a]` -
        #[cfg(feature = "tmux_1_0")]
        if self.append {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-F]` -
        #[cfg(feature = "tmux_2_6")]
        if self.format {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-g]` -
        #[cfg(feature = "tmux_0_8")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-o]` -
        #[cfg(feature = "tmux_1_9")]
        if self.not_overwrite {
            cmd.push_flag(O_LOWERCASE_KEY);
        }

        // `[-q]` -
        #[cfg(feature = "tmux_1_7")]
        if self.quiet {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-u]` -
        #[cfg(feature = "tmux_0_8")]
        if self.unset {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-t target-window]` -
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = &self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window.as_ref());
        }

        // `option`
        if let Some(option) = &self.option {
            cmd.push_param(option.as_ref());
        }

        // `value`
        if let Some(value) = &self.value {
            cmd.push_param(value.as_ref());
        }

        cmd
    }
}
