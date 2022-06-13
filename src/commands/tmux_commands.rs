use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;
use std::fmt;

//#[derive(Debug, Clone)]
//pub struct TmuxCommands<'a>(pub Vec<TmuxCommand<'a>>);

/// command separator [^f1]
///
/// [^f1] "...Each command is terminated by a newline or a semicolon (;) Commands separated by
/// semicolons together form a ‘command sequence’ - if a command in the sequence encounters an
/// error, no subsequent commands are executed..."
/// [[tmux manual](https://man7.org/linux/man-pages/man1/tmux.1.html#COMMAND_PARSING_AND_EXECUTION)]
pub const TMUX_COMMANDS_SEPARATOR: &str = "\\;";
// TODO: rename TERMINATOR
//const TMUX_COMMANDS_SEPARATOR: &str = "\n";

///

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TmuxCommands<'a> {
    pub cmds: Vec<TmuxCommand<'a>>,
    pub separator: Option<Cow<'a, str>>,
}

impl<'a> fmt::Display for TmuxCommands<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let separator = self
            .separator
            .as_ref()
            .unwrap_or(&Cow::Borrowed(TMUX_COMMANDS_SEPARATOR));
        let output = self.to_vec().join(separator.as_ref());
        write!(f, "{}", output)
    }
}

impl<'a> Default for TmuxCommands<'a> {
    fn default() -> Self {
        Self {
            cmds: Vec::new(),
            separator: Some(Cow::Borrowed(";")),
        }
    }
}

impl<'a> TmuxCommands<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    // XXX: -> Self?
    pub fn push(&mut self, cmd: TmuxCommand<'a>) {
        self.cmds.push(cmd);
    }

    pub fn cmd(mut self, cmd: TmuxCommand<'a>) -> Self {
        self.cmds.push(cmd);
        self
    }

    // XXX: mb use in display trait?
    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        let mut v = Vec::new();

        let len = self.cmds.len();
        for (i, cmd) in self.cmds.iter().enumerate() {
            v.extend(cmd.to_vec());

            if let Some(separator) = &self.separator {
                if i < len - 1 {
                    v.push(separator.clone());
                }
            }
        }

        v
    }

    pub fn output(self) -> Vec<Result<TmuxOutput, Error>> {
        let mut v = Vec::new();

        for cmd in self.cmds {
            v.push(cmd.output())
        }

        v
    }

    // NOTE: from bin
    // XXX: error out
    //pub fn output(&self) -> Result<TmuxOutput, Error> {

    //let mut command = Command::new(&self.tmux.bin.as_ref());

    //for tmux_command in &self.cmds.0 {
    //if let Some(cmd) = &tmux_command.cmd {
    //command.arg(cmd.as_ref());
    //}

    //if let Some(args) = &tmux_command.args {
    //for arg in args {
    //command.arg(arg.as_ref());
    //}
    //}

    //command.arg(";");
    //}

    // NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
    //command.stdin(Stdio::inherit());

    //let output = command.output()?;

    //Ok(TmuxOutput(output))
    //}
}
