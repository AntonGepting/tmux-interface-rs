use crate::commands::constants::*;
use crate::{Error, HasSession, KillSession, NewSession, ShowOptions, StartServer, TmuxOutput};
use std::borrow::Cow;
use std::fmt;
use std::process::{Child, Command, ExitStatus, Stdio};

use crate::commands::tmux_bin::TmuxBin;
use crate::commands::tmux_bin_command::TmuxBinCommand;
use crate::commands::tmux_commands::TmuxCommands;

// XXX: environment
/// Standard tmux command line arguments syntax:
/// ```text
/// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// Tmux command line parts:
/// - executable (part I) (example: `tmux`)
/// - executable args (part II) (example: `[-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path]`)
/// - command (part III) (example: `[command]`)
/// - command args (part IV) (example: `[flags]`)
#[derive(Debug, Clone)]
pub struct TmuxCommand<'a> {
    /// Tmux command (part III)
    pub cmd: Option<Cow<'a, str>>,
    /// Tmux command arguments (part IV)
    pub args: Option<Vec<Cow<'a, str>>>,
}

impl<'a> fmt::Display for TmuxCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        if let Some(cmd) = &self.cmd {
            v.push(cmd.as_ref());
        }
        if let Some(args) = &self.args {
            args.iter().for_each(|s| v.push(s.as_ref()));
        }
        let output = v.join(" ");
        write!(f, "{}", output)
    }
}

impl<'a> Default for TmuxCommand<'a> {
    fn default() -> Self {
        TmuxCommand {
            cmd: None,
            args: None,
        }
    }
}

impl<'a> TmuxCommand<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// set tmux command name
    pub fn cmd<S: Into<Cow<'a, str>>>(&mut self, cmd: S) -> &mut Self {
        self.cmd = Some(cmd.into());
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
        self.args.push_param(flag.into());
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert an option, flag and value (`-x  <VALUE>`)
    pub fn push_option<S, U>(&mut self, key: S, option: U) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.args.push_option(key, option);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single parameter (`[VALUE]`)
    pub fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S) -> &mut Self {
        self.args.push_param(param);
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
    pub fn to_vec(&self) -> Vec<&Cow<'a, str>> {
        let mut v = Vec::new();

        if let Some(cmd) = &self.cmd {
            v.push(cmd);
        }

        if let Some(cmd_args) = &self.args {
            v.extend(cmd_args);
        }

        v
    }

    pub fn to_tmux_bin_command(self) -> TmuxBinCommand<'a> {
        self.to_tmux_bin_command_ext(Default::default())
    }

    pub fn to_tmux_bin_command_ext(self, tmux: TmuxBin<'a>) -> TmuxBinCommand<'a> {
        TmuxBinCommand {
            tmux: tmux,
            command: self,
        }
    }

    pub fn append_to(self, cmds: &mut TmuxCommands<'a>) {
        cmds.push(self);
    }
}

// create ready to exec `std::process::Command`
// - create `std::process::Command`
// - push binary arguments
// - push command
// - push command args
impl<'a> From<TmuxCommand<'a>> for Command {
    fn from(tmux: TmuxCommand) -> Self {
        Command::from(&tmux)
    }
}

// create ready to exec `std::process::Command`
// - create `std::process::Command`
// - push binary arguments
// - push command
// - push command args
impl<'a> From<&TmuxCommand<'a>> for Command {
    fn from(tmux: &TmuxCommand) -> Self {
        let mut command = Command::new(TMUX);

        //// XXX: ugly?
        //if let Some(v) = &tmux.bin_args {
        //for a in v {
        //command.arg(a.as_ref());
        //}
        //}

        if let Some(s) = &tmux.cmd {
            command.arg(s.as_ref());
        }

        // XXX: ugly?
        if let Some(v) = &tmux.args {
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

impl<'a> From<NewSession<'a>> for TmuxCommand<'a> {
    fn from(item: NewSession<'a>) -> Self {
        item.0
    }
}

//impl<'a> From<&mut NewSession<'a>> for TmuxCommand<'a> {
//fn from(item: &mut NewSession<'a>) -> Self {
//item.0.clone()
//}
//}

impl<'a> From<&'a NewSession<'a>> for &TmuxCommand<'a> {
    fn from(item: &'a NewSession<'a>) -> Self {
        &item.0
    }
}

impl<'a> From<&mut NewSession<'a>> for TmuxCommand<'a> {
    fn from(item: &mut NewSession<'a>) -> Self {
        item.0.clone()
    }
}

impl<'a> From<ShowOptions<'a>> for TmuxCommand<'a> {
    fn from(item: ShowOptions<'a>) -> Self {
        item.0
    }
}

impl<'a> From<&'a ShowOptions<'a>> for &TmuxCommand<'a> {
    fn from(item: &'a ShowOptions<'a>) -> Self {
        &item.0
    }
}

impl<'a> From<&mut ShowOptions<'a>> for TmuxCommand<'a> {
    fn from(item: &mut ShowOptions<'a>) -> Self {
        item.0.clone()
    }
}

impl<'a> From<&'a HasSession<'a>> for &'a TmuxCommand<'a> {
    fn from(item: &'a HasSession<'a>) -> Self {
        &item.0
    }
}

impl<'a> From<&mut HasSession<'a>> for TmuxCommand<'a> {
    fn from(item: &mut HasSession<'a>) -> Self {
        item.0.clone()
    }
}

impl<'a> From<&'a KillSession<'a>> for &'a TmuxCommand<'a> {
    fn from(item: &'a KillSession<'a>) -> Self {
        &item.0
    }
}

impl<'a> From<&mut KillSession<'a>> for TmuxCommand<'a> {
    fn from(item: &mut KillSession<'a>) -> Self {
        item.0.clone()
    }
}

impl<'a> From<StartServer<'a>> for TmuxCommand<'a> {
    fn from(item: StartServer<'a>) -> Self {
        item.0
    }
}
