use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for setting or unsetting an environment variable
///
/// # Manual
///
/// tmux ^1.0:
/// ```text
/// tmux set-environment [-gru] [-t target-session] name [value]
/// (alias: setenv)
/// ```
#[derive(Default, Debug)]
pub struct SetEnvironment<'a> {
    /// [-g] - make change in the global environment
    #[cfg(feature = "tmux_1_0")]
    pub global: Option<bool>,
    /// [-r] - remove the variable from the environment before starting a new process
    #[cfg(feature = "tmux_1_0")]
    pub remove: Option<bool>,
    /// [-u] - unset a variable
    #[cfg(feature = "tmux_1_0")]
    pub unset: Option<bool>,
    /// [-t target-session] - target-session
    #[cfg(feature = "tmux_1_0")]
    pub target_session: Option<&'a str>,
    // name
    //pub name: &'a str,
    /// [value] - specify the value
    #[cfg(feature = "tmux_1_0")]
    pub value: Option<&'a str>,
}

impl<'a> SetEnvironment<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SetEnvironmentBuilder<'a> {
    #[cfg(feature = "tmux_1_0")]
    pub global: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub remove: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub unset: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub target_session: Option<&'a str>,
    //pub name: &'a str,
    #[cfg(feature = "tmux_1_0")]
    pub value: Option<&'a str>,
}

impl<'a> SetEnvironmentBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn global(&mut self) -> &mut Self {
        self.global = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn remove(&mut self) -> &mut Self {
        self.remove = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn unset(&mut self) -> &mut Self {
        self.unset = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn target_session(&mut self, target_session: &'a str) -> &mut Self {
        self.target_session = Some(target_session);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn value(&mut self, value: &'a str) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn build(&self) -> SetEnvironment<'a> {
        SetEnvironment {
            #[cfg(feature = "tmux_1_0")]
            global: self.global,
            #[cfg(feature = "tmux_1_0")]
            remove: self.remove,
            #[cfg(feature = "tmux_1_0")]
            unset: self.unset,
            #[cfg(feature = "tmux_1_0")]
            target_session: self.target_session,
            #[cfg(feature = "tmux_1_0")]
            //name: &'a str,
            #[cfg(feature = "tmux_1_0")]
            value: self.value,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SET_ENVIRONMENT: &'static str = "set-environment";

    /// # Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux set-environment [-gru] [-t target-session] name [value]
    /// (alias: setenv)
    /// ```
    pub fn set_environment(
        &mut self,
        set_environment: Option<&SetEnvironment>,
        name: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(set_environment) = set_environment {
            #[cfg(feature = "tmux_1_0")]
            if set_environment.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if set_environment.remove.unwrap_or(false) {
                args.push(r_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if set_environment.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(target_session) = set_environment.target_session {
                args.extend_from_slice(&[t_KEY, &target_session])
            }
        }
        args.push(name);
        if let Some(set_environment) = set_environment {
            #[cfg(feature = "tmux_1_0")]
            if let Some(s) = set_environment.value {
                args.push(&s)
            }
        }
        let output = self.command(TmuxInterface::SET_ENVIRONMENT, &args)?;
        Ok(output)
    }
}
