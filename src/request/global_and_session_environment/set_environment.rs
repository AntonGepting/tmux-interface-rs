use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for setting or unsetting an environment variable
///
/// # Manual
///
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
    pub target_session: Option<&'a str>,
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
/// All functions from man tmux "Global and session environment" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT)
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
            if let Some(s) = set_environment.target_session {
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
