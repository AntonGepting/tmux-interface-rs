use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;

/// Status line
impl<'a> TmuxInterface<'a> {

    const DISPLAY_MESSAGE: &'static str = "display-message";

    /// ```text
    /// tmux display-message [-p] [-c target-client] [-t target-pane] [message]
    /// (alias: display)
    /// ```
    pub fn display_message(&self, print: Option<bool>, target_client: Option<&str>,
                           target_pane: Option<&str>, message: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if print.unwrap_or(false) { args.push(p_KEY); }
        target_client.and_then(|s| Some(args.extend_from_slice(&[c_KEY, s])));
        target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
        message.and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::DISPLAY_MESSAGE, &args)?;
        Ok(output.status.success())
    }


}
