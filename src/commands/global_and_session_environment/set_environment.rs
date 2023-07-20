use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type SetEnv<'a> = SetEnvironment<'a>;

/// Structure for setting or unsetting an environment variable
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// set-environment [-Fhgru] [-t target-session] name [value]
/// (alias: setenv)
/// ```
///
/// tmux ^1.0:
/// ```text
/// set-environment [-gru] [-t target-session] name [value]
/// (alias: setenv)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SetEnvironment<'a> {
    /// `[-F]` - value is expanded as a format
    #[cfg(feature = "tmux_3_2")]
    pub expand: bool,

    /// `[-h]` - marks the variable as hidden
    #[cfg(feature = "tmux_3_2")]
    pub hidden: bool,

    /// `[-g]` - make change in the global environment
    #[cfg(feature = "tmux_1_0")]
    pub global: bool,

    /// `[-r]` - remove the variable from the environment before starting a new process
    #[cfg(feature = "tmux_1_0")]
    pub remove: bool,

    /// `[-u]` - unset a variable
    #[cfg(feature = "tmux_1_0")]
    pub unset: bool,

    /// `[-t target-session]` - target-session
    #[cfg(feature = "tmux_1_0")]
    pub target_session: Option<Cow<'a, str>>,

    /// `name`
    #[cfg(feature = "tmux_1_0")]
    pub name: Option<Cow<'a, str>>,

    /// `[value]` - specify the value
    #[cfg(feature = "tmux_1_0")]
    pub value: Option<Cow<'a, str>>,
}

impl<'a> SetEnvironment<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F]` - value is expanded as a format
    #[cfg(feature = "tmux_3_2")]
    pub fn expand(mut self) -> Self {
        self.expand = true;
        self
    }

    /// `[-h]` - marks the variable as hidden
    #[cfg(feature = "tmux_3_2")]
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// `[-g]` - make change in the global environment
    #[cfg(feature = "tmux_1_0")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-r]` - remove the variable from the environment before starting a new process
    #[cfg(feature = "tmux_1_0")]
    pub fn remove(mut self) -> Self {
        self.remove = true;
        self
    }

    /// `[-u]` - unset a variable
    #[cfg(feature = "tmux_1_0")]
    pub fn unset(mut self) -> Self {
        self.unset = true;
        self
    }

    /// `[-t target-session]` - target-session
    #[cfg(feature = "tmux_1_0")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `name`
    #[cfg(feature = "tmux_1_0")]
    pub fn name<S: Into<Cow<'a, str>>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// `[value]` - specify the value
    #[cfg(feature = "tmux_1_0")]
    pub fn value<S: Into<Cow<'a, str>>>(mut self, value: S) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SET_ENVIRONMENT);

        // `[-F]` - value is expanded as a format
        #[cfg(feature = "tmux_3_2")]
        if self.expand {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-h]` - marks the variable as hidden
        #[cfg(feature = "tmux_3_2")]
        if self.hidden {
            cmd.push_flag(H_LOWERCASE_KEY);
        }

        // `[-g]` - make change in the global environment
        #[cfg(feature = "tmux_1_0")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-r]` - remove the variable from the environment before starting a new process
        #[cfg(feature = "tmux_1_0")]
        if self.remove {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-u]` - unset a variable
        #[cfg(feature = "tmux_1_0")]
        if self.unset {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-t target-session]` - target-session
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // `name`
        #[cfg(feature = "tmux_1_0")]
        if let Some(name) = self.name {
            cmd.push_param(name);
        }

        // `[value]` - specify the value
        #[cfg(feature = "tmux_1_0")]
        if let Some(value) = self.value {
            cmd.push_param(value);
        }

        cmd
    }
}
