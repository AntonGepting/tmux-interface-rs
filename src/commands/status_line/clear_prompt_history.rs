// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type ClearPHist<'a> = ClearPromptHistory<'a>;

/// Clear status prompt history
///
/// # Manual
///
/// tmux >=3.3:
/// ```text
/// clear-prompt-history [-T prompt-type]
/// (alias: clearphist)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ClearPromptHistory<'a> {
    /// `[-T prompt-type]`
    #[cfg(feature = "tmux_3_3")]
    pub prompt_type: Option<Cow<'a, str>>,
}

impl<'a> ClearPromptHistory<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-T prompt-type]`
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_type<S: Into<Cow<'a, str>>>(mut self, prompt_type: S) -> Self {
        self.prompt_type = Some(prompt_type.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CLEAR_PROMPT_HISTORY);

        // `[-T prompt-type]`
        #[cfg(feature = "tmux_3_3")]
        if let Some(prompt_type) = self.prompt_type {
            cmd.push_option(T_UPPERCASE_KEY, prompt_type);
        }

        cmd
    }
}
