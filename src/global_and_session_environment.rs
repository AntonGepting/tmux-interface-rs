use super::error::Error;
use super::tmux_interface::*;
use std::process::Output;

/// # Manual
///
/// ```text
/// tmux set-environment [-gru] [-t target-session] name [value]
/// (alias: setenv)
/// ```
#[derive(Default, Debug)]
pub struct SetEnvironment<'a> {
    pub global: Option<bool>,            // [-g]
    pub remove: Option<bool>,            // [-r]
    pub unset: Option<bool>,             // [-u]
    pub target_session: Option<&'a str>, // [-t target-session]
    //pub name: &'a str,                   // name
    pub value: Option<&'a str>, // [value]
}

impl<'a> SetEnvironment<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Global and session environment
impl<'a> TmuxInterface<'a> {
    const SET_ENVIRONMENT: &'static str = "set-environment";
    const SHOW_ENVIRONMENT: &'static str = "show-environment";

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

    /// # Manual
    ///
    /// ```text
    /// tmux show-environment [-gs] [-t target-session] [variable]
    /// (alias: showenv)
    /// ```
    pub fn show_environment(
        &mut self,
        global: Option<bool>,
        shell_format: Option<bool>,
        target_session: Option<&str>,
        variable: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if global.unwrap_or(false) {
            args.push(g_KEY);
        }
        if shell_format.unwrap_or(false) {
            args.push(s_KEY);
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = variable {
            args.push(&s)
        }
        let output = self.subcommand(TmuxInterface::SHOW_ENVIRONMENT, &args)?;
        Ok(output)
    }
}
