use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SHOW_ENVIRONMENT: &'static str = "show-environment";

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
        target_session: Option<&TargetSession<'a>>,
        variable: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if global.unwrap_or(false) {
            args.push(g_KEY);
        }
        if shell_format.unwrap_or(false) {
            args.push(s_KEY);
        }
        if let Some(target_session) = target_session {
            s = target_session.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = variable {
            args.push(&s)
        }
        let output = self.subcommand(TmuxInterface::SHOW_ENVIRONMENT, &args)?;
        Ok(output)
    }
}
