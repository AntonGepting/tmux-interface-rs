use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type ShowPHist<'a> = ShowPromptHistory<'a>;

/// # Manual
///
/// tmux ^3.3:
/// ```text
/// show-prompt-history [-T prompt-type]
/// (alias: showphist)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ShowPromptHistory<'a> {
    /// `[-T prompt-type]`
    #[cfg(feature = "tmux_3_3")]
    pub prompt_type: Option<Cow<'a, str>>,
}

impl<'a> ShowPromptHistory<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-T prompt]`
    #[cfg(feature = "tmux_1_5")]
    pub fn prompt_type<S: Into<Cow<'a, str>>>(mut self, prompt_type: S) -> Self {
        self.prompt_type = Some(prompt_type.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SHOW_PROMPT_HISTORY);

        // `[-T prompt-type]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(prompt_type) = self.prompt_type {
            cmd.push_option(T_UPPERCASE_KEY, prompt_type);
        }

        cmd
    }
}
