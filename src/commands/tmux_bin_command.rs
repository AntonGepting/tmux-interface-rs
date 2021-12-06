use crate::commands::constants::*;
use crate::commands::tmux_bin::TmuxBin;
use crate::commands::tmux_command::TmuxCommand;
use crate::{Error, TmuxOutput};
use std::borrow::Cow;
use std::fmt;
use std::process::{Child, Command, ExitStatus, Stdio};

// XXX: environment
/// Standard tmux command line arguments syntax:
/// ```text
/// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
#[derive(Debug, Clone)]
pub struct TmuxBinCommand<'a> {
    pub tmux: TmuxBin<'a>,
    pub command: TmuxCommand<'a>,
}

impl<'a> fmt::Display for TmuxBinCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        v.push(self.tmux.bin.as_ref());
        if let Some(bin_args) = &self.tmux.args {
            bin_args.iter().for_each(|s| v.push(s.as_ref()));
        }
        if let Some(cmd) = &self.command.cmd {
            v.push(&cmd);
        }
        if let Some(cmd_args) = &self.command.args {
            cmd_args.iter().for_each(|s| v.push(s.as_ref()));
        }
        let output = v.join(" ");
        write!(f, "{}", output)
    }
}

impl<'a> Default for TmuxBinCommand<'a> {
    fn default() -> Self {
        TmuxBinCommand {
            tmux: TmuxBin::default(),
            command: TmuxCommand::default(),
        }
    }
}

impl<'a> TmuxBinCommand<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn tmux<S: Into<Cow<'a, str>>>(&mut self, tmux: TmuxBin<'a>) -> &mut Self {
        self.tmux = tmux;
        self
    }

    /// set tmux command name
    pub fn cmd(&mut self, cmd: TmuxCommand<'a>) -> &mut Self {
        self.command = cmd;
        self
    }

    /// run tmux command
    pub fn output(&self) -> Result<TmuxOutput, Error> {
        let mut command = Command::from(self);
        // NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
        command.stdin(Stdio::inherit());
        let output = command.output()?;
        Ok(TmuxOutput(output))
    }

    // XXX: really necessary?
    pub fn spawn(&self) -> Result<Child, Error> {
        let mut command = Command::from(self);
        Ok(command.spawn()?)
    }

    // XXX: really necessary?
    pub fn status(&self) -> Result<ExitStatus, Error> {
        let mut command = Command::from(self);
        Ok(command.status()?)
    }

    // XXX: hard bound to cmd_args
    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single flag (`-x`)
    pub fn push_flag<S: Into<Cow<'a, str>>>(&mut self, flag: S) -> &mut Self {
        self.command.push_flag(flag);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert an option, flag and value (`-x  <VALUE>`)
    pub fn push_option<S, U>(&mut self, key: S, option: U) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.command.push_option(key, option);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single parameter (`[VALUE]`)
    pub fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S) -> &mut Self {
        self.command.push_param(param);
        self
    }

    // TODO: custom command
    //pub fn custom<S: Into<Cow<'a, str>>>(&self, ) -> &mut Self {
    //}

    //// create `std::process::Command` from `Self` (consuming `Self`)
    //pub fn to_command(&self) -> Command {
    //Command::from(self)
    //}

    //pub fn to_string(&self) -> String {
    //let mut s = String::new();
    //self.cmd.as_ref().and_then(|cmd| Some(s.push_str(cmd)));
    //s.push(' ');
    //self.cmd_args
    //.as_ref()
    //.and_then(|cmd_args| Some(s.push_str(&cmd_args.join(" "))));
    //s
    //}
}

// create ready to exec `std::process::Command`
// - create `std::process::Command`
// - push binary arguments
// - push command
// - push command args
impl<'a> From<TmuxBinCommand<'a>> for Command {
    fn from(tmux_bin_command: TmuxBinCommand) -> Self {
        Command::from(&tmux_bin_command)
    }
}

// create ready to exec `std::process::Command`
// - create `std::process::Command`
// - push binary arguments
// - push command
// - push command args
impl<'a> From<&TmuxBinCommand<'a>> for Command {
    fn from(tmux_bin_command: &TmuxBinCommand) -> Self {
        let mut command = Command::new(tmux_bin_command.tmux.bin.as_ref());

        // XXX: ugly?
        if let Some(v) = &tmux_bin_command.tmux.args {
            for a in v {
                command.arg(a.as_ref());
            }
        }

        if let Some(s) = &tmux_bin_command.command.cmd {
            command.arg(s.as_ref());
        }

        // XXX: ugly?
        if let Some(v) = &tmux_bin_command.command.args {
            for a in v {
                command.arg(a.as_ref());
            }
        }

        command
    }
}

// trait used for bin_args and cmd_args
pub trait Args<'a> {
    fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S);
    fn push_option<S, U>(&mut self, key: S, option: U)
    where
        S: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>;
    fn push_flag<S: Into<Cow<'a, str>>>(&mut self, flag: S);
}

// trait used for bin_args and cmd_args
impl<'a> Args<'a> for Option<Vec<Cow<'a, str>>> {
    fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S) {
        self.get_or_insert(Vec::new()).push(param.into());
    }

    fn push_option<S, U>(&mut self, key: S, option: U)
    where
        S: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.get_or_insert(Vec::new())
            .extend_from_slice(&[key.into(), option.into()]);
    }

    fn push_flag<S: Into<Cow<'a, str>>>(&mut self, flag: S) {
        self.push_param(flag.into());
    }
}
