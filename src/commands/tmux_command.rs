use cmd_builder::Cmd;

use crate::{Error, Tmux, TmuxOutput};
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
///
/// ```text
/// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TmuxCommand<'a>(pub Cmd<'a>);

// XXX: reason?
//macro_rules! tmux_command!("env", "cmd", "-a", "-b", "-arg 0", "param")

pub const TMUX_COMMAND_ARG_SEPARATOR: &str = " ";

impl<'a> fmt::Display for TmuxCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> TmuxCommand<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// set tmux command name
    pub fn name<S: Into<Cow<'a, str>>>(&mut self, name: S) -> &mut Self {
        self.0.name(name);
        self
    }

    /// run tmux command
    pub fn output(&self) -> Result<TmuxOutput, Error> {
        let mut command = Command::from(&self.0);
        // NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
        command.stdin(Stdio::inherit());
        let output = command.output()?;
        Ok(TmuxOutput(output))
    }

    // XXX: really necessary?
    pub fn spawn(&self) -> Result<Child, Error> {
        let mut command = Command::from(&self.0);
        // NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
        command.stdin(Stdio::inherit());
        let child = command.spawn()?;
        Ok(child)
    }

    // XXX: really necessary?
    pub fn status(&self) -> Result<ExitStatus, Error> {
        let mut command = Command::from(&self.0);
        // NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
        command.stdin(Stdio::inherit());
        let status = command.status()?;
        Ok(status)
    }

    pub fn env<T, U>(&mut self, key: T, value: U) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.0.env(key, value);
        self
    }

    pub fn not_combine_short_flags(&mut self) -> &mut Self {
        self.0.not_combine_short_flags();
        self
    }

    pub fn not_use_alias(&mut self) -> &mut Self {
        self.0.not_use_alias();
        self
    }

    // XXX: hard bound to cmd_args
    // if vec doesn't exist, creates it and appends with given arguments
    /// push a single flag (`-x`)
    pub fn push_flag<S: Into<Cow<'a, str>>>(&mut self, flag: S) -> &mut Self {
        self.0.push_flag(flag);
        self
    }

    pub fn push_flag_short(&mut self, flag: char) -> &mut Self {
        self.0.push_flag_short(flag);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// push an option, flag and value (`-x  <VALUE>`)
    pub fn push_option<U, V>(&mut self, key: U, option: V) -> &mut Self
    where
        U: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.0.push_option(key, option);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// push a single parameter (`<VALUE>`)
    pub fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S) -> &mut Self {
        self.0.push_param(param);
        self
    }

    pub fn push_cmd(&mut self, command: TmuxCommand<'a>) -> &mut Self {
        self.0.push_cmd(command.0);
        self
    }

    pub fn push_cmds(&mut self, commands: TmuxCommands<'a>) -> &mut Self {
        self.0.subcommands = Some(commands.0);
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
        self.0.to_vec()
    }

    pub fn to_tmux(self) -> Tmux<'a> {
        Tmux::new().command(self)
    }

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

// create ready to exec `std::process::Command`
// - create `std::process::Command`
// - push binary arguments
// - push command
// - push command args
impl<'a> From<&TmuxCommand<'a>> for Command {
    fn from(tmux_command: &TmuxCommand) -> Self {
        Command::from(&tmux_command.0)
    }
}

// create ready to exec `std::process::Command`
// - create `std::process::Command`
// - push binary arguments
// - push command
// - push command args
impl<'a> From<TmuxCommand<'a>> for Command {
    fn from(tmux_command: TmuxCommand) -> Self {
        Command::from(tmux_command.0)
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

use crate::{HasSession, KillSession, ListPanes, ListSessions, ListWindows, NewSession};

impl<'a> From<NewSession<'a>> for TmuxCommand<'a> {
    fn from(item: NewSession<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<HasSession<'a>> for TmuxCommand<'a> {
    fn from(item: HasSession<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<KillSession<'a>> for TmuxCommand<'a> {
    fn from(item: KillSession<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ListPanes<'a>> for TmuxCommand<'a> {
    fn from(item: ListPanes<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ListWindows<'a>> for TmuxCommand<'a> {
    fn from(item: ListWindows<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ListSessions<'a>> for TmuxCommand<'a> {
    fn from(item: ListSessions<'a>) -> Self {
        item.build()
    }
}
