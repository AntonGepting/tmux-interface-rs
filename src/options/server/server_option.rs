use crate::options::*;
use crate::{Error, SetOption, ShowOptions, Tmux, TmuxCommand, TmuxCommands, TmuxOutput};
use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

// get
// * get single one
// * get all for object
//
// set
// * set single one
//      * set value
//      * toggle (on/off {default}/off) if no value specified

// FIXME: proper Error in return

// total num: 16
/// `backspace
#[cfg(feature = "tmux_3_1")]
pub const BACKSPACE: &str = "backspace";
/// `buffer-limit`
#[cfg(feature = "tmux_1_5")]
pub const BUFFER_LIMIT: &str = "buffer-limit";
/// `command-alias`
#[cfg(feature = "tmux_2_4")]
pub const COMMAND_ALIAS: &str = "command-alias";
/// `default-terminal`
#[cfg(feature = "tmux_2_1")]
pub const DEFAULT_TERMINAL: &str = "default-terminal";
/// `copy-command`
#[cfg(feature = "tmux_3_2")]
pub const COPY_COMMAND: &str = "copy-command";
/// `escape-time`
#[cfg(feature = "tmux_1_2")]
pub const ESCAPE_TIME: &str = "escape-time";
/// `editor`
#[cfg(feature = "tmux_3_2")]
pub const EDITOR: &str = "editor";
/// `exit-empty`
#[cfg(feature = "tmux_2_7")]
pub const EXIT_EMPTY: &str = "exit-empty";
/// `exit-unattached`
#[cfg(feature = "tmux_1_4")]
pub const EXIT_UNATTACHED: &str = "exit-unattached";
/// `extended-keys`
#[cfg(feature = "tmux_3_2")]
pub const EXTENDED_KEYS: &str = "extended-keys";
/// `focus-events`
#[cfg(feature = "tmux_1_9")]
pub const FOCUS_EVENTS: &str = "focus-events";
/// `history-file`
#[cfg(feature = "tmux_2_1")]
pub const HISTORY_FILE: &str = "history-file";
/// `message-limit`
#[cfg(feature = "tmux_2_0")]
pub const MESSAGE_LIMIT: &str = "message-limit";
/// `prompt-history-limit`
#[cfg(feature = "tmux_3_3")]
pub const PROMPT_HISTORY_LIMIT: &str = "prompt-history-limit";
/// `set-clipboard`
#[cfg(feature = "tmux_1_5")]
pub const SET_CLIPBOARD: &str = "set-clipboard";
/// `terminal-features`
#[cfg(feature = "tmux_3_2")]
pub const TERMINAL_FEATURES: &str = "terminal-features";
/// `terminal-overrides`
#[cfg(feature = "tmux_2_0")]
pub const TERMINAL_OVERRIDES: &str = "terminal-overrides";
/// `user-keys`
#[cfg(feature = "tmux_3_0")]
pub const USER_KEYS: &str = "user-keys";
/// `quiet`
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
pub const QUIET: &str = "quiet";
/// `detach-on-destroy [on | off]
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
pub const DETACH_ON_DESTROY: &str = "detach-on-destroy";

const USER_OPTION_MARKER: &str = "@";
const SEPARATOR: &str = " ";

pub enum ServerOptionName {
    // backspace key
    #[cfg(feature = "tmux_3_1")]
    Backspace,
    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    BufferLimit,
    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    CommandAlias,
    // default-terminal terminal
    #[cfg(feature = "tmux_2_1")]
    DefaultTerminal,
    // copy-command shell-command
    #[cfg(feature = "tmux_3_3")]
    CopyCommand,
    // escape-time time
    #[cfg(feature = "tmux_1_2")]
    EscapeTime,
    // editor shell-command
    #[cfg(feature = "tmux_3_2")]
    Editor,
    // exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    ExitEmpty,
    // exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    ExitUnattached,
    // extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    ExtendedKeys,
    // focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    FocusEvents,
    // history-file path
    #[cfg(feature = "tmux_2_1")]
    HistoryFile,
    // message-limit number
    #[cfg(feature = "tmux_2_0")]
    MessageLimit,
    // prompt-history-limit number
    #[cfg(feature = "tmux_3_3")]
    PromptHistoryLimit,
    // set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    SetClipboard,
    // terminal-features[] string
    #[cfg(feature = "tmux_3_2")]
    TerminalFeatures,
    // terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    TerminalOverrides,
    // user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    UserKeys,
    // quiet [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    Quiet,
    // detach-on-destroy [on | off]
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    DetachOnDestroy,
    // user option
    UserOption(String),
}

fn array_name(name: &str, index: usize) -> String {
    format!("{}[{}]", name, index)
}

//impl<'a> From<ServerOptionName> for Cow<'a, str> {
//fn from(name: ServerOptionName) -> Self {
//Cow::from(&name)
//}
//}

//impl<'a> From<&ServerOptionName> for Cow<'a, str> {
//fn from(name: &ServerOptionName) -> Self {
//match name {
//#[cfg(feature = "tmux_3_1")]
//ServerOptionName::Backspace => BACKSPACE.into(),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::BufferLimit => BUFFER_LIMIT.into(),
//#[cfg(feature = "tmux_2_4")]
////ServerOptionName::CommandAlias(i) => array_name(COMMAND_ALIAS, *i).into(),
//ServerOptionName::CommandAlias => COMMAND_ALIAS.into(),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::CopyCommand => COPY_COMMAND.into(),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::DefaultTerminal => DEFAULT_TERMINAL.into(),
//#[cfg(feature = "tmux_1_2")]
//ServerOptionName::EscapeTime => ESCAPE_TIME.into(),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::Editor => EDITOR.into(),
//#[cfg(feature = "tmux_2_7")]
//ServerOptionName::ExitEmpty => EXIT_EMPTY.into(),
//#[cfg(feature = "tmux_1_4")]
//ServerOptionName::ExitUnattached => EXIT_UNATTACHED.into(),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::ExtendedKeys => EXTENDED_KEYS.into(),
//#[cfg(feature = "tmux_1_9")]
//ServerOptionName::FocusEvents => FOCUS_EVENTS.into(),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::HistoryFile => HISTORY_FILE.into(),
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::MessageLimit => MESSAGE_LIMIT.into(),
//#[cfg(feature = "tmux_3_3")]
//ServerOptionName::PromptHistoryLimit => PROMPT_HISTORY_LIMIT.into(),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::SetClipboard => SET_CLIPBOARD.into(),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::TerminalFeatures(i) => array_name(TERMINAL_FEATURES, *i).into(),
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::TerminalOverrides(i) => array_name(TERMINAL_OVERRIDES, *i).into(),
//#[cfg(feature = "tmux_3_0")]
//ServerOptionName::UserKeys(i) => array_name(USER_KEYS, *i).into(),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
//ServerOptionName::Quiet => QUIET.into(),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
//ServerOptionName::DetachOnDestroy => DETACH_ON_DESTROY.into(),
//ServerOptionName::UserOption(user_option_name) => {
//format!("{}{}", USER_OPTION_MARKER, user_option_name).into()
//}
//}
//}
//}

//impl fmt::Display for ServerOptionName {
//fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
//let s: Cow<'a, str> = self.into();
//write!(f, "{}", s)
//}
//}

pub enum TmuxServerOption {
    A,
}

pub enum TmuxOptionName {
    TmuxServerOption(TmuxServerOption),
    //Session(),
    //Window(),
    //Pane(),
}

//#[derive(Default)]
pub struct TmuxOption<T> {
    pub name: TmuxOptionName,
    pub index: Option<usize>,
    pub global: bool,
    pub value: Option<T>,
}

impl<T: fmt::Display> TmuxOption<T> {}

//#[derive(Default)]
pub struct TmuxServerOption2<T>(pub TmuxOption<T>);

//pub struct OptionsController<'a> {
//pub setter: &'a dyn Fn(&str) -> String,
//pub getter: &'a dyn Fn(&str) -> String,
//}

//impl<'a> OptionsController<'a> {
//pub fn new() -> Self {
//Self {
//setter: &Self::default_getter,
//getter: &Self::default_getter,
//}
//}

//fn default_getter(name: &str) -> String {
//Tmux::new()
//.command(SetOption::new().server().option(name))
//.output()
//.unwrap()
//.to_string()
//}

// get from output containing name value pair (request: `backspace`, response: `backspace C-?`)
//pub fn get_name_value(tmux: Option<Tmux>, name: ServerOptionName) -> Result<ServerOption, Error> {
//let show_options = ShowOptions::new().server().option(name);
//let tmux = match tmux {
//Some(tmux) => tmux.command(show_options),
//None => Tmux::new().command(show_options),
//};
//let output = tmux.output()?.to_string();
////parse(output, name)
//}

//// get from output containing only value (request: `backspace`, response: `C-?`)
//pub fn get_value(tmux: Option<Tmux>, name: ServerOptionName) -> Result<ServerOption, Error> {
//unimplemented!()
////let show_options = ShowOptions::new().server().value().option(name);
////let tmux = match tmux {
////Some(tmux) => tmux.command(show_options),
////None => Tmux::new().command(show_options),
////};
//// FIXME
////tmux.output()?.to_string().parse()
//}

//pub fn setter(&mut self, setter: &'a dyn Fn(&str) -> String) -> &mut Self {
//self.setter = setter;
//self
//}

//pub fn getter(&mut self, getter: &'a dyn Fn(&str) -> String) -> &mut Self {
//self.getter = getter;
//self
//}
//}

//impl<'a> ServerOptionController<'a> {
//pub fn new() -> Self {}
//}

//impl<T: fmt::Display> TmuxServerOption2<T> {
//pub fn buffer_limit() {}

//pub fn get(&self, name: ServerOptionName) -> Result<ServerOption, Error> {
//Tmux::new()
//.command(ShowOptions::new().server().option(name))
//.output()?
//.to_string()
//.parse()
//}

//pub fn set(&self) -> Result<(), Error> {
////Tmux::new()
////.command(SetOption::new().server().option(name).value())
////.output()?;
//Ok(())
//}
//}

pub struct GetServerOption<'a> {
    pub getter: &'a dyn Fn(Cow<'a, str>) -> TmuxCommand<'a>,
}

impl<'a> Default for GetServerOption<'a> {
    fn default() -> Self {
        GetServerOption {
            getter: &get_option_default,
        }
    }
}

fn get_option_default<'a>(name: Cow<'a, str>) -> TmuxCommand<'a> {
    ShowOptions::new().server().value().option(name).build()
}

fn get_global_option_default<'a>(name: Cow<'a, str>) -> TmuxCommand<'a> {
    ShowOptions::new()
        .server()
        .global()
        .option(name)
        .value()
        .build()
}

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
impl<'a> GetServerOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn global() -> Self {
        Self {
            getter: &get_global_option_default,
        }
    }

    pub fn get<T: Into<Cow<'a, str>>>(&self, name: T) -> TmuxCommand<'a> {
        (self.getter)(name.into())
    }

    //pub fn gets<'a>(names: ServerOptionB) -> TmuxCommands<'a> {
    //let mut cmds = TmuxCommands::new();
    //for name in names.0 {
    //cmds.push(Self::get(name));
    //}
    //cmds
    //}

    /// ### Manual
    ///
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace(&self) -> TmuxCommand<'a> {
        self.get(BACKSPACE)
    }

    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit(&self) -> TmuxCommand<'a> {
        self.get(BUFFER_LIMIT)
    }

    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias(&self) -> TmuxCommand<'a> {
        self.get(COMMAND_ALIAS)
    }

    // default-terminal terminal
    #[cfg(feature = "tmux_3_2")]
    pub fn copy_command(&self) -> TmuxCommand<'a> {
        self.get(COPY_COMMAND)
    }

    // copy-command shell-command
    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal(&self) -> TmuxCommand<'a> {
        self.get(DEFAULT_TERMINAL)
    }

    // escape-time time
    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time(&self) -> TmuxCommand<'a> {
        self.get(ESCAPE_TIME)
    }

    // editor shell-command
    #[cfg(feature = "tmux_3_2")]
    pub fn editor(&self) -> TmuxCommand<'a> {
        self.get(EDITOR)
    }

    // exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty(&self) -> TmuxCommand<'a> {
        self.get(EXIT_EMPTY)
    }

    // exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached(&self) -> TmuxCommand<'a> {
        self.get(EXIT_UNATTACHED)
    }

    // extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys(&self) -> TmuxCommand<'a> {
        self.get(EXTENDED_KEYS)
    }

    // focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events(&self) -> TmuxCommand<'a> {
        self.get(FOCUS_EVENTS)
    }

    // history-file path
    #[cfg(feature = "tmux_2_1")]
    pub fn history_file(&self) -> TmuxCommand<'a> {
        self.get(HISTORY_FILE)
    }

    // message-limit number
    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit(&self) -> TmuxCommand<'a> {
        self.get(MESSAGE_LIMIT)
    }

    // prompt-history-limit number
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_history_limit(&self) -> TmuxCommand<'a> {
        self.get(PROMPT_HISTORY_LIMIT)
    }

    //set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard(&self) -> TmuxCommand<'a> {
        self.get(SET_CLIPBOARD)
    }

    // terminal-features[] string
    #[cfg(feature = "tmux_3_2")]
    pub fn terminal_features(&self) -> TmuxCommand<'a> {
        self.get(TERMINAL_FEATURES)
    }

    // terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides(&self) -> TmuxCommand<'a> {
        self.get(TERMINAL_OVERRIDES)
    }

    // user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys(&self) -> TmuxCommand<'a> {
        self.get(USER_KEYS)
    }

    // quiet [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn quiet(&self) -> TmuxCommand<'a> {
        self.get(QUIET)
    }

    // detach-on-destroy [on | off]
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn detach_on_destroy(&self) -> TmuxCommand<'a> {
        self.get(DETACH_ON_DESTROY)
    }

    // user option
    pub fn user_option<S: fmt::Display>(&self, name: S) -> TmuxCommand<'a> {
        self.get(format!("{}{}", USER_OPTION_MARKER, name))
    }
}

pub struct SetServerOption<'a> {
    pub setter: &'a dyn Fn(Cow<'a, str>, Option<Cow<'a, str>>) -> TmuxCommand<'a>,
    pub unsetter: &'a dyn Fn(Cow<'a, str>) -> TmuxCommand<'a>,
}

impl<'a> Default for SetServerOption<'a> {
    fn default() -> Self {
        Self {
            setter: &set_option_default,
            unsetter: &unset_option_default,
        }
    }
}

fn set_option_default<'a>(name: Cow<'a, str>, value: Option<Cow<'a, str>>) -> TmuxCommand<'a> {
    let cmd = match value {
        Some(data) => SetOption::new().server().option(name).value(data),
        None => SetOption::new().server().option(name),
    };
    cmd.build()
}

fn unset_option_default<'a>(name: Cow<'a, str>) -> TmuxCommand<'a> {
    SetOption::new().server().option(name).unset().build()
}

impl<'a> SetServerOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    //pub fn global() -> Self {
    //Self(())
    //}

    pub fn set<T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        &self,
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        match value {
            Some(data) => self.set_ext(name, Some(data)),
            None => self.unset_ext(name),
        }
    }

    // toggle if value = None
    pub fn unset_ext<T: Into<Cow<'a, str>>>(&self, name: T) -> TmuxCommand<'a> {
        (self.unsetter)(name.into())
    }

    // toggle if value = None
    pub fn set_ext<T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        &self,
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        (self.setter)(name.into(), value.map(|s| s.into()))
    }

    pub fn set_array<S: fmt::Display>(
        &self,
        name: S,
        value: Option<Vec<String>>,
    ) -> TmuxCommands<'a> {
        let mut cmds = TmuxCommands::new();
        if let Some(data) = value {
            for (i, item) in data.iter().enumerate() {
                cmds.push(self.set(format!("{}[{}]", name, i), Some(item.to_owned())));
            }
        }
        cmds
    }

    // backspace key
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace<S: Into<Cow<'a, str>>>(&self, backspace: S) -> TmuxCommand<'a> {
        self.set(BACKSPACE, backspace)
    }

    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit(&self, buffer_limit: Option<usize>) -> TmuxCommand<'a> {
        self.set(BUFFER_LIMIT, buffer_limit.map(|s| s.to_string()))
    }

    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias(&self, command_alias: Option<Vec<String>>) -> TmuxCommands<'a> {
        self.set_array(COMMAND_ALIAS, command_alias)
    }

    // default-terminal terminal
    #[cfg(feature = "tmux_3_2")]
    pub fn copy_command<S: Into<Cow<'a, str>>>(&self, copy_command: Option<S>) -> TmuxCommand<'a> {
        self.set(COPY_COMMAND, copy_command)
    }

    // copy-command shell-command
    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal<S: Into<Cow<'a, str>>>(
        &self,
        default_terminal: Option<S>,
    ) -> TmuxCommand<'a> {
        self.set(DEFAULT_TERMINAL, default_terminal)
    }

    // escape-time time
    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time(&self, escape_time: Option<usize>) -> TmuxCommand<'a> {
        self.set(ESCAPE_TIME, escape_time.map(|s| s.to_string()))
    }

    // editor shell-command
    #[cfg(feature = "tmux_3_2")]
    pub fn editor<S: Into<Cow<'a, str>>>(&self, editor: Option<S>) -> TmuxCommand<'a> {
        self.set(EDITOR, editor)
    }

    // exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty(&self, exit_empty: Option<Switch>) -> TmuxCommand<'a> {
        self.set_ext(EXIT_EMPTY, exit_empty.map(|s| s.to_string()))
    }

    // exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached(&self, exit_unattached: Option<Switch>) -> TmuxCommand<'a> {
        self.set_ext(EXIT_UNATTACHED, exit_unattached.map(|s| s.to_string()))
    }

    // extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys(&self, extended_keys: Option<Switch>) -> TmuxCommand<'a> {
        self.set_ext(EXTENDED_KEYS, extended_keys.map(|s| s.to_string()))
    }

    // focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events(&self, focus_events: Option<Switch>) -> TmuxCommand<'a> {
        self.set_ext(FOCUS_EVENTS, focus_events.map(|s| s.to_string()))
    }

    // history-file path
    #[cfg(feature = "tmux_2_1")]
    pub fn history_file<S: Into<Cow<'a, str>>>(&self, history_file: Option<S>) -> TmuxCommand<'a> {
        self.set(HISTORY_FILE, history_file)
    }

    // message-limit number
    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit(&self, message_limit: Option<usize>) -> TmuxCommand<'a> {
        self.set(MESSAGE_LIMIT, message_limit.map(|s| s.to_string()))
    }

    // prompt-history-limit number
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_history_limit(&self, prompt_history_limit: Option<usize>) -> TmuxCommand<'a> {
        self.set(PROMPT_HISTORY_LIMIT, prompt_history_limit.to_string())
    }

    // set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard(&self, set_clipboard: Option<SetClipboard>) -> TmuxCommand<'a> {
        self.set_ext(SET_CLIPBOARD, set_clipboard.map(|s| s.to_string()))
    }

    // terminal-features[] string
    #[cfg(feature = "tmux_3_2")]
    pub fn terminal_features(&self, terminal_features: Option<Vec<String>>) -> TmuxCommands<'a> {
        self.set_array(TERMINAL_FEATURES, terminal_features)
    }

    // terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides(&self, terminal_overrides: Option<Vec<String>>) -> TmuxCommands<'a> {
        self.set_array(TERMINAL_OVERRIDES, terminal_overrides)
    }

    // user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys(&self, user_keys: Option<Vec<String>>) -> TmuxCommands<'a> {
        self.set_array(USER_KEYS, user_keys)
    }

    // quiet [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn quiet(&self, quiet: Option<Switch>) -> TmuxCommand<'a> {
        self.set_ext(QUIET, quiet.map(|s| s.to_string()))
    }

    // detach-on-destroy [on | off]
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn detach_on_destroy(&self, detach_on_destroy: Option<Switch>) -> TmuxCommand<'a> {
        self.set_ext(DETACH_ON_DESTROY, detach_on_destroy.map(|s| s.to_string()))
    }

    // user option
    pub fn user_option<S: fmt::Display>(&self, name: S, value: Option<String>) -> TmuxCommand<'a> {
        self.set(format!("{}{}", USER_OPTION_MARKER, name), value)
    }
}

//pub struct SetOptions {
//}

//impl<'a> SetOptions<'a> {
////pub fn new() -> TmuxCommands<'a>

////pub fn history_file(mut self) -> TmuxCommands<'a> {
////history_file(, )
////}
//}

#[test]
fn get_server_option_c() {
    let cmd = Tmux::new()
        .command(GetServerOption::new().get(BUFFER_LIMIT))
        .output()
        .unwrap();
    let cmd = Tmux::new()
        .command(GetServerOption::new().buffer_limit())
        .command(GetServerOption::new().set_clipboard())
        .output()
        .unwrap();
    dbg!(&cmd);
    let cmd = TmuxServerOptionOutput::from(cmd).buffer_limit();
    dbg!(&cmd);

    let cmd = Tmux::new()
        .command(GetServerOption::new().command_alias())
        .output()
        .unwrap();
    let cmd = TmuxServerOptionOutput::from(cmd).command_alias();
    dbg!(&cmd);

    let cmds = SetServerOption::new().command_alias(vec!["asdf".to_string(), "a".to_string()]);
    dbg!(&cmds);
}

impl<'a> From<TmuxOutput> for TmuxServerOptionOutput {
    fn from(tmux_output: TmuxOutput) -> Self {
        Self(tmux_output.to_string())
    }
}

//#[derive(Debug)]
//pub struct TmuxServerOptionOutputFull;

// variants possible:
// * option_name value
// * option_name
// * option_name[i] value
//
pub enum TmuxServerOptionOutputFull {
    // backspace key
    #[cfg(feature = "tmux_3_1")]
    Backspace(Option<String>),
    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    BufferLimit(Option<usize>),
    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    //CommandAlias(Option<Vec<String>>),
    //CommandAlias(Option<ArrayItem<String>>),
    CommandAlias(Option<(usize, String)>),
    // default-terminal terminal
    #[cfg(feature = "tmux_2_1")]
    DefaultTerminal(Option<String>),
    //copy_command
    //escape-time time
    #[cfg(feature = "tmux_1_2")]
    EscapeTime(Option<usize>),
    //editor
    //exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    ExitEmpty(Option<Switch>),
    //exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    ExitUnattached(Option<Switch>),
    //extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    ExtendedKeys(Option<Switch>),
    //focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    FocusEvents(Option<Switch>),
    //history-file path
    #[cfg(feature = "tmux_2_1")]
    HistoryFile(Option<String>),
    //message-limit number
    #[cfg(feature = "tmux_2_0")]
    MessageLimit(Option<usize>),
    //set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    SetClipboard(Option<SetClipboard>),
    // terminal-features[]
    //terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    //TerminalOverrides(Option<ArrayItem<String>>),
    TerminalOverrides(Option<(usize, String)>),
    //user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    UserKeys(Option<(usize, String)>),
    // quiet ?
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    Quiet(Option<Switch>),
    // detach-on-destroy ?
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    DetachOnDestroy(Option<Switch>),
    // user option
    UserOption(Option<String>),
}

// mb common
fn parse_value<T: FromStr>(value: Option<&str>) -> Option<T> {
    value.and_then(|data| data.parse().ok())
}

impl TmuxServerOptionOutputFull {
    // split string in 3 parts, name, index (if option is an array) and value
    // TODO: rename
    pub fn get_option(s: &str) -> (Option<&str>, Option<usize>, Option<&str>) {
        let v: Vec<&str> = s.trim().splitn(2, SEPARATOR).collect();
        let value = v.get(1).copied();
        let mut index: Option<usize> = None;
        let mut name = v.get(0).copied();

        if let Some(name_array) = name {
            let v: Vec<&str> = name_array.split(|c| c == '[' || c == ']').collect();
            name = v.get(0).copied();
            index = v.get(1).and_then(|i| i.parse().ok());
        }

        (name, index, value)
    }

    // parse single option with given name value and index
    pub fn parse_option(
        name: &str,
        index: Option<usize>,
        value: Option<&str>,
    ) -> Result<Self, Error> {
        match name {
            #[cfg(feature = "tmux_3_1")]
            BACKSPACE => Ok(Self::Backspace(parse_value(value))),
            #[cfg(feature = "tmux_1_5")]
            BUFFER_LIMIT => Ok(Self::BufferLimit(parse_value(value))),
            #[cfg(feature = "tmux_2_4")]
            COMMAND_ALIAS => Ok(Self::CommandAlias(parse_array_value(index, value))),
            #[cfg(feature = "tmux_2_1")]
            DEFAULT_TERMINAL => Ok(Self::DefaultTerminal(parse_value(value))),
            #[cfg(feature = "tmux_1_2")]
            ESCAPE_TIME => Ok(Self::EscapeTime(parse_value(value))),
            #[cfg(feature = "tmux_2_7")]
            EXIT_EMPTY => Ok(Self::ExitEmpty(parse_value(value))),
            #[cfg(feature = "tmux_1_4")]
            EXIT_UNATTACHED => Ok(Self::ExitUnattached(parse_value(value))),
            #[cfg(feature = "tmux_3_2")]
            EXTENDED_KEYS => Ok(Self::ExtendedKeys(parse_value(value))),
            #[cfg(feature = "tmux_1_9")]
            FOCUS_EVENTS => Ok(Self::FocusEvents(parse_value(value))),
            #[cfg(feature = "tmux_2_1")]
            HISTORY_FILE => Ok(Self::HistoryFile(parse_value(value))),
            #[cfg(feature = "tmux_2_0")]
            MESSAGE_LIMIT => Ok(Self::MessageLimit(parse_value(value))),
            #[cfg(feature = "tmux_1_5")]
            SET_CLIPBOARD => Ok(Self::SetClipboard(parse_value(value))),
            #[cfg(feature = "tmux_2_0")]
            TERMINAL_OVERRIDES => Ok(Self::TerminalOverrides(parse_array_value(index, value))),
            #[cfg(feature = "tmux_3_0")]
            USER_KEYS => Ok(Self::UserKeys(parse_array_value(index, value))),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            QUIET => Ok(Self::Quiet(parse_value(value))),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            DETACH_ON_DESTROY => Ok(Self::DetachOnDestroy(parse_value(value))),
            _ => Err(Error::ParseStatusKeys),
        }
    }
}

// single line
// multiple lines
impl FromStr for TmuxServerOptionOutputFull {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let (name, i, value) = Self::get_option(s);

        //match name {
        //Some(name) => Self::parse_option(name, i, value),
        //None => Err(Error::ParseStatusKeys),
        //}
        Err(Error::ParseStatusKeys)
    }
}

#[derive(Debug)]
pub struct TmuxServerOptionOutput(String);

impl TmuxServerOptionOutput {
    // backspace key
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace(&self) -> Result<String, ()> {
        Ok(self.0.trim_end())
    }

    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit<'a>(&self) -> Result<usize, Error> {
        Ok(self.0.trim_end().parse()?)
    }

    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias(&self) -> Vec<String> {
        let mut v = Vec::new();
        for item in self.0.lines() {
            v.push(item.to_string())
        }
        v
    }

    // default-terminal terminal
    #[cfg(feature = "tmux_3_2")]
    pub fn copy_command(&self) -> String {
        self.0.trim_end().to_string()
    }

    // copy-command shell-command
    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal(&self) -> String {
        self.0.trim_end().to_string()
    }

    // escape-time time
    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time(&self) -> Result<usize, Error> {
        Ok(self.0.trim_end().parse()?)
    }

    // editor shell-command
    #[cfg(feature = "tmux_3_2")]
    pub fn editor(&self) -> String {
        self.0.trim_end()
    }

    // exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // history-file path
    #[cfg(feature = "tmux_2_1")]
    pub fn history_file(&self) -> String {
        self.0.trim_end().to_string()
    }

    // message-limit number
    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit(&self) -> Result<usize, Error> {
        Ok(self.0.trim_end().parse()?)
    }

    // prompt-history-limit number
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_history_limit(&self) -> Result<usize, Error> {
        self.0.trim_end().parse()
    }

    //set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard(&self) -> Result<SetClipboard, Error> {
        self.0.trim_end().parse()
    }

    // terminal-features[] string
    #[cfg(feature = "tmux_3_2")]
    pub fn terminal_features(&self) -> Vec<String> {
        let mut v = Vec::new();
        for item in self.0.lines() {
            v.push(item.to_string())
        }
        v
    }

    // terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides(&self) -> Vec<String> {
        let mut v = Vec::new();
        for item in self.0.lines() {
            v.push(item.to_string())
        }
        v
    }

    // user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys(&self) -> TmuxCommand<'a> {}

    // quiet [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn quiet(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // detach-on-destroy [on | off]
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn detach_on_destroy(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // user option
    //pub fn user_option<S: fmt::Display>(&self, name: S) -> TmuxCommand<'a> {
    //}
}

// TODO: multiline
fn default_show_server_option<'a, S: Into<Cow<'a, str>>>(name: S) -> Result<String, Error> {
    let output = Tmux::new()
        .command(ShowOptions::new().server().option(name).value())
        .output()?;
    Ok(output.to_string())
}

fn default_set_option<'a, S: Into<Cow<'a, str>>>(name: S, value: S) -> Result<TmuxOutput, Error> {
    Tmux::new()
        .command(SetOption::new().server().option(name).value(value))
        .output()
}

//pub struct ArrayItem<T> {
//pub index: usize,
//pub data: T,
//}

//impl<T: fmt::Display> fmt::Display for ArrayItem<T> {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
////let mut s = String::new();

////if let Some(i) = self.index {
//let mut s = format!("[{}]", self.index);
////}

////if let Some(data) = self.data {
//s = format!("{} {}", s, self.data);
////}

//write!(f, "{}", s)
//}
//}

//impl<T: fmt::Display> ArrayItem<T> {
////
//pub fn create_name_with_index<'a>(&self, name: &'a str) -> Cow<'a, str> {
////match self.index {
////Some(i) => format!("{}[{}]", name, i).into(),
////None => name.into(),
////}
//format!("{}[{}]", name, self.index).into()
//}

//pub fn to_string_with_name(&self, name: &str) -> String {
//let mut s = name.to_string();
////if let Some(i) = self.index {
//s = format!("{}[{}]", s, self.index);
////}
////if let Some(data) = self.data {
//s = format!("{} {}", s, self.data);
////}
//s
//}
//}

// mb common
// if option value is not set, print only name
//fn print_option<S: ToString>(name: &str, value: &Option<S>) -> String {
//match value {
//Some(value) => format!("{} {}", name, value.to_string()),
//None => format!("{}", name),
//}
//}

// if option value is not set, print only name
//fn print_array_option<T: fmt::Display>(name: &str, value: &Option<(usize, T)>) -> String {
//match value {
//Some((i, data)) => format!("{}[{}] {}", name, i, data),
//None => format!("{}", name),
//}
//}

//fn print_option_array_item(name: &str, array: Vec<String>, index: usize) -> String {
//let item = array.get(index);
//match item {
//Some(data) => format!("{}[{}] {}", name, index, data),
//None => format!("{}[{}]", name, index),
//}
//}
//fn get_name_value<S: ToString>(name: &str, value: Option<S>) -> {

//}

fn parse_array_value<T: FromStr>(index: Option<usize>, value: Option<&str>) -> Option<(usize, T)> {
    match index {
        Some(i) => value
            .and_then(|data| data.parse().ok())
            .map(|data| (i, data)),
        None => None,
    }
}

//impl fmt::Display for ServerOption {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//let s = match self {
//#[cfg(feature = "tmux_3_1")]
//Self::Backspace(value) => print_option(BACKSPACE, value),
//#[cfg(feature = "tmux_1_5")]
//Self::BufferLimit(value) => print_option(BUFFER_LIMIT, value),
//#[cfg(feature = "tmux_2_4")]
//// FIXME
//Self::CommandAlias(value) => print_array_option(COMMAND_ALIAS, value),
//#[cfg(feature = "tmux_2_1")]
//Self::DefaultTerminal(value) => print_option(DEFAULT_TERMINAL, value),
//#[cfg(feature = "tmux_1_2")]
//Self::EscapeTime(value) => print_option(ESCAPE_TIME, value),
//#[cfg(feature = "tmux_2_7")]
//Self::ExitEmpty(value) => print_option(EXIT_EMPTY, value),
//#[cfg(feature = "tmux_1_4")]
//Self::ExitUnattached(value) => print_option(EXIT_UNATTACHED, value),
//#[cfg(feature = "tmux_3_2")]
//Self::ExtendedKeys(value) => print_option(EXTENDED_KEYS, value),
//#[cfg(feature = "tmux_1_9")]
//Self::FocusEvents(value) => print_option(FOCUS_EVENTS, value),
//#[cfg(feature = "tmux_2_1")]
//Self::HistoryFile(value) => print_option(HISTORY_FILE, value),
//#[cfg(feature = "tmux_2_0")]
//Self::MessageLimit(value) => print_option(MESSAGE_LIMIT, value),
//#[cfg(feature = "tmux_1_5")]
//Self::SetClipboard(value) => print_option(SET_CLIPBOARD, value),
//#[cfg(feature = "tmux_2_0")]
//// FIXME
//Self::TerminalOverrides(value) => print_array_option(TERMINAL_OVERRIDES, value),
//#[cfg(feature = "tmux_3_0")]
//Self::UserKeys(value) => print_array_option(USER_KEYS, value),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
//Self::Quiet(value) => print_option(QUIET, value),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
//Self::DetachOnDestroy(value) => print_option(DETACH_ON_DESTROY, value),
//_ => String::new(),
//};
//write!(f, "{}", s)
//}
//}

//fn get_array_item_name<'a, T: fmt::Display>(
//name: &'a str,
//value: &Option<ArrayItem<T>>,
//) -> Cow<'a, str> {
//match value {
//Some(index) => format!("{}[{}]", name, index).into(),
//None => name.into(),
//}
//}

//impl ServerOption {
//pub fn get_by_name(name: &str) {

//}

//pub fn set_ext(cb: Option<&dyn Fn(&str, &str) -> String>) -> Result<Self, Error> {
//let s = match cb {
//Some(cb) => cb("", ""),
//None => Tmux::new()
//.command(SetOption::new().server().option().value())
//.output()?
//.to_string(),
//};
//s.parse()
//}

//pub fn get_ext<T: FromStr>(
//cb: Option<&dyn Fn(&str) -> String>,
//name: &str,
//) -> Result<T, Error> {
//let s = match cb {
//Some(cb) => cb(name),
//None => Tmux::new()
//.command(ShowOptions::new().server().option(name))
//.output()?
//.to_string(),
//};
//s.parse()
//}

// using both name and value
//pub fn get_full<T: FromStr>(name: &str) -> Result<T, Error> {
//let s = Tmux::new()
//.command(ShowOptions::new().server().value().option(name))
//.output()?
//.to_string();
//parse_option(s,)
//}

// using only tmux returned value (without option name)
//pub fn get_short(name: &str) -> Result<Self, Error> {
//let s = Tmux::new()
//.command(ShowOptions::new().server().value().option(name))
//.output()?
//.to_string();
//Self::parse_option(name, None, Some(&s))
//}

//pub fn set23<'a, S: fmt::Display, T: Into<Cow<'a, str>>>(
//name: T,
//value: Option<S>,
//) -> Result<TmuxOutput, Error> {
//let name = name;
//let value = match value {
//Some(value) => value.to_string(),
//None => "".to_string(),
//};
//Tmux::new()
//.command(SetOption::new().server().value(value).option(name))
//.output()
//}

//pub fn get2<T: FromStr>(name: ServerOptionName) -> Result<T, Error> {
//let output = Tmux::new()
//.command(ShowOptions::new().server().option(name.to_string()))
//.output()?;
//ServerOption::from_str(output)
//}

//pub fn set<S: fmt::Display>(
//self,
//name: ServerOptionName,
//value: Option<S>,
//cb: &dyn Fn(&str, Option<&str>) -> Result<TmuxOutput, Error>,
//) -> Result<TmuxOutput, Error> {
//match name {
//#[cfg(feature = "tmux_3_1")]
//ServerOptionName::Backspace => Self::set23(BACKSPACE, value),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::BufferLimit => Self::set23(BUFFER_LIMIT, value),
////#[cfg(feature = "tmux_2_4")]
////ServerOptionName::CommandAlias(i) => Self::set23(array_name(COMMAND_ALIAS, i), value),
//#[cfg(feature = "tmux_2_4")]
//ServerOptionName::CommandAlias => Self::set23(COMMAND_ALIAS, value),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::CopyCommand => Self::set23(COPY_COMMAND, value),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::DefaultTerminal => Self::set23(DEFAULT_TERMINAL, value),
//#[cfg(feature = "tmux_1_2")]
//ServerOptionName::EscapeTime => Self::set23(ESCAPE_TIME, value),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::Editor => Self::set23(EDITOR, value),
//#[cfg(feature = "tmux_2_7")]
//ServerOptionName::ExitEmpty => Self::set23(EXIT_EMPTY, value),
//#[cfg(feature = "tmux_1_4")]
//ServerOptionName::ExitUnattached => Self::set23(EXIT_UNATTACHED, value),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::ExtendedKeys => Self::set23(EXTENDED_KEYS, value),
//#[cfg(feature = "tmux_1_9")]
//ServerOptionName::FocusEvents => Self::set23(FOCUS_EVENTS, value),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::HistoryFile => Self::set23(HISTORY_FILE, value),
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::MessageLimit => Self::set23(MESSAGE_LIMIT, value),
//#[cfg(feature = "tmux_3_3")]
//ServerOptionName::PromptHistoryLimit => Self::set23(PROMPT_HISTORY_LIMIT, value),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::SetClipboard => Self::set23(SET_CLIPBOARD, value),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::TerminalFeatures(i) => {
//Self::set23(array_name(TERMINAL_FEATURES, i), value)
//}
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::TerminalOverrides(i) => {
//Self::set23(array_name(TERMINAL_OVERRIDES, i), value)
//}
//#[cfg(feature = "tmux_3_0")]
//ServerOptionName::UserKeys(i) => Self::set23(array_name(USER_KEYS, i), value),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
//ServerOptionName::Quiet => Self::set23(QUIET, value),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
//ServerOptionName::DetachOnDestroy => Self::set23(DETACH_ON_DESTROY, value),
//ServerOptionName::UserOption(name) => Self::set23(name.to_string(), value),
//}
//}

//pub fn parse_only_value(
//name: ServerOptionName,
//index: Option<usize>,
//value: Option<&str>,
//) -> Result<Self, Error> {
//match name {
//#[cfg(feature = "tmux_3_1")]
//ServerOptionName::Backspace => Ok(Self::Backspace(parse_value(value))),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::BufferLimit => Ok(Self::BufferLimit(parse_value(value))),
//// FIXME
////#[cfg(feature = "tmux_2_4")]
////ServerOptionName::CommandAlias(_) => {
////Ok(Self::CommandAlias(parse_array_value(index, value)))
////}
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::DefaultTerminal => Ok(Self::DefaultTerminal(parse_value(value))),
//#[cfg(feature = "tmux_1_2")]
//ServerOptionName::EscapeTime => Ok(Self::EscapeTime(parse_value(value))),
//#[cfg(feature = "tmux_2_7")]
//ServerOptionName::ExitEmpty => Ok(Self::ExitEmpty(parse_value(value))),
//#[cfg(feature = "tmux_1_4")]
//ServerOptionName::ExitUnattached => Ok(Self::ExitUnattached(parse_value(value))),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::ExtendedKeys => Ok(Self::ExtendedKeys(parse_value(value))),
//#[cfg(feature = "tmux_1_9")]
//ServerOptionName::FocusEvents => Ok(Self::FocusEvents(parse_value(value))),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::HistoryFile => Ok(Self::HistoryFile(parse_value(value))),
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::MessageLimit => Ok(Self::MessageLimit(parse_value(value))),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::SetClipboard => Ok(Self::SetClipboard(parse_value(value))),
//// FIXME
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::TerminalOverrides(_) => {
//Ok(Self::TerminalOverrides(parse_array_value(index, value)))
//}
//#[cfg(feature = "tmux_3_0")]
//ServerOptionName::UserKeys(_) => Ok(Self::UserKeys(parse_array_value(index, value))),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
//ServerOptionName::Quiet => Ok(Self::Quiet(parse_value(value))),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
//ServerOptionName::DetachOnDestroy => Ok(Self::DetachOnDestroy(parse_value(value))),
//_ => Err(Error::ParseStatusKeys),
//}
//}

// recognize option name, stor where? wrong way, storage equal for all
//pub fn parse_option2<T: FromStr>(
//name: &str,
//index: Option<usize>,
//value: Option<&str>,
//output: &mut Option<T>,
//) {
//match name {
//#[cfg(feature = "tmux_3_1")]
//BACKSPACE => *output = parse_value(value),
//#[cfg(feature = "tmux_1_5")]
//BUFFER_LIMIT => *output = parse_value(value),
////#[cfg(feature = "tmux_2_4")]
////COMMAND_ALIAS => *output = parse_array_value(index, value),
//#[cfg(feature = "tmux_2_1")]
//DEFAULT_TERMINAL => *output = parse_value(value),
//_ => {}
//}
//}
