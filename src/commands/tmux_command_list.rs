use crate::TmuxCommand;
use std::borrow::Cow;
use std::fmt;
use std::process::Command;

//#[derive(Debug, Clone)]
//pub struct TmuxCommands<'a>(pub Vec<TmuxCommand<'a>>);

/// command separator [^f1]
///
/// [^f1] "...Each command is terminated by a newline or a semicolon (;) Commands separated by
/// semicolons together form a ‘command sequence’ - if a command in the sequence encounters an
/// error, no subsequent commands are executed..."
/// [[tmux manual](https://man7.org/linux/man-pages/man1/tmux.1.html#COMMAND_PARSING_AND_EXECUTION)]
pub const TMUX_COMMANDS_SEPARATOR: &str = ";";
pub const TMUX_COMMAND_SEPARATOR: &str = " ";
// TODO: rename TERMINATOR
//const TMUX_COMMANDS_SEPARATOR: &str = "\n";

///

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TmuxCommandList<'a> {
    pub commands: Vec<TmuxCommand<'a>>,

    // XXX: Cow<'a, str> or &'a str?
    pub separator: Option<Cow<'a, str>>,
}

impl<'a> fmt::Display for TmuxCommandList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let separator = self.separator.as_ref().unwrap_or(&Cow::Borrowed(" "));
        //let output = self.to_vec().join(separator.as_ref());
        let output = self.to_vec().join(TMUX_COMMAND_SEPARATOR);
        write!(f, "{}", output)
    }
}

// None = "", Some = ";", Some = "\n"
impl<'a> Default for TmuxCommandList<'a> {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            separator: Some(TMUX_COMMANDS_SEPARATOR.into()),
        }
    }
}

impl<'a> TmuxCommandList<'a> {
    // XXX: optimize
    pub fn push_cmds(&mut self, cmds: TmuxCommandList<'a>) {
        for cmd in cmds.commands {
            self.push(cmd);
        }
    }

    pub fn add_command(mut self, cmd: TmuxCommand<'a>) -> Self {
        self.push(cmd);
        self
    }
}

impl<'a> TmuxCommandList<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    // XXX: -> Self?
    pub fn push<T: Into<TmuxCommand<'a>>>(&mut self, command: T) {
        self.commands.push(command.into());
    }

    // XXX: same fn push?
    pub fn cmd<T: Into<TmuxCommand<'a>>>(mut self, command: T) -> Self {
        self.commands.push(command.into());
        self
    }

    // XXX: mb use in display trait?
    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        let mut v = Vec::new();

        let len = self.commands.len();
        for (i, command) in self.commands.iter().enumerate() {
            v.extend(command.to_vec());

            if let Some(separator) = &self.separator {
                if i < len - 1 {
                    v.push(separator.clone());
                    //v.push(Cow::Borrowed(separator));
                }
            }
        }

        v
    }

    pub fn to_command_vec(self) -> Vec<Command> {
        let mut v = Vec::new();
        for cmd in self.commands {
            v.push(cmd.to_command());
        }
        v
    }

    pub fn separator<S: Into<Cow<'a, str>>>(&mut self, separator: S) -> &mut Self {
        self.separator = Some(separator.into());
        self
    }

    pub fn get_separator(&self) -> Option<&Cow<'a, str>> {
        self.separator.as_ref()
    }

    pub fn into_cmds(self) -> Vec<TmuxCommand<'a>> {
        self.commands
    }
}
