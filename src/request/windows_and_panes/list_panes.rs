use crate::error::Error;
use crate::tmux_interface::*;
//use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const LIST_PANES: &'static str = "list-panes";

    // XXX: better return type
    /// List panes on the server
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-panes [-as] [-F format] [-t target]
    /// (alias: lsp)
    /// ```
    pub fn list_panes(
        &mut self,
        all: Option<bool>,
        session: Option<bool>,
        format: Option<&str>,
        target: Option<&str>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if session.unwrap_or(false) {
            args.push(s_KEY);
        }
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        if let Some(s) = target {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_PANES, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
