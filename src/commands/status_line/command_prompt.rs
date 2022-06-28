use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;
#[cfg(feature = "tmux_3_3")]
use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg(feature = "tmux_3_3")]
pub enum PromptType {
    /// `command`
    Command,
    /// `search`
    Search,
    /// `target`
    Target,
    /// `window-target`
    WindowTarget,
}

#[cfg(feature = "tmux_3_3")]
impl fmt::Display for PromptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            PromptType::Command => "command",
            PromptType::Search => "search",
            PromptType::Target => "target",
            PromptType::WindowTarget => "window-target",
        };

        write!(f, "{}", s)
    }
}

/// Structure for open the command prompt in a client
///
/// # Manual
///
/// tmux ^3.3:
/// ```text
/// command-prompt [-1bFikN] [-I inputs] [-p prompts] [-t target-client] [-T prompt-type] [template]
/// ```
///
/// tmux ^3.2:
/// ```text
/// tmux command-prompt [-1ikNTW] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CommandPrompt<'a> {
    /// `[-1]` - makes the prompt only accept one key press
    #[cfg(feature = "tmux_2_4")]
    pub one_keypress: bool,

    /// `[-b]` - the prompt is shown in the background and the invoking client does not exit until it is dismissed
    #[cfg(feature = "tmux_3_3")]
    pub background: bool,

    /// `[-F]` -
    #[cfg(feature = "tmux_3_3")]
    pub expand_as_format: bool,

    /// `[-i]` execute the command every time the prompt input changes
    #[cfg(feature = "tmux_2_4")]
    pub on_input_change: bool,

    /// `[-k]` - the key press is translated to a key name
    #[cfg(feature = "tmux_3_1")]
    pub key_name: bool,

    /// `[-N]` - makes the prompt only accept numeric key presses
    #[cfg(feature = "tmux_3_1")]
    pub numeric: bool,

    /// `[-T]` - tells tmux that the prompt is for a target which affects what completions are offered when Tab is pressed
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    pub for_target: bool,

    /// `[-T prompt-type]` - tells tmux the prompt type. This affects what completions are offered when Tab is pressed
    #[cfg(feature = "tmux_3_3")]
    pub prompt_type: Option<PromptType>,

    /// `[-W]` - indicates the prompt is for a window.
    #[cfg(feature = "tmux_3_2")]
    pub for_window: bool,

    /// `[-I inputs]` - comma-separated list of the initial text for each prompt
    #[cfg(feature = "tmux_1_5")]
    pub inputs: Option<Cow<'a, str>>,

    /// `[-p prompts]` - prompts is a comma-separated list of prompts which are displayed in order
    #[cfg(feature = "tmux_1_0")]
    pub prompts: Option<Cow<'a, str>>,

    /// `[-t target-client]` - target-client
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[template]` - template
    #[cfg(feature = "tmux_0_8")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> CommandPrompt<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-1]` makesthe prompt only accept one key press
    #[cfg(feature = "tmux_2_4")]
    pub fn one_keypress(mut self) -> Self {
        self.one_keypress = true;
        self
    }

    /// `[-i]` execute the command every time the prompt input changes
    #[cfg(feature = "tmux_2_4")]
    pub fn on_input_change(mut self) -> Self {
        self.on_input_change = true;
        self
    }

    /// `[-k]` - the key press is translated to a key name
    #[cfg(feature = "tmux_3_1")]
    pub fn key_name(mut self) -> Self {
        self.key_name = true;
        self
    }

    /// `[-N]` - makes the prompt only accept numeric key presses
    #[cfg(feature = "tmux_3_1")]
    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }

    /// `[-T]` - tells tmux that the prompt is for a target which affects what completions are offered when Tab is pressed
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    pub fn for_target(mut self) -> Self {
        self.for_target = true;
        self
    }

    /// `[-T prompt-type]`
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_type(mut self, prompt_type: PromptType) -> Self {
        self.prompt_type = Some(prompt_type);
        self
    }

    /// `[-W]` - indicates the prompt is for a window.
    #[cfg(feature = "tmux_3_2")]
    pub fn for_window(mut self) -> Self {
        self.for_window = true;
        self
    }

    /// `[-I inputs]` - comma-separated list of the initial text for each prompt
    #[cfg(feature = "tmux_1_5")]
    pub fn inputs<S: Into<Cow<'a, str>>>(mut self, inputs: S) -> Self {
        self.inputs = Some(inputs.into());
        self
    }

    /// `[-p prompts]` - prompts is a comma-separated list of prompts which are displayed in order
    #[cfg(feature = "tmux_1_0")]
    pub fn prompts<S: Into<Cow<'a, str>>>(mut self, prompts: S) -> Self {
        self.prompts = Some(prompts.into());
        self
    }

    /// `[-t target-client]` - target-client
    #[cfg(feature = "tmux_0_8")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[template]` - template
    #[cfg(feature = "tmux_0_8")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(COMMAND_PROMPT);

        // `[-1]` makesthe prompt only accept one key press
        #[cfg(feature = "tmux_2_4")]
        if self.one_keypress {
            cmd.push_flag(_1_KEY);
        }

        // `[-i]` execute the command every time the prompt input changes
        #[cfg(feature = "tmux_2_4")]
        if self.on_input_change {
            cmd.push_flag(I_LOWERCASE_KEY);
        }

        // `[-k]` - the key press is translated to a key name
        #[cfg(feature = "tmux_3_1")]
        if self.key_name {
            cmd.push_flag(K_LOWERCASE_KEY);
        }

        // `[-N]` - makes the prompt only accept numeric key presses
        #[cfg(feature = "tmux_3_1")]
        if self.numeric {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-T]` - tells tmux that the prompt is for a target which affects what completions are offered when Tab is pressed
        #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
        if self.for_target {
            cmd.push_flag(T_UPPERCASE_KEY);
        }

        // `[-T prompt-type]` -
        #[cfg(feature = "tmux_3_3")]
        if let Some(prompt_type) = self.prompt_type {
            cmd.push_option(T_UPPERCASE_KEY, prompt_type.to_string());
        }

        // `[-W]` - indicates the prompt is for a window.
        #[cfg(feature = "tmux_3_2")]
        if self.for_window {
            cmd.push_flag(W_UPPERCASE_KEY);
        }

        // `[-I inputs]` - comma-separated list of the initial text for each prompt
        #[cfg(feature = "tmux_1_5")]
        if let Some(inputs) = self.inputs {
            cmd.push_option(I_UPPERCASE_KEY, inputs);
        }

        // `[-p prompts]` - prompts is a comma-separated list of prompts which are displayed in order
        #[cfg(feature = "tmux_1_0")]
        if let Some(prompts) = self.prompts {
            cmd.push_option(P_LOWERCASE_KEY, prompts);
        }

        // `[-t target-client]` - target-client
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client);
        }

        // `[template]` - template
        #[cfg(feature = "tmux_0_8")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
