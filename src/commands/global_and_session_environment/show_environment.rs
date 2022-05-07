use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// # Manual
///
/// tmux ^3.2:
/// ```text:
/// tmux show-environment [-hgs] [-t target-session] [variable]
/// (alias: showenv)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux show-environment [-gs] [-t target-session] [variable]
/// (alias: showenv)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux show-environment [-g] [-t target-session] [variable]
/// (alias: showenv)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux show-environment [-g] [-t target-session]
/// (alias: showenv)
/// ```
#[derive(Debug, Default, Clone)]
pub struct ShowEnvironment<'a> {
    /// `[-h]`
    #[cfg(feature = "tmux_3_2")]
    pub hidden: bool,

    /// `[-g]`
    #[cfg(feature = "tmux_1_0")]
    pub global: bool,

    /// `[-s]`
    #[cfg(feature = "tmux_2_1")]
    pub as_shell_commands: bool,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_0")]
    pub target_session: Option<Cow<'a, str>>,

    /// `[variable]`
    #[cfg(feature = "tmux_1_7")]
    pub variable: Option<Cow<'a, str>>,
}

impl<'a> ShowEnvironment<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-h]`
    #[cfg(feature = "tmux_3_2")]
    pub fn hidden(&mut self) -> &mut Self {
        self.hidden = true;
        self
    }

    /// `[-g]`
    #[cfg(feature = "tmux_1_0")]
    pub fn global(&mut self) -> &mut Self {
        self.global = true;
        self
    }

    /// `[-s]`
    #[cfg(feature = "tmux_2_1")]
    pub fn as_shell_commands(&mut self) -> &mut Self {
        self.as_shell_commands = true;
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[variable]`
    #[cfg(feature = "tmux_1_7")]
    pub fn variable<S: Into<Cow<'a, str>>>(&mut self, variable: S) -> &mut Self {
        self.variable = Some(variable.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SHOW_ENVIRONMENT);

        // `[-h]`
        #[cfg(feature = "tmux_3_2")]
        if self.hidden {
            cmd.push_flag(H_LOWERCASE_KEY);
        }

        // `[-g]`
        #[cfg(feature = "tmux_1_0")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-s]`
        #[cfg(feature = "tmux_2_1")]
        if self.as_shell_commands {
            cmd.push_flag(S_LOWERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_session) = &self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session.as_ref());
        }

        // `[variable]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(variable) = &self.variable {
            cmd.push_param(variable.as_ref());
        }

        cmd
    }
}
