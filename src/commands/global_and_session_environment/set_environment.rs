// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type SetEnv<'a> = SetEnvironment<'a>;

/// Set or unset an environment variable
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// set-environment [-Fhgru] [-t target-session] name [value]
/// (alias: setenv)
/// ```
///
/// tmux >=1.0:
/// ```text
/// set-environment [-gru] [-t target-session] name [value]
/// (alias: setenv)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SetEnvironment<'a> {
    /// `[-F]`
    #[cfg(feature = "tmux_3_2")]
    pub expand: bool,

    /// `[-h]`
    #[cfg(feature = "tmux_3_2")]
    pub hidden: bool,

    /// `[-g]`
    #[cfg(feature = "tmux_1_5")]
    pub global: bool,

    /// `[-r]`
    #[cfg(feature = "tmux_1_5")]
    pub remove: bool,

    /// `[-u]`
    #[cfg(feature = "tmux_1_5")]
    pub unset: bool,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    pub target_session: Option<Cow<'a, str>>,

    /// `[name]`
    #[cfg(feature = "tmux_1_5")]
    pub name: Option<Cow<'a, str>>,

    /// `[value]`
    #[cfg(feature = "tmux_1_5")]
    pub value: Option<Cow<'a, str>>,
}

impl<'a> SetEnvironment<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F]`
    #[cfg(feature = "tmux_3_2")]
    pub fn expand(mut self) -> Self {
        self.expand = true;
        self
    }

    /// `[-h]`
    #[cfg(feature = "tmux_3_2")]
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// `[-g]`
    #[cfg(feature = "tmux_1_5")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-r]`
    #[cfg(feature = "tmux_1_5")]
    pub fn remove(mut self) -> Self {
        self.remove = true;
        self
    }

    /// `[-u]`
    #[cfg(feature = "tmux_1_5")]
    pub fn unset(mut self) -> Self {
        self.unset = true;
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[name]`
    #[cfg(feature = "tmux_1_5")]
    pub fn name<S: Into<Cow<'a, str>>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// `[value]`
    #[cfg(feature = "tmux_1_5")]
    pub fn value<S: Into<Cow<'a, str>>>(mut self, value: S) -> Self {
        self.value = Some(value.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SET_ENVIRONMENT);

        // `[-F]`
        #[cfg(feature = "tmux_3_2")]
        if self.expand {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-h]`
        #[cfg(feature = "tmux_3_2")]
        if self.hidden {
            cmd.push_flag(H_LOWERCASE_KEY);
        }

        // `[-g]`
        #[cfg(feature = "tmux_1_5")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-r]`
        #[cfg(feature = "tmux_1_5")]
        if self.remove {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-u]`
        #[cfg(feature = "tmux_1_5")]
        if self.unset {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // `[name]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(name) = self.name {
            cmd.push_param(name);
        }

        // `[value]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(value) = self.value {
            cmd.push_param(value);
        }

        cmd
    }
}
