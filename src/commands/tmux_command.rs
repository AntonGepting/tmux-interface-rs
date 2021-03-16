use crate::commands::NEW_SESSION;
use crate::NewSession;
use crate::TmuxOutput;
use std::process::{Command, Stdio};

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
#[derive(Default, Debug, Clone)]
pub struct TmuxCommand {
    // XXX: rename tmux?
    /// Tmux executable name, (part I) if is `None`, will be used `tmux`
    pub bin: Option<String>,
    /// Tmux executable arguments (part II)
    pub bin_args: Option<Vec<String>>,
    /// Tmux command (part III)
    pub cmd: Option<String>,
    /// Tmux command arguments (part IV)
    pub cmd_args: Option<Vec<String>>,
}

//pub trait MyCommand {
//fn set_bin<S: Into<Cow<'a, str>>>(&mut self, bin: S) -> &mut Self;
//fn get_bin(&self) -> Option<&Cow<'a, str>>;

//fn exec(&self) -> TmuxOutput {
//let tmux_bin = self.get_bin().unwrap_or(&Cow::Borrowed(TmuxCommand::TMUX));
//let mut command = Command::new(&**tmux_bin);
//let output = command.output().unwrap();
//println!("{:?}", output);
//TmuxOutput(output)
//}
//}

impl TmuxCommand {
    const TMUX: &'static str = "tmux";

    //pub fn create(
    //bin: Option<Cow<'a, str>>,
    //bin_args: Option<Vec<Cow<'a, str>>>,
    //cmd: Option<Cow<'a, str>>,
    //cmd_args: Option<Vec<Cow<'a, str>>>,
    //) -> Self {
    //TmuxCommand {
    //bin: bin.clone(),
    //bin_args: bin_args.clone(),
    //cmd: cmd.clone(),
    //cmd_args: cmd_args.clone(),
    //}
    //}

    pub fn bin<S: Into<String>>(&mut self, bin: S) -> &mut Self {
        //self.tmux.bin = bin;
        self.bin = Some(bin.into());
        self
    }

    pub fn cmd<S: Into<String>>(&mut self, cmd: S) -> &mut Self {
        self.cmd = Some(cmd.into());
        self
    }

    // if we are working with same type problems multiple traits methods mixing allowed (NewSession, DetachClient, chaining methods)
    //pub fn new_session(&mut self) -> &mut Self {
    //self.cmd = Some(Cow::Borrowed(<TmuxCommand as NewSession>::NEW_SESSION));
    //self
    //}

    // example
    //pub fn new_session(&mut self) -> impl NewSession {
    //<TmuxCommand as NewSession>::new()
    //}

    // clone
    //pub fn new_session(&self) -> impl NewSession {
    //<TmuxCommand as NewSession>::clone_from(self)
    //}
    pub fn new_session(&mut self) -> &mut Self
    where
        Self: NewSession,
    {
        //self.cmd = Some(Cow::Borrowed(<TmuxCommand as NewSession>::NEW_SESSION));
        self.cmd(NEW_SESSION)
    }

    //// NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
    //cmd.stdin(Stdio::inherit());
    /// run command
    pub fn exec(&self) -> TmuxOutput {
        let tmux_bin = match &self.bin {
            Some(s) => s.into(),
            None => TmuxCommand::TMUX.to_string(),
        };
        let mut command = Command::new(tmux_bin);

        // XXX: ugly?
        if let Some(s) = &self.bin_args {
            for a in s {
                command.arg(&**a);
            }
        }
        if let Some(s) = &self.bin_args {
            command.args(s);
        }

        if let Some(s) = &self.cmd {
            command.arg(&**s);
        }

        // XXX: ugly?
        if let Some(s) = &self.cmd_args {
            for a in s {
                command.arg(&**a);
            }
        }

        println!("{:?}", &self);
        command.stdin(Stdio::inherit());
        let output = command.output().unwrap();
        println!("{:?}", output);
        TmuxOutput(output)
    }

    // XXX: hard bound to cmd_args
    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single flag (`-x`)
    pub fn push_flag<S: Into<String>>(&mut self, flag: S) -> &mut Self {
        self.push_param(flag.into())
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert an option, flag and value (`-x  <VALUE>`)
    pub fn push_option<S, U>(&mut self, key: S, option: U) -> &mut Self
    where
        S: Into<String>,
        U: Into<String>,
    {
        self.cmd_args
            .get_or_insert(Vec::new())
            .extend_from_slice(&[key.into(), option.into()]);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single parameter (`[VALUE]`)
    pub fn push_param<S: Into<String>>(&mut self, param: S) -> &mut Self {
        self.cmd_args.get_or_insert(Vec::new()).push(param.into());
        self
    }

    pub fn new() -> Self {
        Default::default()
    }
}

pub trait TmuxCommandTrait: std::fmt::Debug {
    fn push_flag<S: Into<String>>(&mut self, flag: S) -> &mut Self;
    fn push_option<S, U>(&mut self, key: S, option: U) -> &mut Self
    where
        S: Into<String>,
        U: Into<String>;
    fn push_param<S: Into<String>>(&mut self, param: S) -> &mut Self;

    fn bin<S: Into<String>>(&mut self, bin: S) -> &mut Self;

    fn exec(&self) {
        dbg!(self);
    }
}

impl TmuxCommandTrait for TmuxCommand {
    fn bin<S: Into<String>>(&mut self, bin: S) -> &mut Self {
        self.bin = Some(bin.into());
        self
    }

    fn push_flag<S: Into<String>>(&mut self, flag: S) -> &mut Self {
        self.push_flag(flag);
        self
    }

    fn push_option<S, U>(&mut self, key: S, option: U) -> &mut Self
    where
        S: Into<String>,
        U: Into<String>,
    {
        self.push_option(key, option);
        self
    }

    fn push_param<S: Into<String>>(&mut self, param: S) -> &mut Self {
        self.push_param(param);
        self
    }
}
