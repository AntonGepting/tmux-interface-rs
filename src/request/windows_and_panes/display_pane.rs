use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const DISPLAY_PANES: &'static str = "display-panes";

    /// Display a visible indicator of each pane shown by `target-client`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux display-panes [-b] [-d duration] [-t target-client] [template] (alias: displayp)
    /// ```
    pub fn display_panes(
        &mut self,
        not_block: Option<bool>,
        duration: Option<&str>,
        target_client: Option<&str>,
        template: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if not_block.unwrap_or(false) {
            args.push(b_KEY);
        }
        if let Some(s) = duration {
            args.extend_from_slice(&[d_KEY, &s])
        }
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = template {
            args.push(&s)
        }
        let output = self.subcommand(TmuxInterface::DISPLAY_PANES, &args)?;
        Ok(output)
    }
}
