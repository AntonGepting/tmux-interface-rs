use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SHOW_ENVIRONMENT: &'static str = "show-environment";

    /// # Manual
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
    pub fn show_environment(
        &mut self,
        global: Option<bool>,
        shell_format: Option<bool>,
        target_session: Option<&'a str>,
        variable: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if global.unwrap_or(false) {
            args.push(g_KEY);
        }
        if shell_format.unwrap_or(false) {
            args.push(s_KEY);
        }
        if let Some(target_session) = target_session {
            args.extend_from_slice(&[t_KEY, &target_session])
        }
        if let Some(s) = variable {
            args.push(&s)
        }
        let output = self.subcommand(TmuxInterface::SHOW_ENVIRONMENT, &args)?;
        Ok(output)
    }
}
