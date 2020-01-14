use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Hooks" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#HOOKS)
impl<'a> TmuxInterface<'a> {
    const SHOW_HOOK: &'static str = "show-hook";
    /// # Manual
    ///
    /// ```text
    /// tmux show-hooks [-g] [-t target-session]
    /// ```
    pub fn show_hooks(
        &mut self,
        global: Option<bool>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if global.unwrap_or(false) {
            args.push(g_KEY);
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SHOW_HOOK, &args)?;
        Ok(output)
    }
}
