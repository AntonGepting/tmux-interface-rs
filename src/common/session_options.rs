use crate::common::StatusKeys;
use crate::{Error, SetOptionBuilder, ShowOptionsBuilder, Switch, TargetPane, TmuxInterface};
use std::fmt;
use std::str::FromStr;

//visual-silence [on | off | both]
#[derive(PartialEq, Clone, Debug)]
pub enum Activity {
    On,
    Off,
    Both,
}

impl FromStr for Activity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "both" => Ok(Self::Both),
            _ => Err(Error::new("on/off/both parsing error")),
        }
    }
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            Self::Both => write!(f, "both"),
        }
    }
}

//activity-action [any | none | current | other]
//bell-action [any | none | current | other]
//silence-action [any | none | current | other]
#[derive(PartialEq, Clone, Debug)]
pub enum Action {
    Any,
    None,
    Current,
    Other,
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "any" => Ok(Self::Any),
            "none" => Ok(Self::None),
            "current" => Ok(Self::Current),
            "other" => Ok(Self::Other),
            _ => Err(Error::new("any/none/current/other parsing error")),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::None => write!(f, "none"),
            Self::Current => write!(f, "current"),
            Self::Other => write!(f, "other"),
        }
    }
}

//status [off | on | 2 | 3 | 4 | 5]
#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    On,
    Off,
    _2,
    _3,
    _4,
    _5,
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "2" => Ok(Self::_2),
            "3" => Ok(Self::_3),
            "4" => Ok(Self::_4),
            "5" => Ok(Self::_5),
            _ => Err(Error::new("on/off/2/3/4/5 parsing error")),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            Self::_2 => write!(f, "2"),
            Self::_3 => write!(f, "3"),
            Self::_4 => write!(f, "4"),
            Self::_5 => write!(f, "5"),
        }
    }
}

//status-justify [left | centre | right]
#[derive(PartialEq, Clone, Debug)]
pub enum StatusJustify {
    Left,
    Centre,
    Right,
}

impl FromStr for StatusJustify {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "left" => Ok(Self::Left),
            "centre" => Ok(Self::Centre),
            "right" => Ok(Self::Right),
            _ => Err(Error::new("left/centre/right parsing error")),
        }
    }
}

impl fmt::Display for StatusJustify {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Centre => write!(f, "centre"),
            Self::Right => write!(f, "right"),
        }
    }
}

//status-position [top | bottom]
#[derive(PartialEq, Clone, Debug)]
pub enum StatusPosition {
    Top,
    Bottom,
}

impl FromStr for StatusPosition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            _ => Err(Error::new("top/bottom parsing error")),
        }
    }
}

impl fmt::Display for StatusPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Top => write!(f, "top"),
            Self::Bottom => write!(f, "bottom"),
        }
    }
}

// NOTE: u32 not enough (x64 needed)!
pub const ACTIVITY_ACTION: usize = 1 << 0;
pub const ASSUME_PASTE_TIME: usize = 1 << 1;
pub const BASE_INDEX: usize = 1 << 2;
pub const BELL_ACTION: usize = 1 << 3;
pub const DEFAULT_COMMAND: usize = 1 << 4;
pub const DEFAULT_SHELL: usize = 1 << 5;
pub const DEFAULT_SIZE: usize = 1 << 6;
pub const DESTROY_UNATTACHED: usize = 1 << 7;
pub const DETACH_ON_DESTROY: usize = 1 << 8;
pub const DISPLAY_PANES_ACTIVE_COLOUR: usize = 1 << 9;
pub const DISPLAY_PANES_COLOUR: usize = 1 << 10;
pub const DISPLAY_PANES_TIME: usize = 1 << 11;
pub const DISPLAY_TIME: usize = 1 << 12;
pub const HISTORY_LIMIT: usize = 1 << 13;
pub const KEY_TABLE: usize = 1 << 14;
pub const LOCK_AFTER_TIME: usize = 1 << 15;
pub const LOCK_COMMAND: usize = 1 << 16;
pub const MESSAGE_COMMAND_STYLE: usize = 1 << 17;
pub const MESSAGE_STYLE: usize = 1 << 18;
pub const MOUSE: usize = 1 << 19;
pub const PREFIX: usize = 1 << 20;
pub const PREFIX2: usize = 1 << 21;
pub const RENUMBER_WINDOWS: usize = 1 << 22;
pub const REPEAT_TIME: usize = 1 << 23;
pub const SET_TITLES: usize = 1 << 24;
pub const SET_TITLES_STRING: usize = 1 << 25;
pub const SILENCE_ACTION: usize = 1 << 26;
pub const STATUS: usize = 1 << 27;
pub const STATUS_FORMAT: usize = 1 << 28;
pub const STATUS_INTERVAL: usize = 1 << 29;
pub const STATUS_JUSTIFY: usize = 1 << 30;
pub const STATUS_KEYS: usize = 1 << 31;
pub const STATUS_LEFT: usize = 1 << 32;
pub const STATUS_LEFT_LENGTH: usize = 1 << 33;
pub const STATUS_LEFT_STYLE: usize = 1 << 34;
pub const STATUS_POSITION: usize = 1 << 35;
pub const STATUS_RIGHT: usize = 1 << 36;
pub const STATUS_RIGHT_LENGTH: usize = 1 << 37;
pub const STATUS_RIGHT_STYLE: usize = 1 << 38;
pub const STATUS_STYLE: usize = 1 << 39;
pub const UPDATE_ENVIRONMENT: usize = 1 << 40;
pub const VISUAL_ACTIVITY: usize = 1 << 41;
pub const VISUAL_BELL: usize = 1 << 42;
pub const VISUAL_SILENCE: usize = 1 << 43;
pub const WORD_SEPARATORS: usize = 1 << 44;

pub const SESSION_OPTIONS_NONE: usize = 0;
//pub const SERVER_OPTIONS_DEFAULT: usize = ;
pub const SESSION_OPTIONS_ALL: usize = ACTIVITY_ACTION
    | ASSUME_PASTE_TIME
    | BASE_INDEX
    | BELL_ACTION
    | DEFAULT_COMMAND
    | DEFAULT_SHELL
    | DEFAULT_SIZE
    | DESTROY_UNATTACHED
    | DETACH_ON_DESTROY
    | DISPLAY_PANES_ACTIVE_COLOUR
    | DISPLAY_PANES_TIME
    | DISPLAY_TIME
    | HISTORY_LIMIT
    | KEY_TABLE
    | LOCK_AFTER_TIME
    | LOCK_COMMAND
    | MESSAGE_COMMAND_STYLE
    | MESSAGE_STYLE
    | MOUSE
    | PREFIX
    | PREFIX2
    | RENUMBER_WINDOWS
    | REPEAT_TIME
    | SET_TITLES
    | SET_TITLES_STRING
    | SILENCE_ACTION
    | STATUS
    | STATUS_FORMAT
    | STATUS_INTERVAL
    | STATUS_JUSTIFY
    | STATUS_KEYS
    | STATUS_LEFT
    | STATUS_LEFT_LENGTH
    | STATUS_LEFT_STYLE
    | STATUS_POSITION
    | STATUS_RIGHT
    | STATUS_RIGHT_LENGTH
    | STATUS_RIGHT_STYLE
    | STATUS_STYLE
    | UPDATE_ENVIRONMENT
    | VISUAL_ACTIVITY
    | VISUAL_BELL
    | VISUAL_SILENCE
    | WORD_SEPARATORS;

pub const SESSION_OPTIONS_NUM: usize = 45;
pub const SESSION_OPTIONS: [(
    &str,
    fn(o: &mut SessionOptions, s: &str),
    fn(o: &SessionOptions) -> Option<String>,
    usize,
); SESSION_OPTIONS_NUM] = [
    (
        "activity-action",
        |o, s| o.activity_action = s.parse().ok(),
        |o| o.activity_action.as_ref().map(|v| v.to_string()),
        ACTIVITY_ACTION,
    ),
    (
        "assume-paste-time",
        |o, s| o.assume_paste_time = s.parse().ok(),
        |o| o.assume_paste_time.as_ref().map(|v| v.to_string()),
        ASSUME_PASTE_TIME,
    ),
    (
        "base-index",
        |o, s| o.base_index = s.parse().ok(),
        |o| o.base_index.as_ref().map(|v| v.to_string()),
        BASE_INDEX,
    ),
    (
        "bell-action",
        |o, s| o.bell_action = s.parse().ok(),
        |o| o.bell_action.as_ref().map(|v| v.to_string()),
        BELL_ACTION,
    ),
    (
        "default-command",
        |o, s| o.default_command = s.parse().ok(),
        |o| o.default_command.as_ref().map(|v| v.to_string()),
        DEFAULT_COMMAND,
    ),
    (
        "default-shell",
        |o, s| o.default_shell = s.parse().ok(),
        |o| o.default_shell.as_ref().map(|v| v.to_string()),
        DEFAULT_SHELL,
    ),
    (
        "default-size",
        |o, _s| o.default_size = Some((0, 0)),
        |o| o.default_size.as_ref().map(|v| format!("{}x{}", v.0, v.1)),
        DEFAULT_SIZE,
    ),
    (
        "destroy-unattached",
        |o, s| o.destroy_unattached = s.parse().ok(),
        |o| o.destroy_unattached.as_ref().map(|v| v.to_string()),
        DESTROY_UNATTACHED,
    ),
    (
        "detach-on-destroy",
        |o, s| o.detach_on_destroy = s.parse().ok(),
        |o| o.detach_on_destroy.as_ref().map(|v| v.to_string()),
        DETACH_ON_DESTROY,
    ),
    (
        "display-panes-active-colour",
        |o, s| o.display_panes_active_colour = s.parse().ok(),
        |o| {
            o.display_panes_active_colour
                .as_ref()
                .map(|v| v.to_string())
        },
        DISPLAY_PANES_ACTIVE_COLOUR,
    ),
    (
        "display-panes-colour",
        |o, s| o.display_panes_colour = s.parse().ok(),
        |o| o.display_panes_colour.as_ref().map(|v| v.to_string()),
        DISPLAY_PANES_COLOUR,
    ),
    (
        "display-panes-time",
        |o, s| o.display_panes_time = s.parse().ok(),
        |o| o.display_panes_time.as_ref().map(|v| v.to_string()),
        DISPLAY_PANES_TIME,
    ),
    (
        "display-time",
        |o, s| o.display_time = s.parse().ok(),
        |o| o.display_time.as_ref().map(|v| v.to_string()),
        DISPLAY_TIME,
    ),
    (
        "history-limit",
        |o, s| o.history_limit = s.parse().ok(),
        |o| o.history_limit.as_ref().map(|v| v.to_string()),
        HISTORY_LIMIT,
    ),
    (
        "key-table",
        |o, s| o.key_table = s.parse().ok(),
        |o| o.key_table.as_ref().map(|v| v.to_string()),
        KEY_TABLE,
    ),
    (
        "lock-after-time",
        |o, s| o.lock_after_time = s.parse().ok(),
        |o| o.lock_after_time.as_ref().map(|v| v.to_string()),
        LOCK_AFTER_TIME,
    ),
    (
        "lock-command",
        |o, s| o.lock_command = s.parse().ok(),
        |o| o.lock_command.as_ref().map(|v| v.to_string()),
        LOCK_COMMAND,
    ),
    (
        "message-command-style",
        |o, s| o.message_command_style = s.parse().ok(),
        |o| o.message_command_style.as_ref().map(|v| v.to_string()),
        MESSAGE_COMMAND_STYLE,
    ),
    (
        "message-style",
        |o, s| o.message_style = s.parse().ok(),
        |o| o.message_style.as_ref().map(|v| v.to_string()),
        MESSAGE_STYLE,
    ),
    (
        "mouse",
        |o, s| o.mouse = s.parse().ok(),
        |o| o.mouse.as_ref().map(|v| v.to_string()),
        MOUSE,
    ),
    (
        "prefix",
        |o, s| o.prefix = s.parse().ok(),
        |o| o.prefix.as_ref().map(|v| v.to_string()),
        PREFIX,
    ),
    (
        "prefix2",
        |o, s| o.prefix2 = s.parse().ok(),
        |o| o.prefix2.as_ref().map(|v| v.to_string()),
        PREFIX2,
    ),
    (
        "renumber-windows",
        |o, s| o.renumber_windows = s.parse().ok(),
        |o| o.renumber_windows.as_ref().map(|v| v.to_string()),
        RENUMBER_WINDOWS,
    ),
    (
        "repeat-time",
        |o, s| o.repeat_time = s.parse().ok(),
        |o| o.repeat_time.as_ref().map(|v| v.to_string()),
        REPEAT_TIME,
    ),
    (
        "set-titles",
        |o, s| o.set_titles = s.parse().ok(),
        |o| o.set_titles.as_ref().map(|v| v.to_string()),
        SET_TITLES,
    ),
    (
        "set-titles-string",
        |o, s| o.set_titles_string = s.parse().ok(),
        |o| o.set_titles_string.as_ref().map(|v| v.to_string()),
        SET_TITLES_STRING,
    ),
    (
        "silence-action",
        |o, s| o.silence_action = s.parse().ok(),
        |o| o.silence_action.as_ref().map(|v| v.to_string()),
        SILENCE_ACTION,
    ),
    (
        "status",
        |o, s| o.status = s.parse().ok(),
        |o| o.status.as_ref().map(|v| v.to_string()),
        STATUS,
    ),
    (
        "status-format",
        |o, _s| o.status_format = None,
        |o| o.status_format.as_ref().map(|v| v.join(" ").to_string()),
        STATUS_FORMAT,
    ),
    (
        "status-interval",
        |o, s| o.status_interval = s.parse().ok(),
        |o| o.status_interval.as_ref().map(|v| v.to_string()),
        STATUS_INTERVAL,
    ),
    (
        "status-justify",
        |o, s| o.status_justify = s.parse().ok(),
        |o| o.status_justify.as_ref().map(|v| v.to_string()),
        STATUS_JUSTIFY,
    ),
    (
        "status-keys",
        |o, s| o.status_keys = s.parse().ok(),
        |o| o.status_keys.as_ref().map(|v| v.to_string()),
        STATUS_KEYS,
    ),
    (
        "status-left",
        |o, s| o.status_left = s.parse().ok(),
        |o| o.status_left.as_ref().map(|v| v.to_string()),
        STATUS_LEFT,
    ),
    (
        "status-left-length",
        |o, s| o.status_left_length = s.parse().ok(),
        |o| o.status_left_length.as_ref().map(|v| v.to_string()),
        STATUS_LEFT_LENGTH,
    ),
    (
        "status-left-style",
        |o, s| o.status_left_style = s.parse().ok(),
        |o| o.status_left_style.as_ref().map(|v| v.to_string()),
        STATUS_LEFT_STYLE,
    ),
    (
        "status-position",
        |o, s| o.status_position = s.parse().ok(),
        |o| o.status_position.as_ref().map(|v| v.to_string()),
        STATUS_POSITION,
    ),
    (
        "status-right",
        |o, s| o.status_right = s.parse().ok(),
        |o| o.status_right.as_ref().map(|v| v.to_string()),
        STATUS_RIGHT,
    ),
    (
        "status-right-length",
        |o, s| o.status_right_length = s.parse().ok(),
        |o| o.status_right_length.as_ref().map(|v| v.to_string()),
        STATUS_RIGHT_LENGTH,
    ),
    (
        "status-right-style",
        |o, s| o.status_right_style = s.parse().ok(),
        |o| o.status_right_style.as_ref().map(|v| v.to_string()),
        STATUS_RIGHT_STYLE,
    ),
    (
        "status-style",
        |o, s| o.status_style = s.parse().ok(),
        |o| o.status_style.as_ref().map(|v| v.to_string()),
        STATUS_STYLE,
    ),
    (
        "update-environment",
        |o, _s| o.update_environment = None,
        |o| {
            o.update_environment
                .as_ref()
                .map(|v| v.join(" ").to_string())
        },
        UPDATE_ENVIRONMENT,
    ),
    (
        "visual-activity",
        |o, s| o.visual_activity = s.parse().ok(),
        |o| o.visual_activity.as_ref().map(|v| v.to_string()),
        VISUAL_ACTIVITY,
    ),
    (
        "visual-bell",
        |o, s| o.visual_bell = s.parse().ok(),
        |o| o.visual_bell.as_ref().map(|v| v.to_string()),
        VISUAL_BELL,
    ),
    (
        "visual-silence",
        |o, s| o.visual_silence = s.parse().ok(),
        |o| o.visual_silence.as_ref().map(|v| v.to_string()),
        VISUAL_SILENCE,
    ),
    (
        "word-separators",
        |o, s| o.word_separators = s.parse().ok(),
        |o| o.word_separators.as_ref().map(|v| v.to_string()),
        WORD_SEPARATORS,
    ),
];

// TODO: Vec variables solution
// TODO: check types
// 45 Available session options are:
#[derive(Default, PartialEq, Clone, Debug)]
pub struct SessionOptions {
    //activity-action [any | none | current | other]
    pub activity_action: Option<Activity>,
    //assume-paste-time milliseconds
    pub assume_paste_time: Option<usize>,
    //base-index index
    pub base_index: Option<usize>,
    //bell-action [any | none | current | other]
    pub bell_action: Option<Action>,
    //default-command shell-command
    pub default_command: Option<String>,
    //default-shell path
    pub default_shell: Option<String>,
    //default-size XxY
    pub default_size: Option<(usize, usize)>,
    //destroy-unattached [on | off]
    pub destroy_unattached: Option<Switch>,
    //detach-on-destroy [on | off]
    pub detach_on_destroy: Option<Switch>,
    //display-panes-active-colour colour
    pub display_panes_active_colour: Option<String>,
    //display-panes-colour colour
    pub display_panes_colour: Option<String>,
    //display-panes-time time
    pub display_panes_time: Option<usize>,
    //display-time time
    pub display_time: Option<usize>,
    //history-limit lines
    pub history_limit: Option<usize>,
    //key-table key-table
    pub key_table: Option<String>,
    //lock-after-time number
    pub lock_after_time: Option<usize>,
    //lock-command shell-command
    pub lock_command: Option<String>,
    //message-command-style style
    pub message_command_style: Option<String>,
    //message-style style
    pub message_style: Option<String>,
    //mouse [on | off]
    pub mouse: Option<Switch>,
    //prefix key
    pub prefix: Option<String>,
    //prefix2 key
    pub prefix2: Option<String>,
    //renumber-windows [on | off]
    pub renumber_windows: Option<Switch>,
    //repeat-time time
    pub repeat_time: Option<usize>,
    //set-titles [on | off]
    pub set_titles: Option<Switch>,
    //set-titles-string string
    pub set_titles_string: Option<String>,
    //silence-action [any | none | current | other]
    pub silence_action: Option<Action>,
    //status [off | on | 2 | 3 | 4 | 5]
    pub status: Option<Status>,
    //status-format[] format
    pub status_format: Option<Vec<String>>,
    //status-interval interval
    pub status_interval: Option<usize>,
    //status-justify [left | centre | right]
    pub status_justify: Option<StatusJustify>,
    //status-keys [vi | emacs]
    pub status_keys: Option<StatusKeys>,
    //status-left string
    pub status_left: Option<String>,
    //status-left-length length
    pub status_left_length: Option<usize>,
    //status-left-style style
    pub status_left_style: Option<String>,
    //status-position [top | bottom]
    pub status_position: Option<StatusPosition>,
    //status-right string
    pub status_right: Option<String>,
    //status-right-length length
    pub status_right_length: Option<usize>,
    //status-right-style style
    pub status_right_style: Option<String>,
    //status-style style
    pub status_style: Option<String>,
    //update-environment[] variable
    pub update_environment: Option<Vec<String>>,
    //visual-activity [on | off | both]
    pub visual_activity: Option<Activity>,
    //visual-bell [on | off | both]
    pub visual_bell: Option<Activity>,
    //visual-silence [on | off | both]
    pub visual_silence: Option<Activity>,
    //word-separators string
    pub word_separators: Option<String>,
}

impl SessionOptions {
    pub fn get_all() -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let show_options = ShowOptionsBuilder::<TargetPane>::new()
            .global_options()
            .build();
        let s = tmux.show_options(Some(&show_options))?;
        s.parse()
    }

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    pub fn get(bitflags: usize) -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let selected_option = SESSION_OPTIONS
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

    // allows selective set by bitmask
    pub fn set(&self, bitflags: usize) -> Result<(), Error> {
        let mut tmux = TmuxInterface::new();
        for selected_option in SESSION_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
            if let Some(selected_value) = selected_option.2(&self) {
                let set_option = SetOptionBuilder::<TargetPane>::new().server().build();
                tmux.set_option(Some(&set_option), selected_option.0, &selected_value)?;
            }
        }
        Ok(())
    }

    // XXX: single set get methods
}

impl FromStr for SessionOptions {
    type Err = Error;

    fn from_str(options: &str) -> Result<Self, Self::Err> {
        let mut session_options: SessionOptions = Default::default();
        let mut v: Vec<&str>;
        for option in options.lines() {
            v = option.trim().split(' ').collect();
            for session_var in SESSION_OPTIONS.iter() {
                if session_var.0 == v[0] {
                    session_var.1(&mut session_options, v[1])
                }
            }
        }
        Ok(session_options)
    }
}

impl fmt::Display for SessionOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // pane option
        for var in SESSION_OPTIONS.iter() {
            // if is set some - extract
            if let Some(ref v) = var.2(self) {
                write!(f, "{} {}\n", var.0, v)?;
            }
        }
        write!(f, "{}", "")
    }
}

#[derive(Default, Debug)]
pub struct SessionOptionsBuilder<'a> {
    pub activity_action: Option<Activity>,
    pub assume_paste_time: Option<usize>,
    pub base_index: Option<usize>,
    pub bell_action: Option<Action>,
    pub default_command: Option<&'a str>,
    pub default_shell: Option<&'a str>,
    pub default_size: Option<(usize, usize)>,
    pub destroy_unattached: Option<Switch>,
    pub detach_on_destroy: Option<Switch>,
    pub display_panes_active_colour: Option<&'a str>,
    pub display_panes_colour: Option<&'a str>,
    pub display_panes_time: Option<usize>,
    pub display_time: Option<usize>,
    pub history_limit: Option<usize>,
    pub key_table: Option<&'a str>,
    pub lock_after_time: Option<usize>,
    pub lock_command: Option<&'a str>,
    pub message_command_style: Option<&'a str>,
    pub message_style: Option<&'a str>,
    pub mouse: Option<Switch>,
    pub prefix: Option<&'a str>,
    pub prefix2: Option<&'a str>,
    pub renumber_windows: Option<Switch>,
    pub repeat_time: Option<usize>,
    pub set_titles: Option<Switch>,
    pub set_titles_string: Option<&'a str>,
    pub silence_action: Option<Action>,
    pub status: Option<Status>,
    pub status_format: Option<Vec<&'a str>>,
    pub status_interval: Option<usize>,
    pub status_justify: Option<StatusJustify>,
    pub status_keys: Option<StatusKeys>,
    pub status_left: Option<&'a str>,
    pub status_left_length: Option<usize>,
    pub status_left_style: Option<&'a str>,
    pub status_position: Option<StatusPosition>,
    pub status_right: Option<&'a str>,
    pub status_right_length: Option<usize>,
    pub status_right_style: Option<&'a str>,
    pub status_style: Option<&'a str>,
    pub update_environment: Option<Vec<&'a str>>,
    pub visual_activity: Option<Activity>,
    pub visual_bell: Option<Activity>,
    pub visual_silence: Option<Activity>,
    pub word_separators: Option<&'a str>,
}

impl<'a> SessionOptionsBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn activity_action(&mut self, activity_action: Activity) -> &mut Self {
        self.activity_action = Some(activity_action);
        self
    }

    pub fn assume_paste_time(&mut self, assume_paste_time: usize) -> &mut Self {
        self.assume_paste_time = Some(assume_paste_time);
        self
    }

    pub fn base_index(&mut self, base_index: usize) -> &mut Self {
        self.base_index = Some(base_index);
        self
    }

    pub fn bell_action(&mut self, bell_action: Action) -> &mut Self {
        self.bell_action = Some(bell_action);
        self
    }

    pub fn default_command(&mut self, default_command: &'a str) -> &mut Self {
        self.default_command = Some(default_command);
        self
    }

    pub fn default_shell(&mut self, default_shell: &'a str) -> &mut Self {
        self.default_shell = Some(default_shell);
        self
    }

    pub fn default_size(&mut self, default_size: (usize, usize)) -> &mut Self {
        self.default_size = Some(default_size);
        self
    }

    pub fn destroy_unattached(&mut self, destroy_unattached: Switch) -> &mut Self {
        self.destroy_unattached = Some(destroy_unattached);
        self
    }

    pub fn detach_on_destroy(&mut self, detach_on_destroy: Switch) -> &mut Self {
        self.detach_on_destroy = Some(detach_on_destroy);
        self
    }

    pub fn display_panes_active_colour(
        &mut self,
        display_panes_active_colour: &'a str,
    ) -> &mut Self {
        self.display_panes_active_colour = Some(display_panes_active_colour);
        self
    }

    pub fn display_panes_colour(&mut self, display_panes_colour: &'a str) -> &mut Self {
        self.display_panes_colour = Some(display_panes_colour);
        self
    }

    pub fn display_panes_time(&mut self, display_panes_time: usize) -> &mut Self {
        self.display_panes_time = Some(display_panes_time);
        self
    }

    pub fn display_time(&mut self, display_time: usize) -> &mut Self {
        self.display_time = Some(display_time);
        self
    }

    pub fn history_limit(&mut self, history_limit: usize) -> &mut Self {
        self.history_limit = Some(history_limit);
        self
    }

    pub fn key_table(&mut self, key_table: &'a str) -> &mut Self {
        self.key_table = Some(key_table);
        self
    }

    pub fn lock_after_time(&mut self, lock_after_time: usize) -> &mut Self {
        self.lock_after_time = Some(lock_after_time);
        self
    }

    pub fn lock_command(&mut self, lock_command: &'a str) -> &mut Self {
        self.lock_command = Some(lock_command);
        self
    }

    pub fn message_command_style(&mut self, message_command_style: &'a str) -> &mut Self {
        self.message_command_style = Some(message_command_style);
        self
    }

    pub fn message_style(&mut self, message_style: &'a str) -> &mut Self {
        self.message_style = Some(message_style);
        self
    }

    pub fn mouse(&mut self, mouse: Switch) -> &mut Self {
        self.mouse = Some(mouse);
        self
    }

    pub fn prefix(&mut self, prefix: &'a str) -> &mut Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn prefix2(&mut self, prefix2: &'a str) -> &mut Self {
        self.prefix2 = Some(prefix2);
        self
    }

    pub fn renumber_windows(&mut self, renumber_windows: Switch) -> &mut Self {
        self.renumber_windows = Some(renumber_windows);
        self
    }

    pub fn repeat_time(&mut self, repeat_time: usize) -> &mut Self {
        self.repeat_time = Some(repeat_time);
        self
    }

    pub fn set_titles(&mut self, set_titles: Switch) -> &mut Self {
        self.set_titles = Some(set_titles);
        self
    }

    pub fn set_titles_string(&mut self, set_titles_string: &'a str) -> &mut Self {
        self.set_titles_string = Some(set_titles_string);
        self
    }

    pub fn silence_action(&mut self, silence_action: Action) -> &mut Self {
        self.silence_action = Some(silence_action);
        self
    }

    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    pub fn status_format(&mut self, status_format: Vec<&'a str>) -> &mut Self {
        self.status_format = Some(status_format);
        self
    }

    pub fn status_interval(&mut self, status_interval: usize) -> &mut Self {
        self.status_interval = Some(status_interval);
        self
    }

    pub fn status_justify(&mut self, status_justify: StatusJustify) -> &mut Self {
        self.status_justify = Some(status_justify);
        self
    }

    pub fn status_keys(&mut self, status_keys: StatusKeys) -> &mut Self {
        self.status_keys = Some(status_keys);
        self
    }

    pub fn status_left(&mut self, status_left: &'a str) -> &mut Self {
        self.status_left = Some(status_left);
        self
    }

    pub fn status_left_length(&mut self, status_left_length: usize) -> &mut Self {
        self.status_left_length = Some(status_left_length);
        self
    }

    pub fn status_left_style(&mut self, status_left_style: &'a str) -> &mut Self {
        self.status_left_style = Some(status_left_style);
        self
    }

    pub fn status_position(&mut self, status_position: StatusPosition) -> &mut Self {
        self.status_position = Some(status_position);
        self
    }

    pub fn status_right(&mut self, status_right: &'a str) -> &mut Self {
        self.status_right = Some(status_right);
        self
    }

    pub fn status_right_length(&mut self, status_right_length: usize) -> &mut Self {
        self.status_right_length = Some(status_right_length);
        self
    }

    pub fn status_right_style(&mut self, status_right_style: &'a str) -> &mut Self {
        self.status_right_style = Some(status_right_style);
        self
    }

    pub fn status_style(&mut self, status_style: &'a str) -> &mut Self {
        self.status_style = Some(status_style);
        self
    }

    pub fn update_environment(&mut self, update_environment: Vec<&'a str>) -> &mut Self {
        self.update_environment = Some(update_environment);
        self
    }

    pub fn visual_activity(&mut self, visual_activity: Activity) -> &mut Self {
        self.visual_activity = Some(visual_activity);
        self
    }

    pub fn visual_bell(&mut self, visual_bell: Activity) -> &mut Self {
        self.visual_bell = Some(visual_bell);
        self
    }

    pub fn visual_silence(&mut self, visual_silence: Activity) -> &mut Self {
        self.visual_silence = Some(visual_silence);
        self
    }

    pub fn word_separators(&mut self, word_separators: &'a str) -> &mut Self {
        self.word_separators = Some(word_separators);
        self
    }

    // XXX: clone rly needed?
    pub fn build(&self) -> SessionOptions {
        SessionOptions {
            activity_action: self.activity_action.clone(),
            assume_paste_time: self.assume_paste_time,
            base_index: self.base_index,
            bell_action: self.bell_action.clone(),
            default_command: self.default_command.map(|s| s.to_string()),
            default_shell: self.default_shell.map(|s| s.to_string()),
            default_size: self.default_size,
            destroy_unattached: self.destroy_unattached.clone(),
            detach_on_destroy: self.detach_on_destroy.clone(),
            display_panes_active_colour: self.display_panes_active_colour.map(|s| s.to_string()),
            display_panes_colour: self.display_panes_colour.map(|s| s.to_string()),
            display_panes_time: self.display_panes_time,
            display_time: self.display_time,
            history_limit: self.history_limit,
            key_table: self.key_table.map(|s| s.to_string()),
            lock_after_time: self.lock_after_time,
            lock_command: self.lock_command.map(|s| s.to_string()),
            message_command_style: self.message_command_style.map(|s| s.to_string()),
            message_style: self.message_style.map(|s| s.to_string()),
            mouse: self.mouse.clone(),
            prefix: self.prefix.map(|s| s.to_string()),
            prefix2: self.prefix2.map(|s| s.to_string()),
            renumber_windows: self.renumber_windows.clone(),
            repeat_time: self.repeat_time,
            set_titles: self.set_titles.clone(),
            set_titles_string: self.set_titles_string.map(|s| s.to_string()),
            silence_action: self.silence_action.clone(),
            status: self.status.clone(),
            status_format: None,
            status_interval: self.status_interval,
            status_justify: self.status_justify.clone(),
            status_keys: self.status_keys.clone(),
            status_left: self.status_left.map(|s| s.to_string()),
            status_left_length: self.status_left_length,
            status_left_style: self.status_left_style.map(|s| s.to_string()),
            status_position: self.status_position.clone(),
            status_right: self.status_right.map(|s| s.to_string()),
            status_right_length: self.status_right_length,
            status_right_style: self.status_right_style.map(|s| s.to_string()),
            status_style: self.status_style.map(|s| s.to_string()),
            update_environment: None,
            visual_activity: self.visual_activity.clone(),
            visual_bell: self.visual_bell.clone(),
            visual_silence: self.visual_silence.clone(),
            word_separators: self.word_separators.map(|s| s.to_string()),
        }
    }
}
