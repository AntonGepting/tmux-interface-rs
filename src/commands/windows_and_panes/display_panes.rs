use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const DISPLAY_PANES: &'static str = "display-panes";
    #[cfg(feature = "use_cmd_alias")]
    const DISPLAY_PANES: &'static str = "displayp";

    /// Display a visible indicator of each pane shown by `target-client`
    ///
    /// # Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// tmux display-panes [-b] [-d duration] [-t target-client] [template]
    /// (alias: displayp)
    /// ```
    ///
    /// tmux ^2.6:
    /// ```text
    /// tmux display-panes [-d duration] [-t target-client] [template]
    /// (alias: displayp)
    /// ```
    ///
    /// tmux ^2.3:
    /// ```text
    /// tmux display-panes [-t target-client] [template]
    /// (alias: displayp)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux display-panes [-t target-client]
    /// (alias: displayp)
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
        let output = self.command(TmuxInterface::DISPLAY_PANES, &args)?;
        Ok(output)
    }
}
