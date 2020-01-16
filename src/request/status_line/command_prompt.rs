use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for open the command prompt in a client
///
/// # Manual
///
/// ```text
/// tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
#[derive(Default, Debug)]
pub struct CommandPrompt<'a> {
    /// [-1] makesthe prompt only accept one key press
    pub one_keypress: Option<bool>,
    /// [-i] execute the command every time the prompt input changes
    pub on_input_change: Option<bool>,
    /// [-I inputs] - comma-separated list of the initial text for each prompt
    pub inputs: Option<&'a str>,
    /// [-p prompts] - prompts is a comma-separated list of prompts which are displayed in order
    pub prompts: Option<&'a str>,
    /// [-t target-client] - target-client
    pub target_client: Option<&'a str>,
    /// [template] - template
    pub template: Option<&'a str>,
}

impl<'a> CommandPrompt<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const COMMAND_PROMPT: &'static str = "command-prompt";

    /// # Manual
    ///
    /// ```text
    /// tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
    /// ```
    pub fn command_prompt(
        &mut self,
        command_prompt: Option<&CommandPrompt>,
    ) -> Result<Output, Error> {
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
}
