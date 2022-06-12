use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;
use std::marker::PhantomData;

/// List the syntax of all commands supported by tmux
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux list-commands [-F format] [command]
/// (alias: lscm)
/// ```
///
/// tmux ^2.3:
/// ```text
/// tmux list-commands [-F format]
/// (alias: lscm)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-commands
/// (alias: lscm)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListCommands<'a> {
    /// `[-F format]`
    #[cfg(feature = "tmux_2_3")]
    pub format: Option<Cow<'a, str>>,

    /// `[command]`
    #[cfg(feature = "tmux_3_2")]
    pub command: Option<Cow<'a, str>>,

    _phantom_data: PhantomData<&'a ()>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListCommands2<T, U> {
    pub format: Option<T>,
    pub command: Option<U>,
}

impl<T: Default + AsRef<str>, U: Default + AsRef<str>> ListCommands2<T, U> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn format(&mut self, format: T) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn command(&mut self, command: U) -> &mut Self {
        self.command = Some(command);
        self
    }
}

#[test]
fn test_listcommands2() {
    let mut s = ListCommands2::new();
    s.format(String::from("format"));
    s.command("command");

    //let mut s = ListCommands2::new();
    //s.format(String::from("format"))
    //s.command("command");
    dbg!(s);
}

impl<'a> ListCommands<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_2_3")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[command]`
    #[cfg(feature = "tmux_3_2")]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(LIST_COMMANDS);

        // `[-F format]`
        #[cfg(feature = "tmux_2_3")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[command]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        cmd
    }
}

impl<'a> From<ListCommands<'a>> for TmuxCommand<'a> {
    fn from(item: ListCommands<'a>) -> Self {
        item.build()
    }
}

//impl<'a> From<&'a mut ListCommands<'a>> for TmuxCommand<'a> {
//fn from(item: &'a mut ListCommands<'a>) -> Self {
//item.build()
//}
//}
