use crate::{Error, SetOptionBuilder, ShowOptionsBuilder, Switch, TargetPane, TmuxInterface};
use std::fmt;
use std::str::FromStr;

const ON: &str = "on";
const OFF: &str = "off";
const EXTERNAL: &str = "external";

//set-clipboard [on | external | off]
#[derive(PartialEq, Clone, Debug)]
pub enum SetClipboard {
    On,
    Off,
    External,
}

impl FromStr for SetClipboard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            EXTERNAL => Ok(Self::External),
            _ => Err(Error::ParseSetClipboard),
        }
    }
}

impl fmt::Display for SetClipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::On => ON,
            Self::Off => OFF,
            Self::External => EXTERNAL,
        };
        f.write_str(value)
    }
}

pub const BACKSPACE: usize = 1 << 0;
pub const BUFFER_LIMIT: usize = 1 << 1;
pub const COMMAND_ALIAS: usize = 1 << 2;
pub const DEFAULT_TERMINAL: usize = 1 << 3;
pub const ESCAPE_TIME: usize = 1 << 4;
pub const EXIT_EMPTY: usize = 1 << 5;
pub const EXIT_UNATTACHED: usize = 1 << 6;
pub const FOCUS_EVENTS: usize = 1 << 7;
pub const HISTORY_FILE: usize = 1 << 8;
pub const MESSAGE_LIMIT: usize = 1 << 9;
pub const SET_CLIPBOARD: usize = 1 << 10;
pub const TERMINAL_OVERRIDES: usize = 1 << 11;
pub const USER_KEYS: usize = 1 << 12;

pub const SERVER_OPTIONS_NONE: usize = 0;
//pub const SERVER_OPTIONS_DEFAULT: usize = ;
pub const SERVER_OPTIONS_ALL: usize = BACKSPACE
    | BUFFER_LIMIT
    | COMMAND_ALIAS
    | DEFAULT_TERMINAL
    | ESCAPE_TIME
    | EXIT_EMPTY
    | EXIT_UNATTACHED
    | FOCUS_EVENTS
    | HISTORY_FILE
    | MESSAGE_LIMIT
    | SET_CLIPBOARD
    | TERMINAL_OVERRIDES
    | USER_KEYS;

pub const SERVER_OPTIONS_NUM: usize = 13;

// array [0. tmux string, 1. parsing method, 2. string formatting method, 3. ID or bitmask]
pub const SERVER_OPTIONS: [(
    &str,
    fn(o: &mut ServerOptions, s: &str),
    fn(o: &ServerOptions) -> Option<String>,
    usize,
); SERVER_OPTIONS_NUM] = [
    (
        "backspace",
        |o, s| o.backspace = s.parse().ok(),
        |o| o.backspace.as_ref().map(|v| v.to_string()),
        BACKSPACE,
    ),
    (
        "buffer-limit",
        |o, s| o.buffer_limit = s.parse().ok(),
        |o| o.buffer_limit.as_ref().map(|v| v.to_string()),
        BUFFER_LIMIT,
    ),
    (
        "command-alias",
        |o, _s| o.command_alias = None,
        |o| o.command_alias.as_ref().map(|v| v.join(" ").to_string()),
        COMMAND_ALIAS,
    ),
    (
        "default-terminal",
        |o, s| o.default_terminal = s.parse().ok(),
        |o| o.default_terminal.as_ref().map(|v| v.to_string()),
        DEFAULT_TERMINAL,
    ),
    (
        "escape-time",
        |o, s| o.escape_time = s.parse().ok(),
        |o| o.escape_time.as_ref().map(|v| v.to_string()),
        ESCAPE_TIME,
    ),
    (
        "exit-empty",
        |o, s| o.exit_empty = s.parse().ok(),
        |o| o.exit_empty.as_ref().map(|v| v.to_string()),
        EXIT_EMPTY,
    ),
    (
        "exit-unattached",
        |o, s| o.exit_unattached = s.parse().ok(),
        |o| o.exit_unattached.as_ref().map(|v| v.to_string()),
        EXIT_UNATTACHED,
    ),
    (
        "focus-events",
        |o, s| o.focus_events = s.parse().ok(),
        |o| o.focus_events.as_ref().map(|v| v.to_string()),
        FOCUS_EVENTS,
    ),
    (
        "history-file",
        |o, s| o.history_file = s.parse().ok(),
        |o| o.history_file.as_ref().map(|v| v.to_string()),
        HISTORY_FILE,
    ),
    (
        "message-limit",
        |o, s| o.message_limit = s.parse().ok(),
        |o| o.message_limit.as_ref().map(|v| v.to_string()),
        MESSAGE_LIMIT,
    ),
    (
        "set-clipboard",
        |o, s| o.set_clipboard = s.parse().ok(),
        |o| o.set_clipboard.as_ref().map(|v| v.to_string()),
        SET_CLIPBOARD,
    ),
    (
        "terminal-overrides",
        |o, _s| o.terminal_overrides = None,
        |o| {
            o.terminal_overrides
                .as_ref()
                .map(|v| v.join(" ").to_string())
        },
        TERMINAL_OVERRIDES,
    ),
    (
        "user-keys",
        |o, _s| o.user_keys = None,
        |o| o.user_keys.as_ref().map(|v| v.join(" ").to_string()),
        USER_KEYS,
    ),
];

// TODO: Vec variables solution
// TODO: check types
// 13 Available server options are:
#[derive(Default, PartialEq, Clone, Debug)]
pub struct ServerOptions {
    // backspace key
    pub backspace: Option<String>,
    // buffer-limit number
    pub buffer_limit: Option<usize>,
    // command-alias[] name=value
    pub command_alias: Option<Vec<String>>,
    // default-terminal terminal
    pub default_terminal: Option<String>,
    //escape-time time
    pub escape_time: Option<usize>,
    //exit-empty [on | off]
    pub exit_empty: Option<Switch>,
    //exit-unattached [on | off]
    pub exit_unattached: Option<Switch>,
    //focus-events [on | off]
    pub focus_events: Option<Switch>,
    //history-file path
    pub history_file: Option<String>,
    //message-limit number
    pub message_limit: Option<usize>,
    //set-clipboard [on | external | off]
    pub set_clipboard: Option<SetClipboard>,
    //terminal-overrides[] string
    pub terminal_overrides: Option<Vec<String>>,
    //user-keys[] key
    pub user_keys: Option<Vec<String>>,
}

impl ServerOptions {
    // faster than SERVER_OPTIONS_ALL bitmask if will be implemented selective
    pub fn get_all() -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let show_options = ShowOptionsBuilder::<TargetPane>::new().server().build();
        let s = tmux.show_options(Some(&show_options))?;
        s.parse()
    }

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    pub fn get(bitflags: usize) -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let selected_option = SERVER_OPTIONS
            .iter()
            .filter(|t| bitflags == t.3)
            .map(|t| format!("{}", t.0))
            .collect::<Vec<String>>()
            .join(" ");
        let show_options = ShowOptionsBuilder::<TargetPane>::new()
            .server()
            .option(&selected_option)
            .build();
        let s = tmux.show_options(Some(&show_options))?;
        s.parse()
    }

    // XXX: add selective multiple vars and single methods
    //pub fn get(bitflags: usize) -> Result<Self, Error> {
    //let mut tmux = TmuxInterface::new();
    //for selected_option in SERVER_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
    //let show_options = ShowOptionsBuilder::<TargetPane>::new()
    //.server()
    //.option(&selected_option.0)
    //.build();
    //let s = tmux.show_options(Some(&show_options))?;
    //s.parse::<ServerOptions>();
    //}
    //Err(Error::new(""))
    //}

    // allows selective set by bitmask
    pub fn set(&self, bitflags: usize) -> Result<(), Error> {
        let mut tmux = TmuxInterface::new();
        for selected_option in SERVER_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
            if let Some(selected_value) = selected_option.2(&self) {
                let set_option = SetOptionBuilder::<TargetPane>::new().server().build();
                tmux.set_option(Some(&set_option), selected_option.0, &selected_value)?;
            }
        }
        Ok(())
    }

    // XXX: mb methods for all fields set get?
}

impl FromStr for ServerOptions {
    type Err = Error;

    fn from_str(options: &str) -> Result<Self, Self::Err> {
        let mut server_options: ServerOptions = Default::default();
        let mut v: Vec<&str>;
        for option in options.lines() {
            v = option.trim().split(' ').collect();
            for server_var in SERVER_OPTIONS.iter() {
                if server_var.0 == v[0] {
                    server_var.1(&mut server_options, v[1])
                }
            }
        }
        Ok(server_options)
    }
}

impl fmt::Display for ServerOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // server option
        for var in SERVER_OPTIONS.iter() {
            // if is set some - extract
            if let Some(ref v) = var.2(self) {
                write!(f, "{} {}\n", var.0, v)?;
            }
        }
        write!(f, "{}", "")
    }
}

#[derive(Default, Debug)]
pub struct ServerOptionsBuilder<'a> {
    pub backspace: Option<&'a str>,
    pub buffer_limit: Option<usize>,
    pub command_alias: Option<Vec<String>>,
    pub default_terminal: Option<&'a str>,
    pub escape_time: Option<usize>,
    pub exit_empty: Option<Switch>,
    pub exit_unattached: Option<Switch>,
    pub focus_events: Option<Switch>,
    pub history_file: Option<&'a str>,
    pub message_limit: Option<usize>,
    pub set_clipboard: Option<SetClipboard>,
    pub terminal_overrides: Option<Vec<String>>,
    pub user_keys: Option<Vec<String>>,
}

impl<'a> ServerOptionsBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn backspace(&mut self, backspace: &'a str) -> &mut Self {
        self.backspace = Some(backspace);
        self
    }

    pub fn buffer_limit(&mut self, buffer_limit: usize) -> &mut Self {
        self.buffer_limit = Some(buffer_limit);
        self
    }

    pub fn command_alias(&mut self, command_alias: Vec<String>) -> &mut Self {
        self.command_alias = Some(command_alias);
        self
    }

    pub fn default_terminal(&mut self, default_terminal: &'a str) -> &mut Self {
        self.default_terminal = Some(default_terminal);
        self
    }

    pub fn escape_time(&mut self, escape_time: usize) -> &mut Self {
        self.escape_time = Some(escape_time);
        self
    }

    pub fn exit_empty(&mut self, exit_empty: Switch) -> &mut Self {
        self.exit_empty = Some(exit_empty);
        self
    }

    pub fn exit_unattached(&mut self, exit_unattached: Switch) -> &mut Self {
        self.exit_unattached = Some(exit_unattached);
        self
    }

    pub fn focus_events(&mut self, focus_events: Switch) -> &mut Self {
        self.focus_events = Some(focus_events);
        self
    }

    pub fn history_file(&mut self, history_file: &'a str) -> &mut Self {
        self.history_file = Some(history_file);
        self
    }

    pub fn message_limit(&mut self, message_limit: usize) -> &mut Self {
        self.message_limit = Some(message_limit);
        self
    }

    pub fn set_clipboard(&mut self, set_clipboard: SetClipboard) -> &mut Self {
        self.set_clipboard = Some(set_clipboard);
        self
    }

    pub fn terminal_overrides(&mut self, terminal_overrides: Vec<String>) -> &mut Self {
        self.terminal_overrides = Some(terminal_overrides);
        self
    }

    pub fn user_keys(&mut self, user_keys: Vec<String>) -> &mut Self {
        self.user_keys = Some(user_keys);
        self
    }

    // XXX: clone rly needed?
    pub fn build(&self) -> ServerOptions {
        ServerOptions {
            backspace: self.backspace.map(|s| s.to_string()),
            buffer_limit: self.buffer_limit,
            command_alias: self.command_alias.clone(),
            default_terminal: self.default_terminal.map(|s| s.to_string()),
            escape_time: self.escape_time,
            exit_empty: self.exit_empty.clone(),
            exit_unattached: self.exit_unattached.clone(),
            focus_events: self.focus_events.clone(),
            history_file: self.history_file.map(|s| s.to_string()),
            message_limit: self.message_limit,
            set_clipboard: self.set_clipboard.clone(),
            terminal_overrides: self.terminal_overrides.clone(),
            user_keys: self.user_keys.clone(),
        }
    }
}
