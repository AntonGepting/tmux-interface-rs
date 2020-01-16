use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for displaying a message
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux display-message [-p] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct DisplayMessage<'a> {
    /// [-a] - list the format variables and their values
    pub list_format_vars: Option<bool>,
    /// [-I] - forward any input read from stdin to the empty pane given by target-pane
    pub forward_stdin: Option<bool>,
    /// [-p] - the output is printed to stdout
    pub print: Option<bool>,
    /// [-v] - print verbose logging as the format is parsed
    pub verbose: Option<bool>,
    /// [-c target-client] - target-client
    pub target_client: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [message] - message
    pub message: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a> DisplayMessage<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const DISPLAY_MESSAGE: &'static str = "display-message";

    /// Structure for displaying a message
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
    ///  (alias: display)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux display-message [-p] [-c target-client] [-t target-pane] [message]
    ///  (alias: display)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn display_message(
        &mut self,
        display_message: Option<&DisplayMessage>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(display_message) = display_message {
            if display_message.list_format_vars.unwrap_or(false) {
                args.push(a_KEY);
            }
            if display_message.forward_stdin.unwrap_or(false) {
                args.push(I_KEY);
            }
            if display_message.print.unwrap_or(false) {
                args.push(p_KEY);
            }
            if display_message.verbose.unwrap_or(false) {
                args.push(v_KEY);
            }
            if let Some(s) = display_message.target_client {
                args.extend_from_slice(&[c_KEY, s])
            }
            if let Some(s) = display_message.target_pane {
                args.extend_from_slice(&[t_KEY, s])
            }
            if let Some(s) = display_message.message {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::DISPLAY_MESSAGE, &args)?;
        Ok(output)
    }

    /// Structure for displaying a message
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
    ///  (alias: display)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux display-message [-p] [-c target-client] [-t target-pane] [message]
    ///  (alias: display)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn display_message(
        &mut self,
        print: Option<bool>,
        target_client: Option<&'a str>,
        target_pane: Option<&'a str>,
        message: Option<&'a str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if print.unwrap_or(false) {
            args.push(p_KEY);
        }
        if let Some(s) = target_client {
            args.extend_from_slice(&[c_KEY, s])
        }
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, s])
        }
        if let Some(s) = message {
            args.push(&s)
        }
        let output = self.subcommand(TmuxInterface::DISPLAY_MESSAGE, &args)?;
        Ok(output)
    }
}
