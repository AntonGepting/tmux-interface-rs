use crate::commands::constants::*;
use crate::{Error, TmuxOutput};
use std::borrow::Cow;
use std::process::{Child, Command, ExitStatus, Stdio};

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
    // XXX: rename tmux?
    /// Tmux executable name, (part I)
    pub bin: Cow<'a, str>,
    /// Tmux executable arguments (part II)
    pub bin_args: Option<Vec<Cow<'a, str>>>,
    /// Tmux command (part III)
    pub cmd: Option<Cow<'a, str>>,
    /// Tmux command arguments (part IV)
    pub cmd_args: Option<Vec<Cow<'a, str>>>,
}

impl<'a> Default for TmuxCommand<'a> {
    fn default() -> Self {
        TmuxCommand {
            bin: Cow::Borrowed(TMUX),
            bin_args: None,
            cmd: None,
            cmd_args: None,
        }
    }
}

impl<'a> TmuxCommand<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// set tmux binary name
    pub fn bin<S: Into<Cow<'a, str>>>(&mut self, bin: S) -> &mut Self {
        //self.tmux.bin = bin;
        self.bin = bin.into();
        self
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
        self.cmd_args.push_param(flag.into());
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert an option, flag and value (`-x  <VALUE>`)
    pub fn push_option<S, U>(&mut self, key: S, option: U) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.cmd_args.push_option(key, option);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single parameter (`[VALUE]`)
    pub fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S) -> &mut Self {
        self.cmd_args.push_param(param);
        self
    }

    // TODO: custom command
    //pub fn custom<S: Into<Cow<'a, str>>>(&self, ) -> &mut Self {
    //}

    //// create `std::process::Command` from `Self` (consuming `Self`)
    //pub fn to_command(&self) -> Command {
    //Command::from(self)
    //}
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
        let mut command = Command::new(tmux.bin.as_ref());

        // XXX: ugly?
        if let Some(v) = &tmux.bin_args {
            for a in v {
                command.arg(a.as_ref());
            }
        }

        if let Some(s) = &tmux.cmd {
            command.arg(s.as_ref());
        }

        // XXX: ugly?
        if let Some(v) = &tmux.cmd_args {
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
