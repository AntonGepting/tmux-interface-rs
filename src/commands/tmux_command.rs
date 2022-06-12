use crate::{Error, TmuxOutput};
use std::borrow::Cow;
use std::fmt;
use std::process::{Child, Command, ExitStatus, Stdio};

//use crate::commands::tmux_bin::TmuxBin;
//use crate::commands::tmux_bin_command::TmuxBinCommand;
use crate::commands::tmux_commands::TmuxCommands;

// XXX: mb enum for command?
// XXX: rename (.cmd to .name)?
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TmuxCommand<'a> {
    /// environment variables
    pub envs: Option<Vec<(Cow<'a, str>, Cow<'a, str>)>>,
    /// Tmux command (part III) (`[new-session]`)
    // XXX: rename command name
    pub name: Option<Cow<'a, str>>,
    /// command alias (`new-session` -> `new`)
    pub alias: Option<Cow<'a, str>>,
    /// Tmux command flags (`[-a] [-b] [-c]`)
    pub flags: Option<Vec<Cow<'a, str>>>,
    pub flags_short: Option<String>,
    /// Tmux command arguments (part IV) (`[-n session-name] [shell-command]`)
    pub args: Option<Vec<Cow<'a, str>>>,
    /// subcommand
    pub cmds: Option<TmuxCommands<'a>>,
    /// separator (" ")
    pub separator: Option<&'a str>,
    // `-f -a` = `-fa`
    pub not_combine_short_flags: bool,
    // `new-session` = `new`
    pub not_use_alias: bool,
}

// XXX: reason?
//macro_rules! tmux_command!("env", "cmd", "-a", "-b", "-arg 0", "param")

pub const TMUX_COMMAND_ARG_SEPARATOR: &str = " ";

impl<'a> fmt::Display for TmuxCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self
            .to_vec()
            .join(self.separator.unwrap_or(TMUX_COMMAND_ARG_SEPARATOR));
        write!(f, "{}", output)
    }
}

impl<'a> TmuxCommand<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// set tmux command name
    pub fn cmd<S: Into<Cow<'a, str>>>(&mut self, cmd: S) -> &mut Self {
        self.name = Some(cmd.into());
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

    pub fn env<T, U>(&mut self, key: T, value: U) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.envs
            .get_or_insert(Vec::new())
            .push((key.into(), value.into()));
        self
    }

    // XXX: hard bound to cmd_args
    // if vec doesn't exist, creates it and appends with given arguments
    /// push a single flag (`-x`)
    pub fn push_flag<S: Into<Cow<'a, str>>>(&mut self, flag: S) -> &mut Self {
        self.args.get_or_insert(Vec::new()).push(flag.into());
        self
    }

    pub fn push_flag_short(&mut self, flag: char) -> &mut Self {
        self.flags_short.get_or_insert(String::new()).push(flag);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// push an option, flag and value (`-x  <VALUE>`)
    pub fn push_option<U, V>(&mut self, key: U, option: V) -> &mut Self
    where
        U: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.args
            .get_or_insert(Vec::new())
            .extend_from_slice(&[key.into(), option.into()]);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// push a single parameter (`<VALUE>`)
    pub fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S) -> &mut Self {
        self.args.get_or_insert(Vec::new()).push(param.into());
        self
    }

    pub fn push_cmd(&mut self, command: TmuxCommand<'a>) -> &mut Self {
        self.cmds.get_or_insert(TmuxCommands::new()).push(command);
        self
    }

    pub fn push_cmds(&mut self, commands: TmuxCommands<'a>) -> &mut Self {
        self.cmds = Some(commands);
        self
    }

    // TODO: custom command
    //pub fn custom<S: Into<Cow<'a, str>>>(&self, ) -> &mut Self {
    //}

    //// create `std::process::Command` from `Self` (consuming `Self`)
    //pub fn to_command(&self) -> Command {
    //Command::from(self)
    //}

    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        let mut v: Vec<Cow<'a, str>> = Vec::new();

        if let Some(envs) = &self.envs {
            for (key, value) in envs {
                v.push(Cow::Owned(format!("{}={}", key, value)));
            }
        }

        if let Some(cmd) = &self.name {
            v.push(cmd.to_owned());
        }

        if let Some(flags_short) = &self.flags_short {
            if self.not_combine_short_flags {
                for c in flags_short.chars() {
                    v.push(Cow::Owned(format!("-{}", c)));
                }
            } else {
                v.push(Cow::Owned(format!("-{}", flags_short)));
            }
        }

        if let Some(args) = &self.args {
            v.extend(args.to_vec());
        }

        if let Some(cmds) = &self.cmds {
            v.extend(cmds.to_vec());
        }

        v
    }

    //pub fn into_tmux_bin_command(self) -> TmuxBinCommand<'a> {
    //self.into_tmux_bin_command_ext(Default::default())
    //}

    //pub fn into_tmux_command(self) -> TmuxCommand<'a> {
    //TmuxCommand::default()
    //}

    //pub fn into_tmux_bin_command_ext(self, tmux: TmuxBin<'a>) -> TmuxBinCommand<'a> {
    //TmuxBinCommand {
    //tmux: tmux,
    //command: self,
    //}
    //}

    //pub fn append_to(self, cmds: &mut TmuxCommands<'a>) {
    //cmds.push(self);
    //}

    //pub fn writeln(self, stdin: &mut ChildStdin) -> Result<(), std::io::Error> {
    //writeln!(stdin, "{}", self.to_string())
    //}
}

const EMPTY_COMMAND: &str = "";

// create ready to exec `std::process::Command`
// - create `std::process::Command`
// - push binary arguments
// - push command
// - push command args
impl<'a> From<&TmuxCommand<'a>> for Command {
    fn from(tmux_command: &TmuxCommand) -> Self {
        // user given command or blank command
        let name = tmux_command
            .name
            .as_ref()
            .unwrap_or(&Cow::Borrowed(EMPTY_COMMAND));
        let mut command = Command::new(name.as_ref());

        // environment variables
        if let Some(envs) = &tmux_command.envs {
            command.envs(
                envs.iter()
                    .map(|(key, value)| (key.as_ref(), value.as_ref())),
            );
        }

        // arguments
        if let Some(args) = &tmux_command.args {
            command.args(args.iter().map(|arg| arg.as_ref()));
        }

        // subcommands
        if let Some(cmds) = &tmux_command.cmds {
            command.args(cmds.to_vec().iter().map(|arg| arg.as_ref()));
        }

        command
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

// trait used for bin_args and cmd_args
//pub trait Args<'a> {
//fn push_param<S: AsRef<str>>(&mut self, param: S);
//fn push_option<S, U>(&mut self, key: S, option: U)
//where
//S: AsRef<str>,
//U: AsRef<str>;
//fn push_flag<S: AsRef<str>>(&mut self, flag: S);
//}

// trait used for bin_args and cmd_args
//impl<'a> Args<'a> for Option<Vec<Cow<'a, str>>> {
//fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S) {
//self.get_or_insert(Vec::new()).push(param.into());
//}

//fn push_option<S, U>(&mut self, key: S, option: U)
//where
//S: AsRef<Cow<'a, str>>,
//U: Into<Cow<'a, str>>,
//{
//self.get_or_insert(Vec::new())
//.extend_from_slice(&[key.into(), option.into()]);
//}

//fn push_flag<S: Into<Cow<'a, str>>>(&mut self, flag: S) {
//self.push_param(flag.into());
//}
//}

////impl<'a> From<&mut NewSession<'a>> for TmuxCommand<'a> {
////fn from(item: &mut NewSession<'a>) -> Self {
////item.0.clone()
////}
////}

//impl<'a> From<&'a NewSession<'a>> for &TmuxCommand<'a> {
//fn from(item: &'a NewSession<'a>) -> Self {
//&item.0
//}
//}

//impl<'a> From<&mut NewSession<'a>> for TmuxCommand<'a> {
//fn from(item: &mut NewSession<'a>) -> Self {
//item.0.clone()
//}
//}

//impl<'a> From<ShowOptions<'a>> for TmuxCommand<'a> {
//fn from(item: ShowOptions<'a>) -> Self {
//item.0
//}
//}

//impl<'a> From<&'a ShowOptions<'a>> for &TmuxCommand<'a> {
//fn from(item: &'a ShowOptions<'a>) -> Self {
//&item.0
//}
//}

//impl<'a> From<&mut ShowOptions<'a>> for TmuxCommand<'a> {
//fn from(item: &mut ShowOptions<'a>) -> Self {
//item.0.clone()
//}
//}

//impl<'a> From<&'a HasSession<'a>> for &'a TmuxCommand<'a> {
//fn from(item: &'a HasSession<'a>) -> Self {
//&item.0
//}
//}

//impl<'a> From<&mut HasSession<'a>> for TmuxCommand<'a> {
//fn from(item: &mut HasSession<'a>) -> Self {
//item.0.clone()
//}
//}

//impl<'a> From<&'a KillSession<'a>> for &'a TmuxCommand<'a> {
//fn from(item: &'a KillSession<'a>) -> Self {
//&item.0
//}
//}

//impl<'a> From<&mut KillSession<'a>> for TmuxCommand<'a> {
//fn from(item: &mut KillSession<'a>) -> Self {
//item.0.clone()
//}
//}

//impl<'a> From<StartServer> for TmuxCommand<'a> {
//fn from(item: StartServer<'a>) -> Self {
//item.0
//}
//}
//
//
//
//    /// Returns mutable reference to tmux executable name `Cow<'a, str>`
/* NOTE: from bin
    ///
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use tmux_interface::commands::tmux_bin::TmuxBin;
    ///
    /// let mut tmux = TmuxBin::default();
    /// let bin = tmux.bin();
    /// assert_eq!(bin, &Cow::Borrowed("tmux"));
    /// ```
    pub fn bin(&self) -> &Cow<'a, str> {
        &self.bin
    }

    /// Returns mutable reference to tmux executable name `Cow<'a, str>`
    ///
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use tmux_interface::commands::tmux_bin::TmuxBin;
    ///
    /// let mut tmux = TmuxBin::default();
    /// *tmux.bin_mut() = Cow::Borrowed("/usr/bin/tmux");
    /// assert_eq!(tmux.bin, Cow::Borrowed("/usr/bin/tmux"));
    /// ```
    /// or
    /// ```
    /// use std::borrow::Cow;
    /// use tmux_interface::commands::tmux_bin::TmuxBin;
    ///
    /// let mut tmux = TmuxBin::default();
    /// *tmux.bin_mut() = "/usr/bin/tmux".into();
    /// assert_eq!(tmux.bin, Cow::Borrowed("/usr/bin/tmux"));
    /// ```
    pub fn bin_mut(&mut self) -> &mut Cow<'a, str> {
        &mut self.bin
    }

*/
