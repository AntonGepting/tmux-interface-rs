use crate::commands::constants::*;
use crate::commands::tmux_bin::TmuxBin;
use crate::commands::tmux_bin_commands::TmuxBinCommands;
use crate::NewSession;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;
use std::process::{Command, Stdio};

//#[derive(Debug, Clone)]
//pub struct TmuxCommands<'a>(pub Vec<TmuxCommand<'a>>);

#[derive(Debug, Clone)]
pub struct TmuxCommands<'a>(pub Vec<TmuxCommand<'a>>);

impl<'a> Default for TmuxCommands<'a> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<'a> TmuxCommands<'a> {
    pub fn new() -> Self {
        TmuxCommands(Vec::new())
    }

    pub fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T) {
        self.0.push(cmd.into());
    }

    pub fn to_vec(&self) -> Vec<&Cow<'a, str>> {
        let mut v = Vec::new();

        //if let Some(tmux_args) = &self.tmux.args {
        //v.extend(tmux_args);
        //}

        for command in &self.0 {
            if let Some(cmd) = &command.cmd {
                v.push(cmd);
            }
            if let Some(args) = &command.args {
                v.extend(args);
            }
        }

        v
    }
}
