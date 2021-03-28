use crate::commands::constants::TMUX;
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

    pub fn bin<S: Into<Cow<'a, str>>>(&mut self, bin: S) -> &mut Self {
        //self.tmux.bin = bin;
        self.bin = bin.into();
        self
    }

    pub fn cmd<S: Into<Cow<'a, str>>>(&mut self, cmd: S) -> &mut Self {
        self.cmd = Some(cmd.into());
        self
    }

    /// run command
    pub fn output(&self) -> Result<TmuxOutput, Error> {
        let mut command = self.push_tmux_command();
        // NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
        command.stdin(Stdio::inherit());
        let output = command.output()?;
        Ok(TmuxOutput(output))
    }

    pub fn push_tmux_command(&self) -> Command {
        let mut command = Command::new(&self.bin.as_ref());

        // XXX: ugly?
        if let Some(s) = &self.bin_args {
            for a in s {
                command.arg(a.as_ref());
            }
        }

        if let Some(s) = &self.cmd {
            command.arg(s.as_ref());
        }

        // XXX: ugly?
        if let Some(s) = &self.cmd_args {
            for a in s {
                command.arg(a.as_ref());
            }
        }

        command
    }

    pub fn spawn(&self) -> Result<Child, Error> {
        let mut command = self.push_tmux_command();
        Ok(command.spawn()?)
    }

    pub fn status(&self) -> Result<ExitStatus, Error> {
        let mut command = self.push_tmux_command();
        Ok(command.status()?)
    }

    // XXX: hard bound to cmd_args
    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single flag (`-x`)
    pub fn push_flag<S: Into<Cow<'a, str>>>(&mut self, flag: S) -> &mut Self {
        self.push_param(flag.into())
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert an option, flag and value (`-x  <VALUE>`)
    pub fn push_option<S, U>(&mut self, key: S, option: U) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.cmd_args
            .get_or_insert(Vec::new())
            .extend_from_slice(&[key.into(), option.into()]);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single parameter (`[VALUE]`)
    pub fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S) -> &mut Self {
        self.cmd_args.get_or_insert(Vec::new()).push(param.into());
        self
    }

    // TODO: custom command
    //pub fn custom<S: Into<Cow<'a, str>>>(&self, ) -> &mut Self {
    //}

    //pub fn output(&self) -> Ou {
    //}

    // prepared for run as std::process::Command()
    pub fn to_command(&self) -> (String, Vec<String>) {
        let bin = self.bin.to_string();

        let args = Vec::new();
        //if let Some(v) = &self.bin_args {
        //args.extend_from_slice(v.borrow());
        //}

        //if let Some(s) = &self.cmd {
        //args.push(s.to_string());
        //}

        //if let Some(v) = &self.cmd_args {
        //args.extend_from_slice(v);
        //}

        (bin, args)
    }
}
