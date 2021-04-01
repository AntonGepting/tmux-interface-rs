use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

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
#[derive(Debug, Clone)]
pub struct CommandPrompt<'a>(pub TmuxCommand<'a>);

impl<'a> Default for CommandPrompt<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(COMMAND_PROMPT)),
            ..Default::default()
        })
    }
}

impl<'a> CommandPrompt<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-1]` makesthe prompt only accept one key press
    #[cfg(feature = "tmux_2_4")]
    pub fn one_keypress(&mut self) -> &mut Self {
        self.0.push_flag(_1_KEY);
        self
    }

    /// `[-i]` execute the command every time the prompt input changes
    #[cfg(feature = "tmux_2_4")]
    pub fn on_input_change(&mut self) -> &mut Self {
        self.0.push_flag(I_LOWERCASE_KEY);
        self
    }

    /// `[-I inputs]` - comma-separated list of the initial text for each prompt
    #[cfg(feature = "tmux_1_5")]
    pub fn inputs<S: Into<Cow<'a, str>>>(&mut self, inputs: S) -> &mut Self {
        self.0.push_option(I_UPPERCASE_KEY, inputs);
        self
    }

    /// `[-p prompts]` - prompts is a comma-separated list of prompts which are displayed in order
    #[cfg(feature = "tmux_1_0")]
    pub fn prompts<S: Into<Cow<'a, str>>>(&mut self, prompts: S) -> &mut Self {
        self.0.push_option(P_LOWERCASE_KEY, prompts);
        self
    }

    /// `[-t target-client]` - target-client
    #[cfg(feature = "tmux_0_8")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_client);
        self
    }

    /// `[template]` - template
    #[cfg(feature = "tmux_0_8")]
    pub fn template<S: Into<Cow<'a, str>>>(&mut self, template: S) -> &mut Self {
        self.0.push_param(template);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for CommandPrompt<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(COMMAND_PROMPT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for CommandPrompt<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(COMMAND_PROMPT)),
            ..Default::default()
        })
    }
}
