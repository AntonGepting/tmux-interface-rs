// auto-generated file
//

use crate::commands::constants::*;
#[cfg(feature = "tmux_3_3")]
use crate::PromptType;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Open the command prompt in a client
///
/// # Manual
///
///
/// tmux >=3.6:
/// ```text
/// command-prompt [-1bFiklN] [-I inputs] [-p prompts] [-t target-client] [-T prompt-type] [template]
/// ```
///
/// tmux >=3.3:
/// ```text
/// command-prompt [-1bFikN] [-I inputs] [-p prompts] [-t target-client] [-T prompt-type] [template]
/// ```
///
/// tmux >=3.2:
/// ```text
/// command-prompt [-1ikNTW] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=3.1:
/// ```text
/// command-prompt [-1ikN] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=3.0a:
/// ```text
/// command-prompt [-1Ni] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=2.4:
/// ```text
/// command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=1.5:
/// ```text
/// command-prompt [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=0.8:
/// ```text
/// command-prompt [-t target-client] [template]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CommandPrompt<'a> {
    /// `[-1]`
    #[cfg(feature = "tmux_2_4")]
    pub one_keypress: bool,

    /// `[-b]`
    #[cfg(feature = "tmux_3_3")]
    pub background: bool,

    /// `[-F]`
    #[cfg(feature = "tmux_3_3")]
    pub expand_as_format: bool,

    /// `[-i]`
    #[cfg(feature = "tmux_2_4")]
    pub on_input_change: bool,

    /// `[-k]`
    #[cfg(feature = "tmux_3_1")]
    pub key_name: bool,

    /// `[-l]`
    #[cfg(feature = "tmux_3_6")]
    pub disable_splitting: bool,

    /// `[-N]`
    #[cfg(feature = "tmux_3_0a")]
    pub numeric: bool,

    /// `[-T]`
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    pub prompt_type: bool,

    /// `[-W]`
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    pub for_window: bool,

    /// `[-I inputs]`
    #[cfg(feature = "tmux_1_5")]
    pub inputs: Option<Cow<'a, str>>,

    /// `[-p prompts]`
    #[cfg(feature = "tmux_1_5")]
    pub prompts: Option<Cow<'a, str>>,

    /// `[-t target-client]`
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-T prompt-type]`
    #[cfg(feature = "tmux_3_3")]
    pub prompt_type: Option<PromptType>,

    /// `[template]`
    #[cfg(feature = "tmux_0_8")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> CommandPrompt<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-1]`
    #[cfg(feature = "tmux_2_4")]
    pub fn one_keypress(mut self) -> Self {
        self.one_keypress = true;
        self
    }

    /// `[-b]`
    #[cfg(feature = "tmux_3_3")]
    pub fn background(mut self) -> Self {
        self.background = true;
        self
    }

    /// `[-F]`
    #[cfg(feature = "tmux_3_3")]
    pub fn expand_as_format(mut self) -> Self {
        self.expand_as_format = true;
        self
    }

    /// `[-i]`
    #[cfg(feature = "tmux_2_4")]
    pub fn on_input_change(mut self) -> Self {
        self.on_input_change = true;
        self
    }

    /// `[-k]`
    #[cfg(feature = "tmux_3_1")]
    pub fn key_name(mut self) -> Self {
        self.key_name = true;
        self
    }

    /// `[-l]`
    #[cfg(feature = "tmux_3_6")]
    pub fn disable_splitting(mut self) -> Self {
        self.disable_splitting = true;
        self
    }

    /// `[-N]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }

    /// `[-T]`
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    pub fn prompt_type(mut self) -> Self {
        self.prompt_type = true;
        self
    }

    /// `[-W]`
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    pub fn for_window(mut self) -> Self {
        self.for_window = true;
        self
    }

    /// `[-I inputs]`
    #[cfg(feature = "tmux_1_5")]
    pub fn inputs<S: Into<Cow<'a, str>>>(mut self, inputs: S) -> Self {
        self.inputs = Some(inputs.into());
        self
    }

    /// `[-p prompts]`
    #[cfg(feature = "tmux_1_5")]
    pub fn prompts<S: Into<Cow<'a, str>>>(mut self, prompts: S) -> Self {
        self.prompts = Some(prompts.into());
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-T prompt-type]`
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_type(mut self, prompt_type: PromptType) -> Self {
        self.prompt_type = Some(prompt_type);
        self
    }

    /// `[template]`
    #[cfg(feature = "tmux_0_8")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(COMMAND_PROMPT);

        // `[-1]`
        #[cfg(feature = "tmux_2_4")]
        if self.one_keypress {
            cmd.push_flag(_1_KEY);
        }

        // `[-b]`
        #[cfg(feature = "tmux_3_3")]
        if self.background {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-F]`
        #[cfg(feature = "tmux_3_3")]
        if self.expand_as_format {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-i]`
        #[cfg(feature = "tmux_2_4")]
        if self.on_input_change {
            cmd.push_flag(I_LOWERCASE_KEY);
        }

        // `[-k]`
        #[cfg(feature = "tmux_3_1")]
        if self.key_name {
            cmd.push_flag(K_LOWERCASE_KEY);
        }

        // `[-l]`
        #[cfg(feature = "tmux_3_6")]
        if self.disable_splitting {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-N]`
        #[cfg(feature = "tmux_3_0a")]
        if self.numeric {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-T]`
        #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
        if self.prompt_type {
            cmd.push_flag(T_UPPERCASE_KEY);
        }

        // `[-W]`
        #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
        if self.for_window {
            cmd.push_flag(W_UPPERCASE_KEY);
        }

        // `[-I inputs]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(inputs) = self.inputs {
            cmd.push_option(I_UPPERCASE_KEY, inputs);
        }

        // `[-p prompts]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(prompts) = self.prompts {
            cmd.push_option(P_LOWERCASE_KEY, prompts);
        }

        // `[-t target-client]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client);
        }

        // `[-T prompt-type]`
        #[cfg(feature = "tmux_3_3")]
        if let Some(prompt_type) = self.prompt_type {
            cmd.push_option(T_UPPERCASE_KEY, prompt_type.to_string());
        }

        // `[template]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
