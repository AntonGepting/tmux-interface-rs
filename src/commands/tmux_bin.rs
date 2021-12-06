use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct TmuxBin<'a> {
    /// Tmux environment variables
    pub env: Option<Vec<(Cow<'a, str>, Cow<'a, str>)>>,
    /// Tmux executable name, (part I)
    pub bin: Cow<'a, str>,
    /// Tmux executable arguments (part II)
    pub args: Option<Vec<Cow<'a, str>>>,
}

impl<'a> Default for TmuxBin<'a> {
    fn default() -> Self {
        TmuxBin {
            env: None,
            bin: Cow::Borrowed(TMUX),
            args: None,
        }
    }
}

impl<'a> TmuxBin<'a> {
    pub fn new() -> Self {
        TmuxBin::default()
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.args = Some(vec![Cow::Borrowed("-v")]);
        self
    }
}
