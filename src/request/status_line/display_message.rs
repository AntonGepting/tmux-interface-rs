use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Structure for displaying a message
///
/// # Manual
///
/// tmux ^3.0a:
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
pub struct DisplayMessage<'a, T: Display> {
    #[cfg(any(feature = "tmux_2_9", feature = "tmux_X_X"))]
    /// [-a] - list the format variables and their values
    pub list_format_vars: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    /// [-I] - forward any input read from stdin to the empty pane given by target-pane
    pub forward_stdin: Option<bool>,
    /// [-p] - the output is printed to stdout
    pub print: Option<bool>,
    #[cfg(any(feature = "tmux_2_9", feature = "tmux_X_X"))]
    /// [-v] - print verbose logging as the format is parsed
    pub verbose: Option<bool>,
    /// [-c target-client] - target-client
    pub target_client: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    /// [message] - message
    pub message: Option<&'a str>,
}

impl<'a, T: Display + Default> DisplayMessage<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct DisplayMessageBuilder<'a, T> {
    #[cfg(feature = "tmux_X_X")]
    pub list_format_vars: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    pub forward_stdin: Option<bool>,
    pub print: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    pub verbose: Option<bool>,
    pub target_client: Option<&'a str>,
    pub target_pane: Option<&'a T>,
    pub message: Option<&'a str>,
}

impl<'a, T: Display + Default> DisplayMessageBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_X_X")]
    pub fn list_format_vars(&mut self) -> &mut Self {
        self.list_format_vars = Some(true);
        self
    }

    #[cfg(feature = "tmux_X_X")]
    pub fn forward_stdin(&mut self) -> &mut Self {
        self.forward_stdin = Some(true);
        self
    }

    pub fn print(&mut self) -> &mut Self {
        self.print = Some(true);
        self
    }

    #[cfg(feature = "tmux_X_X")]
    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = Some(true);
        self
    }

    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn message(&mut self, message: &'a str) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn build(&self) -> DisplayMessage<'a, T> {
        DisplayMessage {
            #[cfg(feature = "tmux_X_X")]
            list_format_vars: self.list_format_vars,
            #[cfg(feature = "tmux_X_X")]
            forward_stdin: self.forward_stdin,
            print: self.print,
            #[cfg(feature = "tmux_X_X")]
            verbose: self.verbose,
            target_client: self.target_client,
            target_pane: self.target_pane,
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
    pub fn display_message<T: Display>(
        &mut self,
        display_message: Option<&DisplayMessage<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(display_message) = display_message {
            #[cfg(feature = "tmux_X_X")]
            {
                if display_message.list_format_vars.unwrap_or(false) {
                    args.push(a_KEY);
                }
            }
            #[cfg(feature = "tmux_X_X")]
            {
                if display_message.forward_stdin.unwrap_or(false) {
                    args.push(I_KEY);
                }
            }
            if display_message.print.unwrap_or(false) {
                args.push(p_KEY);
            }
            #[cfg(feature = "tmux_X_X")]
            {
                if display_message.verbose.unwrap_or(false) {
                    args.push(v_KEY);
                }
            }
            if let Some(s) = display_message.target_client {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(target_pane) = display_message.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = display_message.message {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::DISPLAY_MESSAGE, &args)?;
        Ok(output)
    }
}
