use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
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
    pub global: Option<bool>,
    /// [-r] - remove the variable from the environment before starting a new process
    pub remove: Option<bool>,
    /// [-u] - unset a variable
    pub unset: Option<bool>,
    /// [-t target-session] - target-session
    pub target_session: Option<&'a TargetSession<'a>>,
    // name
    //pub name: &'a str,
    /// [value] - specify the value
    pub value: Option<&'a str>,
}

impl<'a> SetEnvironment<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SetEnvironmentBuilder<'a> {
    pub global: Option<bool>,
    pub remove: Option<bool>,
    pub unset: Option<bool>,
    pub target_session: Option<&'a TargetSession<'a>>,
    //pub name: &'a str,
    pub value: Option<&'a str>,
}

impl<'a> SetEnvironmentBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn global(&mut self) -> &mut Self {
        self.global = Some(true);
        self
    }

    pub fn remove(&mut self) -> &mut Self {
        self.remove = Some(true);
        self
    }

    pub fn unset(&mut self) -> &mut Self {
        self.unset = Some(true);
        self
    }

    pub fn target_session(&mut self, target_session: &'a TargetSession<'a>) -> &mut Self {
        self.target_session = Some(target_session);
        self
    }

    pub fn value(&mut self, value: &'a str) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn build(&self) -> SetEnvironment<'a> {
        SetEnvironment {
            global: self.global,
            remove: self.remove,
            unset: self.unset,
            target_session: self.target_session,
            //name: &'a str,
            value: self.value,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SET_ENVIRONMENT: &'static str = "set-environment";

    /// # Manual
    ///
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
        let s;
        if let Some(set_environment) = set_environment {
            if set_environment.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if set_environment.remove.unwrap_or(false) {
                args.push(r_KEY);
            }
            if set_environment.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            if let Some(target_session) = set_environment.target_session {
                s = target_session.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(name);
        if let Some(set_environment) = set_environment {
            if let Some(s) = set_environment.value {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::SET_ENVIRONMENT, &args)?;
        Ok(output)
    }
}
