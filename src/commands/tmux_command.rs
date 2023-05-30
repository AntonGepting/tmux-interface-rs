use crate::{Tmux, TmuxCommands};
use std::borrow::Cow;
use std::fmt;
use std::process::Command;

// XXX: cmd, command, tmux command all proper names in methods, fields
// XXX: mb enum for command?
// XXX: rename (.cmd to .name)?
// XXX: environment
/// Standard tmux command line arguments syntax:
///
/// ```text
/// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TmuxCommand<'a> {
    /// environment variables
    pub envs: Option<Vec<(Cow<'a, str>, Cow<'a, str>)>>,

    /// command name
    pub name: Option<Cow<'a, str>>,

    /// command alias
    pub alias: Option<Cow<'a, str>>,

    // XXX: remove
    /// flags (`[-a] [-b] [-c]`)
    pub flags: Option<Vec<Cow<'a, str>>>,

    /// short flags (`[-a] [-b] [-c]`)
    pub flags_short: Option<String>,

    /// arguments: long flags, options, parameters (`[--longflag] [-o opt] [param]`)
    pub args: Option<Vec<Cow<'a, str>>>,

    /// subcommands list
    pub subcommands: Option<TmuxCommands<'a>>,

    // XXX: Cow<'a, str> or &'a str?
    /// separator between command and it's flags, args, subcommand (" ")
    pub separator: Option<&'a str>,

    // XXX: Cow<'a, str> or &'a str?
    /// flags, args separator (usually double dash `--`)
    pub flags_args_separator: Option<&'a str>,

    /// combine multiple single flags into flags line (`-f -a` = `-fa`)
    pub combine_short_flags: bool,

    /// use command alias instead of name (`new-session` = `new`)
    pub use_alias: bool,
}

// XXX: reason?
//macro_rules! tmux_command!("env", "cmd", "-a", "-b", "-arg 0", "param")

pub const EMPTY_CMD: &str = "";
pub const TMUX_COMMAND_ARG_SEPARATOR: &str = " ";

impl<'a> Default for TmuxCommand<'a> {
    fn default() -> Self {
        Self {
            envs: None,
            name: None,
            alias: None,
            flags: None,
            flags_short: None,
            args: None,
            subcommands: None,
            separator: None,
            flags_args_separator: None,
            combine_short_flags: true,
            use_alias: true,
        }
    }
}

// XXX: reason?
// s. clap
//macro_rules! tmux_command!("env", "cmd", "-a", "-b", "-arg 0", "param")

impl<'a> fmt::Display for TmuxCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self
            .to_vec()
            .join(self.separator.unwrap_or(TMUX_COMMAND_ARG_SEPARATOR));
        write!(f, "{}", output)
    }
}

impl<'a> TmuxCommand<'a> {
    /// Create new `Cmd` structure (using `default()` method)
    pub fn new() -> Self {
        Default::default()
    }

    /// Create new `Cmd` structure, initialize both `not_combine_short_flags` and `not_use_alias`
    /// fields with `true`. Command will not combine flags (separate flags will be used instead),
    /// and not use alias (command name will be used instead)
    pub fn new_full<S: Into<Cow<'a, str>>>(name: S) -> Self {
        Self {
            name: Some(name.into()),
            combine_short_flags: false,
            use_alias: false,
            ..Default::default()
        }
    }

    /// Create and set `name` field
    pub fn with_name<S: Into<Cow<'a, str>>>(name: S) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// Create and set `alias` field
    pub fn with_alias<S: Into<Cow<'a, str>>>(alias: S) -> Self {
        Self {
            alias: Some(alias.into()),
            ..Default::default()
        }
    }

    pub fn with_cmds(cmd_list: TmuxCommands<'a>) -> Self {
        Self {
            subcommands: Some(cmd_list),
            ..Default::default()
        }
    }

    /// Set `Cmd.name` field
    pub fn name<S: Into<Cow<'a, str>>>(&mut self, cmd: S) -> &mut Self {
        self.name = Some(cmd.into());
        self
    }

    /// Set `Cmd.alias` field
    pub fn alias<S: Into<Cow<'a, str>>>(&mut self, alias: S) -> &mut Self {
        self.alias = Some(alias.into());
        self
    }

    /// Add an environment variable to `Cmd.env`
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

    // XXX: rename subcmd?
    pub fn push_cmd(&mut self, cmd: TmuxCommand<'a>) -> &mut Self {
        self.subcommands
            .get_or_insert(TmuxCommands::new())
            .push(cmd);
        self
    }

    // XXX: rename subcmd?
    pub fn push_cmds(&mut self, cmd_list: TmuxCommands<'a>) -> &mut Self {
        self.subcommands = Some(cmd_list);
        self
    }

    pub fn arg<T, U>(&mut self, flag: T, opt: U) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        let v = self.args.get_or_insert(Vec::new());
        v.push(flag.into());
        v.push(opt.into());
        self
    }

    // XXX: -> &mut Self, or Self
    pub fn opt<T, U>(&mut self, short: T, opt: U) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        let v = self.args.get_or_insert(Vec::new());
        v.push(short.into());
        v.push(opt.into());
        self
    }

    pub fn param<T: Into<Cow<'a, str>>>(&mut self, param: T) -> &mut Self {
        self.args.get_or_insert(Vec::new()).push(param.into());
        self
    }

    /// Set `Cmd.combine_short_flags` to `true`
    pub fn combine_short_flags(&mut self) -> &mut Self {
        self.combine_short_flags = true;
        self
    }

    pub fn not_combine_short_flags(&mut self) -> &mut Self {
        self.combine_short_flags = false;
        self
    }

    pub fn combine_short_flags_ext(&mut self, state: bool) -> &mut Self {
        self.combine_short_flags = state;
        self
    }

    /// Set `Cmd.use_alias` to `true`
    // XXX: Remove, replace?
    pub fn use_alias(&mut self) -> &mut Self {
        self.use_alias = true;
        self
    }

    pub fn not_use_alias(&mut self) -> &mut Self {
        self.use_alias = false;
        self
    }

    pub fn use_alias_ext(&mut self, state: bool) -> &mut Self {
        self.use_alias = state;
        self
    }

    //pub fn arg(&mut self, arg: &'a str) -> &mut Self {
    //self.args.get_or_insert(Vec::new()).push(arg);
    //self
    //}

    // TODO: custom command
    //pub fn custom<S: Into<Cow<'a, str>>>(&self, ) -> &mut Self {
    //}

    //// create `std::process::Command` from `Self` (consuming `Self`)
    //pub fn to_command(&self) -> Command {
    //Command::from(self)
    //}

    // NOTE: can't be consuming `to_vec(self)`, borrowing used in `fmt(&self)`
    /// Transform `Cmd` to `Vec<Cow<'a, str>>`
    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        let mut v: Vec<Cow<'a, str>> = Vec::new();

        if let Some(envs) = &self.envs {
            for (key, value) in envs {
                v.push(Cow::Owned(format!("{}={}", key, value)));
            }
        }

        // XXX: ugly
        if self.use_alias {
            if let Some(alias) = &self.alias {
                v.push(alias.clone());
            } else if let Some(name) = &self.name {
                v.push(name.clone());
            }
        } else if let Some(name) = &self.name {
            v.push(name.clone());
        }

        if let Some(flags_short) = &self.flags_short {
            if self.combine_short_flags {
                v.push(Cow::Owned(format!("-{}", flags_short)));
            } else {
                for c in flags_short.chars() {
                    v.push(Cow::Owned(format!("-{}", c)));
                }
            }
        }

        if let Some(args) = &self.args {
            v.extend(args.to_vec());
        }

        if let Some(cmds) = &self.subcommands {
            v.extend(cmds.to_vec());
        }

        v
    }

    /// Transform `Cmd` into [`std::process::Command`]
    pub fn to_command(self) -> Command {
        let name = self.name.as_ref().unwrap_or(&Cow::Borrowed(""));
        let mut command = Command::new(name.as_ref());

        if let Some(envs) = self.envs {
            command.envs(
                envs.iter()
                    .map(|(key, value)| (key.as_ref(), value.as_ref())),
            );
        }

        if let Some(args) = self.args {
            command.args(args.iter().map(|arg| arg.as_ref()));
        }

        // additional subcommands
        if let Some(cmds) = self.subcommands {
            command.args(cmds.to_vec().iter().map(|arg| arg.as_ref()));
        }

        //if let Some(stdin) = self.stdin {
        //command.stdin(stdin);
        //}

        //if let Some(stdout) = self.stdout {
        //command.stdout(stdout);
        //}

        //if let Some(stderr) = self.stderr {
        //command.stderr(stderr);
        //}

        command
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
    //pub fn into_vec(self) -> Vec<&'a str> {
    //let mut v = Vec::new();

    //v.push(self.name);

    //for cmd in self.cmds.cmds {
    //v.append(&mut cmd.into_vec());
    //}

    //for arg in self.args.args {
    ////v.append(&mut args.into_vec());
    //match arg {
    //Arg::Flag(flag) => {
    //if let Some(short) = flag.get_short() {
    //v.push(short);
    //}
    //}
    //Arg::Opt(opt) => {}
    //Arg::Param(param) => {}
    //_ => {}
    //};
    //}

    //v
    //}
}

// create ready to exec [`std::process::Command`]
// * create [`std::process::Command`]
// * push environment variables
// * push binary arguments
// * push subcommand
impl<'a> From<&TmuxCommand<'a>> for Command {
    fn from(cmd: &TmuxCommand) -> Self {
        // user given command or blank command
        let name = cmd.name.as_ref().unwrap_or(&Cow::Borrowed(EMPTY_CMD));
        let mut command = Command::new(name.as_ref());

        // environment variables
        if let Some(envs) = &cmd.envs {
            command.envs(
                envs.iter()
                    .map(|(key, value)| (key.as_ref(), value.as_ref())),
            );
        }

        // arguments
        if let Some(args) = &cmd.args {
            command.args(args.iter().map(|arg| arg.as_ref()));
        }

        // subcommands
        if let Some(cmds) = &cmd.subcommands {
            command.args(cmds.to_vec().iter().map(|arg| arg.as_ref()));
        }

        command
    }
}

// create ready to exec [`std::process::Command`]
// * create [`std::process::Command`]
// * push environment variables
// * push binary arguments
// * push subcommand
impl<'a> From<TmuxCommand<'a>> for Command {
    fn from(cmd: TmuxCommand) -> Self {
        Command::from(&cmd)
    }
}

/* NOTE: from bin
    /// Returns mutable reference to tmux executable name `Cow<'a, str>`
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

impl<'a> TmuxCommand<'a> {
    pub fn add_command(mut self, command: TmuxCommand<'a>) -> Self {
        self.push_cmd(command);
        self
    }

    // TODO: custom command
    //pub fn custom<S: Into<Cow<'a, str>>>(&self, ) -> &mut Self {
    //}

    //// create `std::process::Command` from `Self` (consuming `Self`)
    //pub fn to_command(&self) -> Command {
    //Command::from(self)
    //}

    /// create [`Tmux`] from [`TmuxCommand`]
    pub fn into_tmux(self) -> Tmux<'a> {
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

//pub trait BuildCommand<'a> {
//fn build(self) -> TmuxCommand<'a>;
//}

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
