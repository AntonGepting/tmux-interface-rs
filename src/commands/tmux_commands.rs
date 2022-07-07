use crate::TmuxCommand;
use cmd_builder::CmdList;
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

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TmuxCommands<'a>(pub CmdList<'a>);

impl<'a> fmt::Display for TmuxCommands<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> TmuxCommands<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    // XXX: -> Self?
    pub fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T) {
        self.0.push(cmd.into().0);
    }

    pub fn cmd(mut self, cmd: TmuxCommand<'a>) -> Self {
        self.0.push(cmd.0);
        self
    }

    // XXX: mb use in display trait?
    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        self.0.to_vec()
    }

    // XXX: ugly
    //pub fn output(self) -> Vec<Result<TmuxOutput, Error>> {
    //let mut v = Vec::new();
    //for output in self.0.output() {
    //let output = match output {
    //Ok(o) => Ok(TmuxOutput(o)),
    //Err(e) => Err(Error::from(e)),
    //};
    //v.push(output);
    //}
    //v
    //}

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
