use crate::commands::constants::*;
use crate::commands::tmux_bin::TmuxBin;
use crate::commands::tmux_commands::TmuxCommands;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct TmuxBinCommands<'a> {
    pub tmux: TmuxBin<'a>,
    pub commands: TmuxCommands<'a>,
}

impl<'a> Default for TmuxBinCommands<'a> {
    fn default() -> Self {
        Self {
            tmux: TmuxBin::default(),
            commands: TmuxCommands::default(),
        }
    }
}

impl<'a> TmuxBinCommands<'a> {
    pub fn new(tmux: TmuxBin<'a>, commands: TmuxCommands<'a>) -> Self {
        TmuxBinCommands { tmux, commands }
    }

    pub fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T) {
        self.commands.push(cmd.into());
    }

    //pub fn append(&mut self) -> &mut Self {
    //self.commands.append();
    //self
    //}

    //pub fn to_vec(&self) -> Vec<&Cow<'a, str>> {
    //let mut v = Vec::new();

    ////let c = self.commands.to_vec();

    //for c in &self.commands.0 {
    //v.extend(c.to_vec());
    //}

    ////if let Some(tmux_args) = &self.tmux.args {
    ////v.extend(tmux_args);
    ////}

    //v
    //}

    //pub fn to_string(&self) -> String {
    //}

    //pub fn start_server(&mut self) -> StartServer<'a> {
    //StartServer::new()
    //}

    //pub fn new_session(&mut self) -> NewSession<'a> {
    //NewSession::new()
    //}

    //pub fn show_options(&mut self) -> ShowOptions<'a> {
    //ShowOptions::new()
    //}

    // XXX: error out
    pub fn output(&self) -> Result<TmuxOutput, Error> {
        let mut command = Command::new(&self.tmux.bin.as_ref());

        for tmux_command in &self.commands.0 {
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
            commands: item.clone(),
        }
    }
}

impl<'a> From<TmuxCommands<'a>> for TmuxBinCommands<'a> {
    fn from(item: TmuxCommands<'a>) -> Self {
        Self {
            tmux: Default::default(),
            commands: item,
        }
    }
}
