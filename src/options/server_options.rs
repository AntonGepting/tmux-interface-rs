#[cfg(feature = "tmux_2_0")]
use super::create_insert_vec;
use crate::{Error, SetOption, ShowOptions, Switch};
use std::fmt;
use std::str::FromStr;

const ON: &str = "on";
const OFF: &str = "off";
#[cfg(feature = "tmux_2_6")]
const EXTERNAL: &str = "external";

//set-clipboard [on | external | off]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_1_5")]
pub enum SetClipboard {
    On,
    Off,
    #[cfg(feature = "tmux_2_6")]
    External,
}

#[cfg(feature = "tmux_1_5")]
impl FromStr for SetClipboard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            #[cfg(feature = "tmux_2_6")]
            EXTERNAL => Ok(Self::External),
            _ => Err(Error::ParseSetClipboard),
        }
    }
}

#[cfg(feature = "tmux_1_5")]
impl fmt::Display for SetClipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::On => ON,
            Self::Off => OFF,
            #[cfg(feature = "tmux_2_6")]
            Self::External => EXTERNAL,
        };
        f.write_str(value)
    }
}

// XXX: conditionals?
// NOTE: total num: 14 (usize)
pub const BACKSPACE: usize = 1;
pub const BUFFER_LIMIT: usize = 1 << 1;
pub const COMMAND_ALIAS: usize = 1 << 2;
pub const DEFAULT_TERMINAL: usize = 1 << 3;
pub const ESCAPE_TIME: usize = 1 << 4;
pub const EXIT_EMPTY: usize = 1 << 5;
pub const EXIT_UNATTACHED: usize = 1 << 6;
pub const FOCUS_EVENTS: usize = 1 << 7;
pub const HISTORY_FILE: usize = 1 << 8;
pub const MESSAGE_LIMIT: usize = 1 << 9;
pub const QUIET: usize = 1 << 10;
pub const SET_CLIPBOARD: usize = 1 << 11;
pub const TERMINAL_OVERRIDES: usize = 1 << 12;
pub const USER_KEYS: usize = 1 << 13;
pub const DETACH_ON_DESTROY: usize = 1 << 14;

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
    | QUIET
    | SET_CLIPBOARD
    | TERMINAL_OVERRIDES
    | USER_KEYS
    | DETACH_ON_DESTROY;

#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
pub const SERVER_OPTIONS_NUM: usize = 0;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_3")))]
pub const SERVER_OPTIONS_NUM: usize = 2;
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_5")))]
pub const SERVER_OPTIONS_NUM: usize = 3;
//#[cfg(all(feature = "tmux_1_4", not(feature = "tmux_1_5")))]
//pub const SERVER_OPTIONS_NUM: usize = 3;
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_1_9")))]
pub const SERVER_OPTIONS_NUM: usize = 5;
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub const SERVER_OPTIONS_NUM: usize = 5;
//#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_9")))]
//pub const SERVER_OPTIONS_NUM: usize = 5;
//#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
//pub const SERVER_OPTIONS_NUM: usize = 5;
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_1_9a")))]
pub const SERVER_OPTIONS_NUM: usize = 6;
#[cfg(all(feature = "tmux_1_9a", not(feature = "tmux_2_0")))]
pub const SERVER_OPTIONS_NUM: usize = 6;
#[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_1")))]
pub const SERVER_OPTIONS_NUM: usize = 7;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
pub const SERVER_OPTIONS_NUM: usize = 9;
//#[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
//pub const SERVER_OPTIONS_NUM: usize = 9;
//#[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
//pub const SERVER_OPTIONS_NUM: usize = 9;
#[cfg(all(feature = "tmux_2_4", not(feature = "tmux_2_7")))]
pub const SERVER_OPTIONS_NUM: usize = 10;
//#[cfg(all(feature = "tmux_2_5", not(feature = "tmux_2_7")))]
//pub const SERVER_OPTIONS_NUM: usize = 10;
//#[cfg(all(feature = "tmux_2_5", not(feature = "tmux_2_7")))]
//pub const SERVER_OPTIONS_NUM: usize = 10;
//#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_2_7")))]
//pub const SERVER_OPTIONS_NUM: usize = 10;
#[cfg(all(feature = "tmux_2_7", not(feature = "tmux_3_0")))]
pub const SERVER_OPTIONS_NUM: usize = 11;
//#[cfg(all(feature = "tmux_2_8", not(feature = "tmux_2_9")))]
//pub const SERVER_OPTIONS_NUM: usize = 11;
//#[cfg(all(feature = "tmux_2_9", not(feature = "tmux_2_9a")))]
//pub const SERVER_OPTIONS_NUM: usize = 11;
//#[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_0")))]
//pub const SERVER_OPTIONS_NUM: usize = 11;
#[cfg(all(feature = "tmux_3_0", not(feature = "tmux_3_1")))]
pub const SERVER_OPTIONS_NUM: usize = 12;
//#[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_1")))]
//pub const SERVER_OPTIONS_NUM: usize = 12;
#[cfg(all(feature = "tmux_3_1", not(feature = "tmux_X_X")))]
pub const SERVER_OPTIONS_NUM: usize = 13;
#[cfg(feature = "tmux_X_X")]
pub const SERVER_OPTIONS_NUM: usize = 13; // FIXME: 15 in master

// array [0. tmux string, 1. parsing method, 2. string formatting method, 3. ID or bitmask]
// TODO: array to Vec or again structure from_str
pub const SERVER_OPTIONS: [(
    &str,
    fn(o: &mut ServerOptions, i: Option<usize>, s: &str),
    fn(o: &ServerOptions) -> Option<String>,
    usize,
); SERVER_OPTIONS_NUM] = [
    #[cfg(feature = "tmux_3_1")]
    (
        "backspace",
        |o, _, s| o.backspace = s.parse().ok(),
        |o| o.backspace.as_ref().map(|v| v.to_string()),
        BACKSPACE,
    ),
    #[cfg(feature = "tmux_1_5")]
    (
        "buffer-limit",
        |o, _, s| o.buffer_limit = s.parse().ok(),
        |o| o.buffer_limit.as_ref().map(|v| v.to_string()),
        BUFFER_LIMIT,
    ),
    #[cfg(feature = "tmux_2_4")]
    (
        "command-alias",
        |o, i, s| o.command_alias = create_insert_vec(o.command_alias.as_mut(), i, s),
        |o| o.command_alias.as_ref().map(|v| v.join(" ")),
        COMMAND_ALIAS,
    ),
    #[cfg(feature = "tmux_2_1")]
    (
        "default-terminal",
        |o, _, s| o.default_terminal = s.parse().ok(),
        |o| o.default_terminal.as_ref().map(|v| v.to_string()),
        DEFAULT_TERMINAL,
    ),
    #[cfg(feature = "tmux_1_2")]
    (
        "escape-time",
        |o, _, s| o.escape_time = s.parse().ok(),
        |o| o.escape_time.as_ref().map(|v| v.to_string()),
        ESCAPE_TIME,
    ),
    #[cfg(feature = "tmux_2_7")]
    (
        "exit-empty",
        |o, _, s| o.exit_empty = s.parse().ok(),
        |o| o.exit_empty.as_ref().map(|v| v.to_string()),
        EXIT_EMPTY,
    ),
    #[cfg(feature = "tmux_1_4")]
    (
        "exit-unattached",
        |o, _, s| o.exit_unattached = s.parse().ok(),
        |o| o.exit_unattached.as_ref().map(|v| v.to_string()),
        EXIT_UNATTACHED,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "focus-events",
        |o, _, s| o.focus_events = s.parse().ok(),
        |o| o.focus_events.as_ref().map(|v| v.to_string()),
        FOCUS_EVENTS,
    ),
    #[cfg(feature = "tmux_2_1")]
    (
        "history-file",
        |o, _, s| o.history_file = s.parse().ok(),
        |o| o.history_file.as_ref().map(|v| v.to_string()),
        HISTORY_FILE,
    ),
    #[cfg(feature = "tmux_2_0")]
    (
        "message-limit",
        |o, _, s| o.message_limit = s.parse().ok(),
        |o| o.message_limit.as_ref().map(|v| v.to_string()),
        MESSAGE_LIMIT,
    ),
    #[cfg(feature = "tmux_1_5")]
    (
        "set-clipboard",
        |o, _, s| o.set_clipboard = s.parse().ok(),
        |o| o.set_clipboard.as_ref().map(|v| v.to_string()),
        SET_CLIPBOARD,
    ),
    #[cfg(feature = "tmux_2_0")]
    (
        "terminal-overrides",
        |o, i, s| o.terminal_overrides = create_insert_vec(o.terminal_overrides.as_mut(), i, s),
        |o| o.terminal_overrides.as_ref().map(|v| v.join(" ")),
        TERMINAL_OVERRIDES,
    ),
    #[cfg(feature = "tmux_3_0")]
    (
        "user-keys",
        |o, i, s| o.user_keys = create_insert_vec(o.user_keys.as_mut(), i, s),
        |o| o.user_keys.as_ref().map(|v| v.join(" ").to_string()),
        USER_KEYS,
    ),
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    (
        "quiet",
        |o, _, s| o.quiet = s.parse().ok(),
        |o| o.quiet.as_ref().map(|v| v.to_string()),
        QUIET,
    ),
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    (
        "detach-on-destroy",
        |o, i, s| o.detach_on_destroy = s.parse().ok(),
        |o| o.detach_on_destroy.as_ref().map(|v| v.to_string()),
        DETACH_ON_DESTROY,
    ),
];

// XXX: ugly array implementation?
// XXX: String vs &str, send/recieve like 2 structures
// XXX: String vs &str optimization
// TODO: Vec variables solution for arrays
// TODO: check types
// TODO: command_alias and terminal_overrides both as String and as Vec<String> see tmux versions
#[derive(Default, PartialEq, Clone, Debug)]
pub struct ServerOptions {
    // backspace key
    #[cfg(feature = "tmux_3_1")]
    pub backspace: Option<String>,
    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    pub buffer_limit: Option<usize>,
    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    pub command_alias: Option<Vec<String>>,
    // default-terminal terminal
    #[cfg(feature = "tmux_2_1")]
    pub default_terminal: Option<String>,
    //escape-time time
    #[cfg(feature = "tmux_1_2")]
    pub escape_time: Option<usize>,
    //exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    pub exit_empty: Option<Switch>,
    //exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    pub exit_unattached: Option<Switch>,
    //focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    pub focus_events: Option<Switch>,
    //history-file path
    #[cfg(feature = "tmux_2_1")]
    pub history_file: Option<String>,
    //message-limit number
    #[cfg(feature = "tmux_2_0")]
    pub message_limit: Option<usize>,
    //set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    pub set_clipboard: Option<SetClipboard>,
    //terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    pub terminal_overrides: Option<Vec<String>>,
    //user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    pub user_keys: Option<Vec<String>>,
    // quiet ?
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub quiet: Option<Switch>,
    // detach-on-destroy ?
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub detach_on_destroy: Option<Switch>,
    //pub user_options: Option<HashMap<String, String>>
}

impl ServerOptions {
    // faster than SERVER_OPTIONS_ALL bitmask if will be implemented selective
    pub fn get_all() -> Result<Self, Error> {
        ShowOptions::new().server().output()?.to_string().parse()
    }

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    // FIXME: tmux v1.6 option attribute not exists, all options will be showed
    #[cfg(feature = "tmux_1_7")]
    pub fn get(bitflags: usize) -> Result<Self, Error> {
        let selected_option = SERVER_OPTIONS
            .iter()
            .filter(|t| bitflags == t.3)
            .map(|t| t.0.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        ShowOptions::new()
            .server()
            .option(&selected_option)
            .output()?
            .to_string()
            .parse()
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
        for selected_option in SERVER_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
            if let Some(selected_value) = selected_option.2(&self) {
                SetOption::new()
                    .server()
                    .option(selected_option.0)
                    .value(&selected_value)
                    .output()?;
            }
        }
        Ok(())
    }

    // XXX: mb methods for all fields set get?
}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?
impl FromStr for ServerOptions {
    type Err = Error;

    fn from_str(options: &str) -> Result<Self, Self::Err> {
        let mut server_options: ServerOptions = Default::default();
        let mut v: Vec<&str>;
        let mut arr: Vec<&str>;
        for option in options.lines() {
            v = option.trim().splitn(2, ' ').collect();
            arr = v[0].split(|c| c == '[' || c == ']').collect();
            for server_var in SERVER_OPTIONS.iter() {
                if server_var.0 == arr[0] {
                    server_var.1(
                        &mut server_options,
                        arr.get(1).and_then(|i| i.parse::<usize>().ok()),
                        v.get(1).unwrap_or(&""),
                    )
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
                writeln!(f, "{} {}", var.0, v)?;
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ServerOptionsBuilder<'a> {
    #[cfg(feature = "tmux_3_1")]
    pub backspace: Option<&'a str>,
    #[cfg(feature = "tmux_1_5")]
    pub buffer_limit: Option<usize>,
    #[cfg(feature = "tmux_2_4")]
    pub command_alias: Option<Vec<String>>,
    #[cfg(feature = "tmux_2_1")]
    pub default_terminal: Option<&'a str>,
    #[cfg(feature = "tmux_1_2")]
    pub escape_time: Option<usize>,
    #[cfg(feature = "tmux_2_7")]
    pub exit_empty: Option<Switch>,
    #[cfg(feature = "tmux_1_4")]
    pub exit_unattached: Option<Switch>,
    #[cfg(feature = "tmux_1_9")]
    pub focus_events: Option<Switch>,
    #[cfg(feature = "tmux_2_1")]
    pub history_file: Option<&'a str>,
    #[cfg(feature = "tmux_2_0")]
    pub message_limit: Option<usize>,
    #[cfg(feature = "tmux_1_5")]
    pub set_clipboard: Option<SetClipboard>,
    #[cfg(feature = "tmux_2_0")]
    pub terminal_overrides: Option<Vec<String>>,
    #[cfg(feature = "tmux_3_0")]
    pub user_keys: Option<Vec<String>>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub quiet: Option<Switch>,
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub detach_on_destroy: Option<Switch>,
    _phantom: &'a str,
}

impl<'a> ServerOptionsBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn backspace(&mut self, backspace: &'a str) -> &mut Self {
        self.backspace = Some(backspace);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit(&mut self, buffer_limit: usize) -> &mut Self {
        self.buffer_limit = Some(buffer_limit);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias(&mut self, command_alias: Vec<&str>) -> &mut Self {
        self.command_alias = Some(command_alias.iter().map(|s| (*s).to_string()).collect());
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal(&mut self, default_terminal: &'a str) -> &mut Self {
        self.default_terminal = Some(default_terminal);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time(&mut self, escape_time: usize) -> &mut Self {
        self.escape_time = Some(escape_time);
        self
    }

    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty(&mut self, exit_empty: Switch) -> &mut Self {
        self.exit_empty = Some(exit_empty);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached(&mut self, exit_unattached: Switch) -> &mut Self {
        self.exit_unattached = Some(exit_unattached);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events(&mut self, focus_events: Switch) -> &mut Self {
        self.focus_events = Some(focus_events);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn history_file(&mut self, history_file: &'a str) -> &mut Self {
        self.history_file = Some(history_file);
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit(&mut self, message_limit: usize) -> &mut Self {
        self.message_limit = Some(message_limit);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard(&mut self, set_clipboard: SetClipboard) -> &mut Self {
        self.set_clipboard = Some(set_clipboard);
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides(&mut self, terminal_overrides: Vec<&str>) -> &mut Self {
        self.terminal_overrides = Some(
            terminal_overrides
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
        );
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys(&mut self, user_keys: Vec<String>) -> &mut Self {
        self.user_keys = Some(user_keys);
        self
    }

    // XXX: clone rly needed?
    pub fn build(&self) -> ServerOptions {
        ServerOptions {
            #[cfg(feature = "tmux_3_1")]
            backspace: self.backspace.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_5")]
            buffer_limit: self.buffer_limit,
            #[cfg(feature = "tmux_2_4")]
            command_alias: self.command_alias.clone(),
            #[cfg(feature = "tmux_2_1")]
            default_terminal: self.default_terminal.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_2")]
            escape_time: self.escape_time,
            #[cfg(feature = "tmux_2_7")]
            exit_empty: self.exit_empty.clone(),
            #[cfg(feature = "tmux_1_4")]
            exit_unattached: self.exit_unattached.clone(),
            #[cfg(feature = "tmux_1_9")]
            focus_events: self.focus_events.clone(),
            #[cfg(feature = "tmux_2_1")]
            history_file: self.history_file.map(|s| s.to_string()),
            #[cfg(feature = "tmux_2_0")]
            message_limit: self.message_limit,
            #[cfg(feature = "tmux_1_5")]
            set_clipboard: self.set_clipboard.clone(),
            #[cfg(feature = "tmux_2_0")]
            terminal_overrides: self.terminal_overrides.clone(),
            #[cfg(feature = "tmux_3_0")]
            user_keys: self.user_keys.clone(),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            quiet: self.quiet.clone(),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            detach_on_destroy: self.quiet,
        }
    }
}
