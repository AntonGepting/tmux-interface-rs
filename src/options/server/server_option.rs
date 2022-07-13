use super::*;
use crate::{Error, SetOption, ShowOptions, Switch, Tmux, TmuxOutput};
use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

// FIXME: proper Error in return

// total num: 16
#[cfg(feature = "tmux_3_1")]
pub const BACKSPACE: &str = "backspace";
#[cfg(feature = "tmux_1_5")]
pub const BUFFER_LIMIT: &str = "buffer-limit";
#[cfg(feature = "tmux_2_4")]
pub const COMMAND_ALIAS: &str = "command-alias";
#[cfg(feature = "tmux_2_1")]
pub const DEFAULT_TERMINAL: &str = "default-terminal";
#[cfg(feature = "tmux_1_2")]
pub const ESCAPE_TIME: &str = "escape-time";
#[cfg(feature = "tmux_2_7")]
pub const EXIT_EMPTY: &str = "exit-empty";
#[cfg(feature = "tmux_1_4")]
pub const EXIT_UNATTACHED: &str = "exit-unattached";
#[cfg(feature = "tmux_3_2")]
pub const EXTENDED_KEYS: &str = "extended-keys";
#[cfg(feature = "tmux_1_9")]
pub const FOCUS_EVENTS: &str = "focus-events";
#[cfg(feature = "tmux_2_1")]
pub const HISTORY_FILE: &str = "history-file";
#[cfg(feature = "tmux_2_0")]
pub const MESSAGE_LIMIT: &str = "message-limit";
#[cfg(feature = "tmux_1_5")]
pub const SET_CLIPBOARD: &str = "set-clipboard";
#[cfg(feature = "tmux_2_0")]
pub const TERMINAL_OVERRIDES: &str = "terminal-overrides";
#[cfg(feature = "tmux_3_0")]
pub const USER_KEYS: &str = "user-keys";
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
pub const QUIET: &str = "quiet";
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
pub const DETACH_ON_DESTROY: &str = "detach-on-destroy";

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

pub struct OptionsController<'a> {
    pub setter: &'a dyn Fn(&str) -> String,
    pub getter: &'a dyn Fn(&str) -> String,
}

impl<'a> OptionsController<'a> {
    pub fn new() -> Self {
        Self {
            setter: &Self::default_getter,
            getter: &Self::default_getter,
        }
    }

    fn default_getter(name: &str) -> String {
        Tmux::new()
            .command(SetOption::new().server().option(name))
            .output()
            .unwrap()
            .to_string()
    }

    pub fn getaaaa(tmux: Option<Tmux>, name: &str) {
        let show_options = ShowOptions::new().server().option(name);
        let tmux = match tmux {
            Some(tmux) => tmux.command(show_options),
            None => Tmux::new().command(show_options),
        };
        tmux.output().unwrap().to_string();
    }

    pub fn setter(&mut self, setter: &'a dyn Fn(&str) -> String) -> &mut Self {
        self.setter = setter;
        self
    }

    pub fn getter(&mut self, getter: &'a dyn Fn(&str) -> String) -> &mut Self {
        self.getter = getter;
        self
    }
}

impl<'a> ServerOptionController<'a> {
    pub fn new() -> Self {}
}

impl<T: fmt::Display> TmuxServerOption2<T> {
    pub fn buffer_limit() {}

    pub fn get(&self, name: ServerOptionName) -> Result<ServerOption, Error> {
        Tmux::new()
            .command(ShowOptions::new().server().option(name))
            .output()?
            .to_string()
            .parse()
    }

    pub fn set(&self) -> Result<(), Error> {
        Tmux::new()
            .command(SetOption::new().server().option(name).value())
            .output()?;
        Ok(())
    }
}

// variants possible:
// * option_name value
// * option_name
//
pub enum ServerOption {
    // backspace key
    #[cfg(feature = "tmux_3_1")]
    Backspace(Option<String>),
    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    BufferLimit(Option<usize>),
    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    CommandAlias(Option<ArrayItem<String>>),
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
    TerminalOverrides(Option<ArrayItem<String>>),
    //user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    UserKeys(Option<Vec<String>>),
    // quiet ?
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    Quiet(Option<Switch>),
    // detach-on-destroy ?
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    DetachOnDestroy(Option<Switch>),
    // user option
    UserOption(Option<String>),
}

pub struct ArrayItem<T> {
    pub index: usize,
    pub data: T,
}

impl<T: fmt::Display> fmt::Display for ArrayItem<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        //if let Some(i) = self.index {
        s = format!("[{}]", self.index);
        //}

        //if let Some(data) = self.data {
        s = format!("{} {}", s, self.data);
        //}

        write!(f, "{}", s)
    }
}

impl<T: fmt::Display> ArrayItem<T> {
    //
    pub fn create_name_with_index<'a>(&self, name: &'a str) -> Cow<'a, str> {
        //match self.index {
        //Some(i) => format!("{}[{}]", name, i).into(),
        //None => name.into(),
        //}
        format!("{}[{}]", name, self.index).into()
    }

    pub fn to_string_with_name(&self, name: &str) -> String {
        let mut s = name.to_string();
        //if let Some(i) = self.index {
        s = format!("{}[{}]", s, self.index);
        //}
        //if let Some(data) = self.data {
        s = format!("{} {}", s, self.data);
        //}
        s
    }
}

// mb common
// if option value is not set, print only name
fn print_option<S: ToString>(name: &str, value: &Option<S>) -> String {
    match value {
        Some(value) => format!("{} {}", name, value.to_string()),
        None => format!("{}", name),
    }
}

// if option value is not set, print only name
fn print_array_option<T: fmt::Display>(name: &str, value: &Option<ArrayItem<T>>) -> String {
    match value {
        Some(value) => value.to_string_with_name(name),
        None => format!("{}", name),
    }
}

//fn print_option_array_item(name: &str, array: Vec<String>, index: usize) -> String {
//let item = array.get(index);
//match item {
//Some(data) => format!("{}[{}] {}", name, index, data),
//None => format!("{}[{}]", name, index),
//}
//}
//fn get_name_value<S: ToString>(name: &str, value: Option<S>) -> {

//}

// mb common
fn parse_value<T: FromStr>(value: Option<&str>) -> Option<T> {
    value.and_then(|data| data.parse().ok())
}

impl fmt::Display for ServerOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            #[cfg(feature = "tmux_3_1")]
            Self::Backspace(value) => print_option(BACKSPACE, value),
            #[cfg(feature = "tmux_1_5")]
            Self::BufferLimit(value) => print_option(BUFFER_LIMIT, value),
            #[cfg(feature = "tmux_2_4")]
            // FIXME
            Self::CommandAlias(value) => print_array_option(COMMAND_ALIAS, value),
            #[cfg(feature = "tmux_2_1")]
            Self::DefaultTerminal(value) => print_option(DEFAULT_TERMINAL, value),
            #[cfg(feature = "tmux_1_2")]
            Self::EscapeTime(value) => print_option(ESCAPE_TIME, value),
            #[cfg(feature = "tmux_2_7")]
            Self::ExitEmpty(value) => print_option(EXIT_EMPTY, value),
            #[cfg(feature = "tmux_1_4")]
            Self::ExitUnattached(value) => print_option(EXIT_UNATTACHED, value),
            #[cfg(feature = "tmux_3_2")]
            Self::ExtendedKeys(value) => print_option(EXTENDED_KEYS, value),
            #[cfg(feature = "tmux_1_9")]
            Self::FocusEvents(value) => print_option(FOCUS_EVENTS, value),
            #[cfg(feature = "tmux_2_1")]
            Self::HistoryFile(value) => print_option(HISTORY_FILE, value),
            #[cfg(feature = "tmux_2_0")]
            Self::MessageLimit(value) => print_option(MESSAGE_LIMIT, value),
            #[cfg(feature = "tmux_1_5")]
            Self::SetClipboard(value) => print_option(SET_CLIPBOARD, value),
            #[cfg(feature = "tmux_2_0")]
            // FIXME
            Self::TerminalOverrides(value) => print_array_option(TERMINAL_OVERRIDES, value),
            #[cfg(feature = "tmux_3_0")]
            Self::UserKeys(value) => print_option(USER_KEYS, value),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            Self::Quiet(value) => print_option(QUIET, value),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            Self::DetachOnDestroy(value) => print_option(DETACH_ON_DESTROY, value),
            _ => String::new(),
        };
        write!(f, "{}", s)
    }
}

fn get_array_item_name<'a, T: fmt::Display>(
    name: &'a str,
    value: &Option<ArrayItem<T>>,
) -> Cow<'a, str> {
    match value {
        Some(index) => format!("{}[{}]", name, index).into(),
        None => name.into(),
    }
}

impl ServerOption {
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

    pub fn get_ext(cb: Option<&dyn Fn(&str) -> String>, name: &str) -> Result<Self, Error> {
        let s = match cb {
            Some(cb) => cb(name),
            None => Tmux::new()
                .command(ShowOptions::new().server().option(name))
                .output()?
                .to_string(),
        };
        s.parse()
    }

    // using both name and value
    pub fn get_full(name: &str) -> Result<Self, Error> {
        let s = Tmux::new()
            .command(ShowOptions::new().server().value().option(name))
            .output()?
            .to_string();
        s.parse()
    }

    // using only tmux returned value (without option name)
    pub fn get_short(name: &str) -> Result<Self, Error> {
        let s = Tmux::new()
            .command(ShowOptions::new().server().value().option(name))
            .output()?
            .to_string();
        Self::parse_option(name, None, Some(&s))
    }

    pub fn set23<'a, S: fmt::Display, T: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> Result<TmuxOutput, Error> {
        let name = name;
        let value = match value {
            Some(value) => value.to_string(),
            None => "".to_string(),
        };
        Tmux::new()
            .command(SetOption::new().server().value(value).option(name))
            .output()
    }

    pub fn set(
        self,
        cb: &dyn Fn(&str, Option<&str>) -> Result<TmuxOutput, Error>,
    ) -> Result<TmuxOutput, Error> {
        match self {
            #[cfg(feature = "tmux_3_1")]
            Self::Backspace(value) => Self::set23(BACKSPACE, value),
            #[cfg(feature = "tmux_1_5")]
            Self::BufferLimit(value) => Self::set23(BUFFER_LIMIT, value),
            #[cfg(feature = "tmux_2_4")]
            Self::CommandAlias(value) => {
                Self::set23(get_array_item_name(COMMAND_ALIAS, &value), value)
            }
            #[cfg(feature = "tmux_2_1")]
            Self::DefaultTerminal(value) => Self::set23(DEFAULT_TERMINAL, value),
            #[cfg(feature = "tmux_1_2")]
            Self::EscapeTime(value) => Self::set23(ESCAPE_TIME, value),
            #[cfg(feature = "tmux_2_7")]
            Self::ExitEmpty(value) => Self::set23(EXIT_EMPTY, value),
            #[cfg(feature = "tmux_1_4")]
            Self::ExitUnattached(value) => Self::set23(EXIT_UNATTACHED, value),
            #[cfg(feature = "tmux_3_2")]
            Self::ExtendedKeys(value) => Self::set23(EXTENDED_KEYS, value),
            #[cfg(feature = "tmux_1_9")]
            Self::FocusEvents(value) => Self::set23(FOCUS_EVENTS, value),
            #[cfg(feature = "tmux_2_1")]
            Self::HistoryFile(value) => Self::set23(HISTORY_FILE, value),
            #[cfg(feature = "tmux_2_0")]
            Self::MessageLimit(value) => Self::set23(MESSAGE_LIMIT, value),
            #[cfg(feature = "tmux_1_5")]
            Self::SetClipboard(value) => Self::set23(SET_CLIPBOARD, value),
            #[cfg(feature = "tmux_2_0")]
            Self::TerminalOverrides(value) => {
                Self::set23(get_array_item_name(TERMINAL_OVERRIDES, &value), value)
            }
            #[cfg(feature = "tmux_3_0")]
            Self::UserKeys(value) => Self::set23(USER_KEYS, value),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            Self::Quiet(value) => Self::set23(QUIET, value),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            Self::DetachOnDestroy(value) => Self::set23(DETACH_ON_DESTROY, value),
            _ => Err(Error::ParseStatusKeys),
        }
        //Tmux::new()
        //.command(SetOption::new().server().value().option(name))
        //.output()
    }

    pub fn parse_option(
        name: &str,
        index: Option<usize>,
        value: Option<&str>,
    ) -> Result<ServerOption, Error> {
        match name {
            #[cfg(feature = "tmux_3_1")]
            BACKSPACE => Ok(Self::Backspace(parse_value(value))),
            #[cfg(feature = "tmux_1_5")]
            BUFFER_LIMIT => Ok(Self::BufferLimit(parse_value(value))),
            // FIXME
            //#[cfg(feature = "tmux_2_4")]
            //COMMAND_ALIAS => Ok(Self::CommandAlias(parse_value(value))),
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
            // FIXME
            //#[cfg(feature = "tmux_2_0")]
            //TERMINAL_OVERRIDES => Ok(Self::TerminalOverrides(v[1].parse().ok())),
            #[cfg(feature = "tmux_3_0")]
            USER_KEYS => Ok(Self::UserKeys(parse_value(value))),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            QUIET => Ok(Self::Quiet(parse_value(value))),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            DETACH_ON_DESTROY => Ok(Self::DetachOnDestroy(parse_value(value))),
            _ => Err(Error::ParseStatusKeys),
        }
    }
}

const SEPARATOR: &str = " ";

// split string in 3 parts, name, index (if an array), value
fn get_option(s: &str) -> (Option<&str>, Option<usize>, Option<&str>) {
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

impl FromStr for ServerOption {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, i, value) = get_option(s);

        match name {
            Some(name) => Self::parse_option(name, i, value),
            None => Err(Error::ParseStatusKeys),
        }
    }
}
