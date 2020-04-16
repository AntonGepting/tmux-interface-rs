use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for open the command prompt in a client
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux command-prompt [-1ikN] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^3.0:
/// ```text
/// tmux command-prompt [-1Ni] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux command-prompt [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux command-prompt [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux command-prompt [-t target-client] [template]
/// ```
#[derive(Default, Debug)]
pub struct CommandPrompt<'a> {
    /// [-1] makesthe prompt only accept one key press
    #[cfg(feature = "tmux_2_4")]
    pub one_keypress: Option<bool>,
    /// [-i] execute the command every time the prompt input changes
    #[cfg(feature = "tmux_2_4")]
    pub on_input_change: Option<bool>,
    /// [-I inputs] - comma-separated list of the initial text for each prompt
    #[cfg(feature = "tmux_1_5")]
    pub inputs: Option<&'a str>,
    /// [-p prompts] - prompts is a comma-separated list of prompts which are displayed in order
    #[cfg(feature = "tmux_1_0")]
    pub prompts: Option<&'a str>,
    /// [-t target-client] - target-client
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<&'a str>,
    /// [template] - template
    #[cfg(feature = "tmux_0_8")]
    pub template: Option<&'a str>,
}

impl<'a> CommandPrompt<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct CommandPromptBuilder<'a> {
    #[cfg(feature = "tmux_2_4")]
    pub one_keypress: Option<bool>,
    #[cfg(feature = "tmux_2_4")]
    pub on_input_change: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub inputs: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub prompts: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub template: Option<&'a str>,
}

impl<'a> CommandPromptBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn one_keypress(&mut self) -> &mut Self {
        self.one_keypress = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn on_input_change(&mut self) -> &mut Self {
        self.on_input_change = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn inputs(&mut self, inputs: &'a str) -> &mut Self {
        self.inputs = Some(inputs);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn prompts(&mut self, prompts: &'a str) -> &mut Self {
        self.prompts = Some(prompts);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn template(&mut self, template: &'a str) -> &mut Self {
        self.template = Some(template);
        self
    }

    pub fn build(&self) -> CommandPrompt<'a> {
        CommandPrompt {
            #[cfg(feature = "tmux_2_4")]
            one_keypress: self.one_keypress,
            #[cfg(feature = "tmux_2_4")]
            on_input_change: self.on_input_change,
            #[cfg(feature = "tmux_1_5")]
            inputs: self.inputs,
            #[cfg(feature = "tmux_1_0")]
            prompts: self.prompts,
            #[cfg(feature = "tmux_0_8")]
            target_client: self.target_client,
            #[cfg(feature = "tmux_0_8")]
            template: self.template,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const COMMAND_PROMPT: &'static str = "command-prompt";

    /// # Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// tmux command-prompt [-1ikN] [-I inputs] [-p prompts] [-t target-client] [template]
    /// ```
    ///
    /// tmux ^3.0:
    /// ```text
    /// tmux command-prompt [-1Ni] [-I inputs] [-p prompts] [-t target-client] [template]
    /// ```
    ///
    /// tmux ^2.4:
    /// ```text
    /// tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux command-prompt [-I inputs] [-p prompts] [-t target-client] [template]
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux command-prompt [-p prompts] [-t target-client] [template]
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux command-prompt [-t target-client] [template]
    /// ```
    pub fn command_prompt(
        &mut self,
        command_prompt: Option<&CommandPrompt>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(command_prompt) = command_prompt {
            #[cfg(feature = "tmux_2_4")]
            {
                if command_prompt.one_keypress.unwrap_or(false) {
                    args.push(_1_KEY);
                }
            }
            #[cfg(feature = "tmux_2_4")]
            {
                if command_prompt.on_input_change.unwrap_or(false) {
                    args.push(i_KEY);
                }
            }
            #[cfg(feature = "tmux_1_5")]
            {
                if let Some(s) = command_prompt.inputs {
                    args.extend_from_slice(&[I_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_1_0")]
            {
                if let Some(s) = command_prompt.prompts {
                    args.extend_from_slice(&[p_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if let Some(s) = command_prompt.target_client {
                    args.extend_from_slice(&[t_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if let Some(s) = command_prompt.template {
                    args.push(&s)
                }
            }
        }
        let output = self.subcommand(TmuxInterface::COMMAND_PROMPT, &args)?;
        Ok(output)
    }
}
