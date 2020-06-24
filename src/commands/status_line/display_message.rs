use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for displaying a message
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux ^2.9a:
/// ```text
/// tmux display-message [-apv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux display-message [-p] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux display-message [-p] [-t target-client] [message]
///  (alias: display)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux display-message [-t target-client] [message]
///  (alias: display)
/// ```
#[derive(Default, Debug)]
pub struct DisplayMessage<'a> {
    /// [-a] - list the format variables and their values
    #[cfg(feature = "tmux_2_9a")]
    pub list_format_vars: Option<bool>,
    /// [-I] - forward any input read from stdin to the empty pane given by target-pane
    #[cfg(feature = "tmux_3_0")]
    pub forward_stdin: Option<bool>,
    /// [-p] - the output is printed to stdout
    #[cfg(feature = "tmux_2_9a")]
    pub print: Option<bool>,
    /// [-v] - print verbose logging as the format is parsed
    #[cfg(feature = "tmux_2_9a")]
    pub verbose: Option<bool>,
    /// [-c target-client] - target-client
    #[cfg(feature = "tmux_1_0")]
    pub target_client: Option<&'a str>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<&'a str>,
    /// [message] - message
    #[cfg(feature = "tmux_1_0")]
    pub message: Option<&'a str>,
}

impl<'a> DisplayMessage<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct DisplayMessageBuilder<'a> {
    #[cfg(feature = "tmux_2_9a")]
    pub list_format_vars: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub forward_stdin: Option<bool>,
    #[cfg(feature = "tmux_2_9a")]
    pub print: Option<bool>,
    #[cfg(feature = "tmux_2_9a")]
    pub verbose: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub target_client: Option<&'a str>,
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub message: Option<&'a str>,
}

impl<'a> DisplayMessageBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn list_format_vars(&mut self) -> &mut Self {
        self.list_format_vars = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn forward_stdin(&mut self) -> &mut Self {
        self.forward_stdin = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn print(&mut self) -> &mut Self {
        self.print = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn message(&mut self, message: &'a str) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn build(&self) -> DisplayMessage<'a> {
        DisplayMessage {
            #[cfg(feature = "tmux_2_9a")]
            list_format_vars: self.list_format_vars,
            #[cfg(feature = "tmux_3_0")]
            forward_stdin: self.forward_stdin,
            #[cfg(feature = "tmux_2_9a")]
            print: self.print,
            #[cfg(feature = "tmux_2_9a")]
            verbose: self.verbose,
            #[cfg(feature = "tmux_1_0")]
            target_client: self.target_client,
            #[cfg(feature = "tmux_1_5")]
            target_pane: self.target_pane,
            #[cfg(feature = "tmux_1_0")]
            message: self.message,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const DISPLAY_MESSAGE: &'static str = "display-message";

    /// Structure for displaying a message
    ///
    /// # Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
    ///  (alias: display)
    /// ```
    ///
    /// tmux ^2.9a:
    /// ```text
    /// tmux display-message [-apv] [-c target-client] [-t target-pane] [message]
    ///  (alias: display)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux display-message [-p] [-c target-client] [-t target-pane] [message]
    ///  (alias: display)
    /// ```
    ///
    /// tmux ^1.2:
    /// ```text
    /// tmux display-message [-p] [-t target-client] [message]
    ///  (alias: display)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux display-message [-t target-client] [message]
    ///  (alias: display)
    /// ```
    pub fn display_message(
        &mut self,
        display_message: Option<&DisplayMessage>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(display_message) = display_message {
            #[cfg(feature = "tmux_2_9a")]
            if display_message.list_format_vars.unwrap_or(false) {
                args.push(a_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if display_message.forward_stdin.unwrap_or(false) {
                args.push(I_KEY);
            }
            #[cfg(feature = "tmux_2_9a")]
            if display_message.print.unwrap_or(false) {
                args.push(p_KEY);
            }
            #[cfg(feature = "tmux_2_9a")]
            if display_message.verbose.unwrap_or(false) {
                args.push(v_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(s) = display_message.target_client {
                args.extend_from_slice(&[c_KEY, &s])
            }
            #[cfg(feature = "tmux_1_5")]
            if let Some(target_pane) = display_message.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(message) = display_message.message {
                args.push(&message)
            }
        }
        let output = self.command(TmuxInterface::DISPLAY_MESSAGE, &args)?;
        Ok(output)
    }
}
