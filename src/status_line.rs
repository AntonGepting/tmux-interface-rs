use super::error::Error;
use super::tmux_interface::*;
use std::process::Output;

/// # Manual
///
/// ```text
/// tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
#[derive(Default, Debug)]
pub struct CommandPrompt<'a> {
    pub one_keypress: Option<bool>,     // [-1]
    pub on_input_change: Option<bool>,  // [-i]
    pub inputs: Option<&'a str>,        // [-I inputs]
    pub prompts: Option<&'a str>,       // [-p prompts]
    pub target_client: Option<&'a str>, // [-t target-client]
    pub template: Option<&'a str>,      // [template]
}

impl<'a> CommandPrompt<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Status line
impl<'a> TmuxInterface<'a> {
    const COMMAND_PROMPT: &'static str = "command-prompt";
    const CONFIRM_BEFORE: &'static str = "confirm-before";
    const DISPLAY_MESSAGE: &'static str = "display-message";

    /// # Manual
    ///
    /// ```text
    /// tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
    /// ```
    pub fn command_prompt(&self, command_prompt: Option<&CommandPrompt>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(command_prompt) = command_prompt {
            if command_prompt.one_keypress.unwrap_or(false) {
                args.push(_1_KEY);
            }
            if command_prompt.on_input_change.unwrap_or(false) {
                args.push(i_KEY);
            }
            if let Some(s) = command_prompt.inputs {
                args.extend_from_slice(&[I_KEY, &s])
            }
            if let Some(s) = command_prompt.prompts {
                args.extend_from_slice(&[p_KEY, &s])
            }
            if let Some(s) = command_prompt.target_client {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = command_prompt.template {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::COMMAND_PROMPT, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux confirm-before [-p prompt] [-t target-client] command
    /// (alias: confirm)
    /// ```
    pub fn confirm_before(
        &self,
        prompt: Option<&str>,
        target_client: Option<&str>,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = prompt {
            args.extend_from_slice(&[p_KEY, &s])
        }
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(command);
        let output = self.subcommand(TmuxInterface::CONFIRM_BEFORE, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux display-message [-p] [-c target-client] [-t target-pane] [message]
    /// (alias: display)
    /// ```
    pub fn display_message(
        &self,
        print: Option<bool>,
        target_client: Option<&str>,
        target_pane: Option<&str>,
        message: Option<&str>,
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
