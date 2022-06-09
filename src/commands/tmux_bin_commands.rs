//use crate::commands::constants::*;
use crate::commands::tmux_bin::TmuxBin;
use crate::commands::tmux_commands::TmuxCommands;
use crate::{Error, TmuxCommand, TmuxOutput};
//use std::borrow::Cow;
use std::fmt;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct TmuxBinCommands<'a> {
    pub tmux: TmuxBin<'a>,
    pub cmds: TmuxCommands<'a>,
}

impl<'a> fmt::Display for TmuxBinCommands<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.tmux.to_string(), self.cmds.to_string())
    }
}

impl<'a> Default for TmuxBinCommands<'a> {
    fn default() -> Self {
        Self {
            tmux: TmuxBin::default(),
            cmds: TmuxCommands::default(),
        }
    }
}

impl<'a> TmuxBinCommands<'a> {
    pub fn new(tmux: TmuxBin<'a>, cmds: TmuxCommands<'a>) -> Self {
        TmuxBinCommands { tmux, cmds }
    }

    pub fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T) {
        self.cmds.push(cmd.into());
    }

    // XXX: error out
    pub fn output(&self) -> Result<TmuxOutput, Error> {
        let mut command = Command::new(&self.tmux.bin.as_ref());

        for tmux_command in &self.cmds.0 {
            if let Some(cmd) = &tmux_command.cmd {
                command.arg(cmd.as_ref());
            }

            if let Some(args) = &tmux_command.args {
                for arg in args {
                    command.arg(arg.as_ref());
                }
            }

            command.arg(";");
        }

        // NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
        command.stdin(Stdio::inherit());

        let output = command.output()?;

        Ok(TmuxOutput(output))
    }
}

// XXX: clone
impl<'a> From<&TmuxCommands<'a>> for TmuxBinCommands<'a> {
    fn from(item: &TmuxCommands<'a>) -> Self {
        Self {
            tmux: Default::default(),
            cmds: item.clone(),
        }
    }
}

impl<'a> From<TmuxCommands<'a>> for TmuxBinCommands<'a> {
    fn from(item: TmuxCommands<'a>) -> Self {
        Self {
            tmux: Default::default(),
            cmds: item,
        }
    }
}

pub trait Build<'a> {
    fn build(&self) -> TmuxCommand<'a>;
}
