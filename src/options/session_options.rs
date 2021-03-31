use super::create_insert_vec;
use crate::options::StatusKeys;
use crate::{Error, SetOption, ShowOptions, Switch};
use std::fmt;
use std::str::FromStr;

//visual-silence [on | off | both]
#[cfg(feature = "tmux_0_8")]
#[derive(PartialEq, Clone, Debug)]
pub enum Activity {
    On,
    Off,
    #[cfg(feature = "tmux_2_6")]
    Both,
}

#[cfg(feature = "tmux_0_8")]
impl FromStr for Activity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            #[cfg(feature = "tmux_2_6")]
            "both" => Ok(Self::Both),
            _ => Err(Error::ParseActivity),
        }
    }
}

#[cfg(feature = "tmux_0_8")]
impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            #[cfg(feature = "tmux_2_6")]
            Self::Both => write!(f, "both"),
        }
    }
}

//activity-action [any | none | current | other]
//bell-action [any | none | current | other]
//silence-action [any | none | current | other]
#[cfg(feature = "tmux_0_8")]
#[derive(PartialEq, Clone, Debug)]
pub enum Action {
    Any,
    None,
    Current,
    #[cfg(feature = "tmux_2_1")]
    Other,
}

#[cfg(feature = "tmux_0_8")]
impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "any" => Ok(Self::Any),
            "none" => Ok(Self::None),
            "current" => Ok(Self::Current),
            #[cfg(feature = "tmux_2_1")]
            "other" => Ok(Self::Other),
            _ => Err(Error::ParseAction),
        }
    }
}

#[cfg(feature = "tmux_0_8")]
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::None => write!(f, "none"),
            Self::Current => write!(f, "current"),
            #[cfg(feature = "tmux_2_1")]
            Self::Other => write!(f, "other"),
        }
    }
}

//status [off | on | 2 | 3 | 4 | 5]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_0_8")]
pub enum Status {
    On,
    Off,
    #[cfg(feature = "tmux_2_9")]
    _2,
    #[cfg(feature = "tmux_2_9")]
    _3,
    #[cfg(feature = "tmux_2_9")]
    _4,
    #[cfg(feature = "tmux_2_9")]
    _5,
}

#[cfg(feature = "tmux_0_8")]
impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            #[cfg(feature = "tmux_2_9")]
            "2" => Ok(Self::_2),
            #[cfg(feature = "tmux_2_9")]
            "3" => Ok(Self::_3),
            #[cfg(feature = "tmux_2_9")]
            "4" => Ok(Self::_4),
            #[cfg(feature = "tmux_2_9")]
            "5" => Ok(Self::_5),
            _ => Err(Error::ParseStatus),
        }
    }
}

#[cfg(feature = "tmux_0_8")]
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            #[cfg(feature = "tmux_2_9")]
            Self::_2 => write!(f, "2"),
            #[cfg(feature = "tmux_2_9")]
            Self::_3 => write!(f, "3"),
            #[cfg(feature = "tmux_2_9")]
            Self::_4 => write!(f, "4"),
            #[cfg(feature = "tmux_2_9")]
            Self::_5 => write!(f, "5"),
        }
    }
}

//status-justify [left | centre | right]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_1_0")]
pub enum StatusJustify {
    Left,
    Centre,
    Right,
}

#[cfg(feature = "tmux_1_0")]
impl FromStr for StatusJustify {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "left" => Ok(Self::Left),
            "centre" => Ok(Self::Centre),
            "right" => Ok(Self::Right),
            _ => Err(Error::ParseStatusJustify),
        }
    }
}

#[cfg(feature = "tmux_1_0")]
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
#[cfg(feature = "tmux_1_7")]
pub enum StatusPosition {
    Top,
    Bottom,
}

#[cfg(feature = "tmux_1_7")]
impl FromStr for StatusPosition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            _ => Err(Error::ParseStatusPosition),
        }
    }
}

#[cfg(feature = "tmux_1_7")]
impl fmt::Display for StatusPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Top => write!(f, "top"),
            Self::Bottom => write!(f, "bottom"),
        }
    }
}

// XXX: conditionals?
// NOTE: total num: 81 (u128)
pub const ACTIVITY_ACTION: u128 = 1;
pub const ASSUME_PASTE_TIME: u128 = 1 << 1;
pub const BASE_INDEX: u128 = 1 << 2;
pub const BELL_ACTION: u128 = 1 << 3;
pub const BELL_ON_ALERT: u128 = 1 << 4;
pub const BUFFER_LIMIT: u128 = 1 << 5;
pub const DEFAULT_COMMAND: u128 = 1 << 6;
pub const DEFAULT_SHELL: u128 = 1 << 7;
pub const DEFAULT_PATH: u128 = 1 << 8;
pub const DEFAULT_TERMINAL: u128 = 1 << 9;
pub const DEFAULT_SIZE: u128 = 1 << 10;
pub const DESTROY_UNATTACHED: u128 = 1 << 11;
pub const DETACH_ON_DESTROY: u128 = 1 << 12;
pub const DISPLAY_PANES_ACTIVE_COLOUR: u128 = 1 << 13;
pub const DISPLAY_PANES_COLOUR: u128 = 1 << 14;
pub const DISPLAY_PANES_TIME: u128 = 1 << 15;
pub const DISPLAY_TIME: u128 = 1 << 16;
pub const HISTORY_LIMIT: u128 = 1 << 17;
pub const KEY_TABLE: u128 = 1 << 18;
pub const LOCK_AFTER_TIME: u128 = 1 << 19;
pub const LOCK_COMMAND: u128 = 1 << 20;
pub const LOCK_SERVER: u128 = 1 << 21;
pub const MESSAGE_ATTR: u128 = 1 << 22;
pub const MESSAGE_BG: u128 = 1 << 23;
pub const MESSAGE_COMMAND_ATTR: u128 = 1 << 24;
pub const MESSAGE_COMMAND_BG: u128 = 1 << 25;
pub const MESSAGE_COMMAND_FG: u128 = 1 << 26;
pub const MESSAGE_FG: u128 = 1 << 27;
pub const MESSAGE_COMMAND_STYLE: u128 = 1 << 28;
pub const MESSAGE_LIMIT: u128 = 1 << 29;
pub const MESSAGE_STYLE: u128 = 1 << 30;
pub const MOUSE_RESIZE_PANE: u128 = 1 << 31;
pub const MOUSE_SELECT_PANE: u128 = 1 << 32;
pub const MOUSE_SELECT_WINDOW: u128 = 1 << 33;
pub const MOUSE: u128 = 1 << 34;
pub const MOUSE_UTF8: u128 = 1 << 35;
pub const PANE_ACTIVE_BORDER_BG: u128 = 1 << 36;
pub const PANE_ACTIVE_BORDER_FG: u128 = 1 << 37;
pub const PANE_BORDER_BG: u128 = 1 << 38;
pub const PANE_BORDER_FG: u128 = 1 << 39;
pub const PANE_ACTIVE_BORDER_STYLE: u128 = 1 << 40;
pub const PANE_BORDER_STYLE: u128 = 1 << 41;
pub const PREFIX: u128 = 1 << 42;
pub const PREFIX2: u128 = 1 << 43;
pub const RENUMBER_WINDOWS: u128 = 1 << 44;
pub const REPEAT_TIME: u128 = 1 << 45;
pub const SET_REMAIN_ON_EXIT: u128 = 1 << 46;
pub const SET_TITLES: u128 = 1 << 47;
pub const SET_TITLES_STRING: u128 = 1 << 48;
pub const SILENCE_ACTION: u128 = 1 << 49;
pub const STATUS: u128 = 1 << 50;
pub const STATUS_ATTR: u128 = 1 << 51;
pub const STATUS_BG: u128 = 1 << 52;
pub const STATUS_FG: u128 = 1 << 53;
pub const STATUS_FORMAT: u128 = 1 << 54;
pub const STATUS_INTERVAL: u128 = 1 << 55;
pub const STATUS_JUSTIFY: u128 = 1 << 56;
pub const STATUS_KEYS: u128 = 1 << 57;
pub const STATUS_LEFT: u128 = 1 << 58;
pub const STATUS_LEFT_ATTR: u128 = 1 << 59;
pub const STATUS_LEFT_BG: u128 = 1 << 60;
pub const STATUS_LEFT_FG: u128 = 1 << 61;
pub const STATUS_LEFT_LENGTH: u128 = 1 << 62;
pub const STATUS_LEFT_STYLE: u128 = 1 << 63;
pub const STATUS_POSITION: u128 = 1 << 64;
pub const STATUS_RIGHT: u128 = 1 << 65;
pub const STATUS_RIGHT_ATTR: u128 = 1 << 66;
pub const STATUS_RIGHT_BG: u128 = 1 << 67;
pub const STATUS_RIGHT_FG: u128 = 1 << 68;
pub const STATUS_RIGHT_LENGTH: u128 = 1 << 69;
pub const STATUS_RIGHT_STYLE: u128 = 1 << 70;
pub const STATUS_STYLE: u128 = 1 << 71;
pub const STATUS_UTF8: u128 = 1 << 72;
pub const TERMINAL_OVERRIDES: u128 = 1 << 73;
pub const UPDATE_ENVIRONMENT: u128 = 1 << 74;
pub const USER_KEYS: u128 = 1 << 75;
pub const VISUAL_ACTIVITY: u128 = 1 << 76;
pub const VISUAL_BELL: u128 = 1 << 77;
pub const VISUAL_CONTENT: u128 = 1 << 78;
pub const VISUAL_SILENCE: u128 = 1 << 79;
pub const WORD_SEPARATORS: u128 = 1 << 80;
// XXX: add later?
//pub const USER_OPTIONS: u128 = 1 << 65;

pub const SESSION_OPTIONS_NONE: u128 = 0;
//pub const SERVER_OPTIONS_DEFAULT: u128 = ;

// XXX: conditionals?
pub const SESSION_OPTIONS_ALL: u128 = ACTIVITY_ACTION
    | ASSUME_PASTE_TIME
    | BASE_INDEX
    | BELL_ACTION
    | BELL_ON_ALERT
    | BUFFER_LIMIT
    | DEFAULT_COMMAND
    | DEFAULT_SHELL
    | DEFAULT_PATH
    | DEFAULT_TERMINAL
    | DEFAULT_SIZE
    | DESTROY_UNATTACHED
    | DETACH_ON_DESTROY
    | DISPLAY_PANES_ACTIVE_COLOUR
    | DISPLAY_PANES_COLOUR
    | DISPLAY_PANES_TIME
    | DISPLAY_TIME
    | HISTORY_LIMIT
    | KEY_TABLE
    | LOCK_AFTER_TIME
    | LOCK_COMMAND
    | LOCK_SERVER
    | MESSAGE_ATTR
    | MESSAGE_BG
    | MESSAGE_COMMAND_ATTR
    | MESSAGE_COMMAND_BG
    | MESSAGE_COMMAND_FG
    | MESSAGE_FG
    | MESSAGE_COMMAND_STYLE
    | MESSAGE_LIMIT
    | MESSAGE_STYLE
    | MOUSE_RESIZE_PANE
    | MOUSE_SELECT_PANE
    | MOUSE_SELECT_WINDOW
    | MOUSE
    | MOUSE_UTF8
    | PANE_ACTIVE_BORDER_BG
    | PANE_ACTIVE_BORDER_FG
    | PANE_BORDER_BG
    | PANE_BORDER_FG
    | PANE_ACTIVE_BORDER_STYLE
    | PANE_BORDER_STYLE
    | PREFIX
    | PREFIX2
    | RENUMBER_WINDOWS
    | REPEAT_TIME
    | SET_REMAIN_ON_EXIT
    | SET_TITLES
    | SET_TITLES_STRING
    | SILENCE_ACTION
    | STATUS
    | STATUS_ATTR
    | STATUS_BG
    | STATUS_FG
    | STATUS_FORMAT
    | STATUS_INTERVAL
    | STATUS_JUSTIFY
    | STATUS_KEYS
    | STATUS_LEFT
    | STATUS_LEFT_ATTR
    | STATUS_LEFT_BG
    | STATUS_LEFT_FG
    | STATUS_LEFT_LENGTH
    | STATUS_LEFT_STYLE
    | STATUS_POSITION
    | STATUS_RIGHT
    | STATUS_RIGHT_ATTR
    | STATUS_RIGHT_BG
    | STATUS_RIGHT_FG
    | STATUS_RIGHT_LENGTH
    | STATUS_RIGHT_STYLE
    | STATUS_STYLE
    | STATUS_UTF8
    | TERMINAL_OVERRIDES
    | UPDATE_ENVIRONMENT
    | USER_KEYS
    | VISUAL_ACTIVITY
    | VISUAL_BELL
    | VISUAL_CONTENT
    | VISUAL_SILENCE
    | WORD_SEPARATORS;
//| USER_OPTIONS;

#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
pub const SESSION_OPTIONS_NUM: usize = 0;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_1")))]
pub const SESSION_OPTIONS_NUM: usize = 43;
#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
pub const SESSION_OPTIONS_NUM: usize = 46;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_3")))]
pub const SESSION_OPTIONS_NUM: usize = 52;
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
pub const SESSION_OPTIONS_NUM: usize = 52;
#[cfg(all(feature = "tmux_1_4", not(feature = "tmux_1_5")))]
pub const SESSION_OPTIONS_NUM: usize = 55;
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_1_6")))]
pub const SESSION_OPTIONS_NUM: usize = 58;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_7")))]
pub const SESSION_OPTIONS_NUM: usize = 63;
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
pub const SESSION_OPTIONS_NUM: usize = 65;
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const SESSION_OPTIONS_NUM: usize = 66;
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_1_9a")))]
pub const SESSION_OPTIONS_NUM: usize = 53;
#[cfg(all(feature = "tmux_1_9a", not(feature = "tmux_2_0")))]
pub const SESSION_OPTIONS_NUM: usize = 53;
#[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_1")))]
pub const SESSION_OPTIONS_NUM: usize = 48;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
pub const SESSION_OPTIONS_NUM: usize = 44;
#[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_3")))]
pub const SESSION_OPTIONS_NUM: usize = 43;
#[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
pub const SESSION_OPTIONS_NUM: usize = 43;
#[cfg(all(feature = "tmux_2_4", not(feature = "tmux_2_5")))]
pub const SESSION_OPTIONS_NUM: usize = 42;
#[cfg(all(feature = "tmux_2_5", not(feature = "tmux_2_6")))]
pub const SESSION_OPTIONS_NUM: usize = 42;
#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_2_7")))]
pub const SESSION_OPTIONS_NUM: usize = 44;
#[cfg(all(feature = "tmux_2_7", not(feature = "tmux_2_8")))]
pub const SESSION_OPTIONS_NUM: usize = 44;
#[cfg(all(feature = "tmux_2_8", not(feature = "tmux_2_9")))]
pub const SESSION_OPTIONS_NUM: usize = 44;
#[cfg(all(feature = "tmux_2_9", not(feature = "tmux_2_9a")))]
pub const SESSION_OPTIONS_NUM: usize = 46; // NOTE: 47 with double default-size entry in man?
#[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_0")))]
pub const SESSION_OPTIONS_NUM: usize = 46; // NOTE: 47 with double default-size entry in man?
#[cfg(all(feature = "tmux_3_0", not(feature = "tmux_3_0a")))]
pub const SESSION_OPTIONS_NUM: usize = 45;
#[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_1")))]
pub const SESSION_OPTIONS_NUM: usize = 45;
#[cfg(all(feature = "tmux_3_1", not(feature = "tmux_3_1a")))]
pub const SESSION_OPTIONS_NUM: usize = 45;
#[cfg(all(feature = "tmux_3_1a", not(feature = "tmux_3_1b")))]
pub const SESSION_OPTIONS_NUM: usize = 45;
#[cfg(all(feature = "tmux_3_1b", not(feature = "tmux_X_X")))]
pub const SESSION_OPTIONS_NUM: usize = 45;
#[cfg(feature = "tmux_X_X")]
pub const SESSION_OPTIONS_NUM: usize = 45;

// TODO: waiting for const generics stabilization https://github.com/rust-lang/rust/issues/44580
pub const SESSION_OPTIONS: [(
    &str,
    fn(o: &mut SessionOptions, i: Option<usize>, s: &str),
    fn(o: &SessionOptions) -> Option<String>,
    u128,
); SESSION_OPTIONS_NUM] = [
    #[cfg(feature = "tmux_2_6")]
    (
        "activity-action",
        |o, _, s| o.activity_action = s.parse().ok(),
        |o| o.activity_action.as_ref().map(|v| v.to_string()),
        ACTIVITY_ACTION,
    ),
    #[cfg(feature = "tmux_1_8")]
    (
        "assume-paste-time",
        |o, _, s| o.assume_paste_time = s.parse().ok(),
        |o| o.assume_paste_time.as_ref().map(|v| v.to_string()),
        ASSUME_PASTE_TIME,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "base-index",
        |o, _, s| o.base_index = s.parse().ok(),
        |o| o.base_index.as_ref().map(|v| v.to_string()),
        BASE_INDEX,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "bell-action",
        |o, _, s| o.bell_action = s.parse().ok(),
        |o| o.bell_action.as_ref().map(|v| v.to_string()),
        BELL_ACTION,
    ),
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    (
        "bell-on-alert",
        |o, _, s| o.bell_on_alert = s.parse().ok(),
        |o| o.bell_on_alert.as_ref().map(|v| v.to_string()),
        BELL_ON_ALERT,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    (
        "buffer-limit",
        |o, _, s| o.buffer_limit = s.parse().ok(),
        |o| o.buffer_limit.as_ref().map(|v| v.to_string()),
        BUFFER_LIMIT,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "default-command",
        |o, _, s| o.default_command = s.parse().ok(),
        |o| o.default_command.as_ref().map(|v| v.to_string()),
        DEFAULT_COMMAND,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "default-shell",
        |o, _, s| o.default_shell = s.parse().ok(),
        |o| o.default_shell.as_ref().map(|v| v.to_string()),
        DEFAULT_SHELL,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "default-path",
        |o, _, s| o.default_path = s.parse().ok(),
        |o| o.default_path.as_ref().map(|v| v.to_string()),
        DEFAULT_PATH,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    (
        "default-terminal",
        |o, _, s| o.default_terminal = s.parse().ok(),
        |o| o.default_terminal.as_ref().map(|v| v.to_string()),
        DEFAULT_TERMINAL,
    ),
    #[cfg(feature = "tmux_2_9")]
    (
        "default-size",
        |o, _, _s| o.default_size = Some((0, 0)),
        |o| o.default_size.as_ref().map(|v| format!("{}x{}", v.0, v.1)),
        DEFAULT_SIZE,
    ),
    #[cfg(feature = "tmux_1_4")]
    (
        "destroy-unattached",
        |o, _, s| o.destroy_unattached = s.parse().ok(),
        |o| o.destroy_unattached.as_ref().map(|v| v.to_string()),
        DESTROY_UNATTACHED,
    ),
    #[cfg(feature = "tmux_1_4")]
    (
        "detach-on-destroy",
        |o, _, s| o.detach_on_destroy = s.parse().ok(),
        |o| o.detach_on_destroy.as_ref().map(|v| v.to_string()),
        DETACH_ON_DESTROY,
    ),
    #[cfg(feature = "tmux_1_2")]
    (
        "display-panes-active-colour",
        |o, _, s| o.display_panes_active_colour = s.parse().ok(),
        |o| {
            o.display_panes_active_colour
                .as_ref()
                .map(|v| v.to_string())
        },
        DISPLAY_PANES_ACTIVE_COLOUR,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "display-panes-colour",
        |o, _, s| o.display_panes_colour = s.parse().ok(),
        |o| o.display_panes_colour.as_ref().map(|v| v.to_string()),
        DISPLAY_PANES_COLOUR,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "display-panes-time",
        |o, _, s| o.display_panes_time = s.parse().ok(),
        |o| o.display_panes_time.as_ref().map(|v| v.to_string()),
        DISPLAY_PANES_TIME,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "display-time",
        |o, _, s| o.display_time = s.parse().ok(),
        |o| o.display_time.as_ref().map(|v| v.to_string()),
        DISPLAY_TIME,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "history-limit",
        |o, _, s| o.history_limit = s.parse().ok(),
        |o| o.history_limit.as_ref().map(|v| v.to_string()),
        HISTORY_LIMIT,
    ),
    #[cfg(feature = "tmux_2_2")]
    (
        "key-table",
        |o, _, s| o.key_table = s.parse().ok(),
        |o| o.key_table.as_ref().map(|v| v.to_string()),
        KEY_TABLE,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "lock-after-time",
        |o, _, s| o.lock_after_time = s.parse().ok(),
        |o| o.lock_after_time.as_ref().map(|v| v.to_string()),
        LOCK_AFTER_TIME,
    ),
    #[cfg(feature = "tmux_1_1")]
    (
        "lock-command",
        |o, _, s| o.lock_command = s.parse().ok(),
        |o| o.lock_command.as_ref().map(|v| v.to_string()),
        LOCK_COMMAND,
    ),
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    (
        "lock-server",
        |o, _, s| o.lock_server = s.parse().ok(),
        |o| o.lock_server.as_ref().map(|v| v.to_string()),
        LOCK_SERVER,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "message-attr",
        |o, _, s| o.message_attr = s.parse().ok(),
        |o| o.message_attr.as_ref().map(|v| v.to_string()),
        MESSAGE_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "message-bg",
        |o, _, s| o.message_bg = s.parse().ok(),
        |o| o.message_bg.as_ref().map(|v| v.to_string()),
        MESSAGE_BG,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "message-command-attr",
        |o, _, s| o.message_command_attr = s.parse().ok(),
        |o| o.message_command_attr.as_ref().map(|v| v.to_string()),
        MESSAGE_COMMAND_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "message-command-bg",
        |o, _, s| o.message_command_bg = s.parse().ok(),
        |o| o.message_command_bg.as_ref().map(|v| v.to_string()),
        MESSAGE_COMMAND_BG,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "message-command-fg",
        |o, _, s| o.message_command_fg = s.parse().ok(),
        |o| o.message_command_fg.as_ref().map(|v| v.to_string()),
        MESSAGE_COMMAND_FG,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "message-fg",
        |o, _, s| o.message_fg = s.parse().ok(),
        |o| o.message_fg.as_ref().map(|v| v.to_string()),
        MESSAGE_FG,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "message-command-style",
        |o, _, s| o.message_command_style = s.parse().ok(),
        |o| o.message_command_style.as_ref().map(|v| v.to_string()),
        MESSAGE_COMMAND_STYLE,
    ),
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    (
        "message-limit",
        |o, _, s| o.message_limit = s.parse().ok(),
        |o| o.message_limit.as_ref().map(|v| v.to_string()),
        MESSAGE_LIMIT,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "message-style",
        |o, _, s| o.message_style = s.parse().ok(),
        |o| o.message_style.as_ref().map(|v| v.to_string()),
        MESSAGE_STYLE,
    ),
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    (
        "mouse-resize-pane",
        |o, _, s| o.mouse_resize_pane = s.parse().ok(),
        |o| o.mouse_resize_pane.as_ref().map(|v| v.to_string()),
        MOUSE,
    ),
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    (
        "mouse-select-pane",
        |o, _, s| o.mouse_select_pane = s.parse().ok(),
        |o| o.mouse_select_pane.as_ref().map(|v| v.to_string()),
        MOUSE,
    ),
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    (
        "mouse-select-window",
        |o, _, s| o.mouse_select_window = s.parse().ok(),
        |o| o.mouse_select_window.as_ref().map(|v| v.to_string()),
        MOUSE,
    ),
    #[cfg(feature = "tmux_2_1")]
    (
        "mouse",
        |o, _, s| o.mouse = s.parse().ok(),
        |o| o.mouse.as_ref().map(|v| v.to_string()),
        MOUSE,
    ),
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    (
        "mouse-utf8",
        |o, _, s| o.mouse_utf8 = s.parse().ok(),
        |o| o.mouse_utf8.as_ref().map(|v| v.to_string()),
        MOUSE,
    ),
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    (
        "pane-active-border-bg",
        |o, _, s| o.pane_active_border_bg = s.parse().ok(),
        |o| o.pane_active_border_bg.as_ref().map(|v| v.to_string()),
        PANE_ACTIVE_BORDER_BG,
    ),
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    (
        "pane-active-border-fg",
        |o, _, s| o.pane_active_border_fg = s.parse().ok(),
        |o| o.pane_active_border_fg.as_ref().map(|v| v.to_string()),
        PANE_ACTIVE_BORDER_FG,
    ),
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    (
        "pane-border-bg",
        |o, _, s| o.pane_border_bg = s.parse().ok(),
        |o| o.pane_border_bg.as_ref().map(|v| v.to_string()),
        PANE_BORDER_BG,
    ),
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    (
        "pane-border-fg",
        |o, _, s| o.pane_border_fg = s.parse().ok(),
        |o| o.pane_border_fg.as_ref().map(|v| v.to_string()),
        PANE_BORDER_FG,
    ),
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    (
        "pane-active-border-style",
        |o, _, s| o.pane_active_border_style = s.parse().ok(),
        |o| o.pane_active_border_style.as_ref().map(|v| v.to_string()),
        PANE_ACTIVE_BORDER_STYLE,
    ),
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    (
        "pane-border-style",
        |o, _, s| o.pane_border_style = s.parse().ok(),
        |o| o.pane_border_style.as_ref().map(|v| v.to_string()),
        PANE_BORDER_STYLE,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "prefix",
        |o, _, s| o.prefix = s.parse().ok(),
        |o| o.prefix.as_ref().map(|v| v.to_string()),
        PREFIX,
    ),
    #[cfg(feature = "tmux_1_6")]
    (
        "prefix2",
        |o, _, s| o.prefix2 = s.parse().ok(),
        |o| o.prefix2.as_ref().map(|v| v.to_string()),
        PREFIX2,
    ),
    #[cfg(feature = "tmux_1_7")]
    (
        "renumber-windows",
        |o, _, s| o.renumber_windows = s.parse().ok(),
        |o| o.renumber_windows.as_ref().map(|v| v.to_string()),
        RENUMBER_WINDOWS,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "repeat-time",
        |o, _, s| o.repeat_time = s.parse().ok(),
        |o| o.repeat_time.as_ref().map(|v| v.to_string()),
        REPEAT_TIME,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    (
        "set-remain-on-exit",
        |o, _, s| o.set_remain_on_exit = s.parse().ok(),
        |o| o.set_remain_on_exit.as_ref().map(|v| v.to_string()),
        SET_REMAIN_ON_EXIT,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "set-titles",
        |o, _, s| o.set_titles = s.parse().ok(),
        |o| o.set_titles.as_ref().map(|v| v.to_string()),
        SET_TITLES,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "set-titles-string",
        |o, _, s| o.set_titles_string = s.parse().ok(),
        |o| o.set_titles_string.as_ref().map(|v| v.to_string()),
        SET_TITLES_STRING,
    ),
    #[cfg(feature = "tmux_2_6")]
    (
        "silence-action",
        |o, _, s| o.silence_action = s.parse().ok(),
        |o| o.silence_action.as_ref().map(|v| v.to_string()),
        SILENCE_ACTION,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "status",
        |o, _, s| o.status = s.parse().ok(),
        |o| o.status.as_ref().map(|v| v.to_string()),
        STATUS,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "status-attr",
        |o, _, s| o.status_attr = s.parse().ok(),
        |o| o.status_attr.as_ref().map(|v| v.to_string()),
        STATUS_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "status-bg",
        |o, _, s| o.status_bg = s.parse().ok(),
        |o| o.status_bg.as_ref().map(|v| v.to_string()),
        STATUS_BG,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "status-fg",
        |o, _, s| o.status_fg = s.parse().ok(),
        |o| o.status_fg.as_ref().map(|v| v.to_string()),
        STATUS_FG,
    ),
    #[cfg(feature = "tmux_2_9")]
    (
        "status-format",
        |o, i, s| o.status_format = create_insert_vec(o.status_format.as_mut(), i, s),
        |o| o.status_format.as_ref().map(|v| v.join(" ").to_string()),
        STATUS_FORMAT,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "status-interval",
        |o, _, s| o.status_interval = s.parse().ok(),
        |o| o.status_interval.as_ref().map(|v| v.to_string()),
        STATUS_INTERVAL,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "status-justify",
        |o, _, s| o.status_justify = s.parse().ok(),
        |o| o.status_justify.as_ref().map(|v| v.to_string()),
        STATUS_JUSTIFY,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "status-keys",
        |o, _, s| o.status_keys = s.parse().ok(),
        |o| o.status_keys.as_ref().map(|v| v.to_string()),
        STATUS_KEYS,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "status-left",
        |o, _, s| o.status_left = s.parse().ok(),
        |o| o.status_left.as_ref().map(|v| v.to_string()),
        STATUS_LEFT,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "status-left-attr",
        |o, _, s| o.status_left_attr = s.parse().ok(),
        |o| o.status_left_attr.as_ref().map(|v| v.to_string()),
        STATUS_LEFT_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "status-left-bg",
        |o, _, s| o.status_left_bg = s.parse().ok(),
        |o| o.status_left_bg.as_ref().map(|v| v.to_string()),
        STATUS_LEFT_BG,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "status-left-fg",
        |o, _, s| o.status_left_fg = s.parse().ok(),
        |o| o.status_left_fg.as_ref().map(|v| v.to_string()),
        STATUS_LEFT_FG,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "status-left-length",
        |o, _, s| o.status_left_length = s.parse().ok(),
        |o| o.status_left_length.as_ref().map(|v| v.to_string()),
        STATUS_LEFT_LENGTH,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "status-left-style",
        |o, _, s| o.status_left_style = s.parse().ok(),
        |o| o.status_left_style.as_ref().map(|v| v.to_string()),
        STATUS_LEFT_STYLE,
    ),
    #[cfg(feature = "tmux_1_7")]
    (
        "status-position",
        |o, _, s| o.status_position = s.parse().ok(),
        |o| o.status_position.as_ref().map(|v| v.to_string()),
        STATUS_POSITION,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "status-right",
        |o, _, s| o.status_right = s.parse().ok(),
        |o| o.status_right.as_ref().map(|v| v.to_string()),
        STATUS_RIGHT,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "status-right-attr",
        |o, _, s| o.status_right_attr = s.parse().ok(),
        |o| o.status_right_attr.as_ref().map(|v| v.to_string()),
        STATUS_RIGHT_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "status-right-bg",
        |o, _, s| o.status_right_bg = s.parse().ok(),
        |o| o.status_right_bg.as_ref().map(|v| v.to_string()),
        STATUS_RIGHT_BG,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "status-right-fg",
        |o, _, s| o.status_right_fg = s.parse().ok(),
        |o| o.status_right_fg.as_ref().map(|v| v.to_string()),
        STATUS_RIGHT_FG,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "status-right-length",
        |o, _, s| o.status_right_length = s.parse().ok(),
        |o| o.status_right_length.as_ref().map(|v| v.to_string()),
        STATUS_RIGHT_LENGTH,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "status-right-style",
        |o, _, s| o.status_right_style = s.parse().ok(),
        |o| o.status_right_style.as_ref().map(|v| v.to_string()),
        STATUS_RIGHT_STYLE,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "status-style",
        |o, _, s| o.status_style = s.parse().ok(),
        |o| o.status_style.as_ref().map(|v| v.to_string()),
        STATUS_STYLE,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    (
        "status-utf8",
        |o, _, s| o.status_utf8 = s.parse().ok(),
        |o| o.status_utf8.as_ref().map(|v| v.to_string()),
        STATUS_UTF8,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    (
        "terminal-overrides",
        //|o, i, s| o.terminal_overrides = create_insert_vec(o.update_environment.as_mut(), i, s),
        |o, _, s| o.terminal_overrides = s.parse().ok(),
        |o| o.terminal_overrides.as_ref().map(|v| v.to_string()),
        TERMINAL_OVERRIDES,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "update-environment",
        |o, i, s| o.update_environment = create_insert_vec(o.update_environment.as_mut(), i, s),
        |o| o.update_environment.as_ref().map(|v| v.join(" ")),
        UPDATE_ENVIRONMENT,
    ),
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    (
        "user-keys",
        |o, i, s| o.user_keys = create_insert_vec(o.user_keys.as_mut(), i, s),
        |o| o.user_keys.as_ref().map(|v| v.join(" ")),
        USER_KEYS,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "visual-activity",
        |o, _, s| o.visual_activity = s.parse().ok(),
        |o| o.visual_activity.as_ref().map(|v| v.to_string()),
        VISUAL_ACTIVITY,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "visual-bell",
        |o, _, s| o.visual_bell = s.parse().ok(),
        |o| o.visual_bell.as_ref().map(|v| v.to_string()),
        VISUAL_BELL,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    (
        "visial-content",
        |o, _, s| o.visual_content = s.parse().ok(),
        |o| o.visual_content.as_ref().map(|v| v.to_string()),
        VISUAL_CONTENT,
    ),
    #[cfg(feature = "tmux_1_4")]
    (
        "visual-silence",
        |o, _, s| o.visual_silence = s.parse().ok(),
        |o| o.visual_silence.as_ref().map(|v| v.to_string()),
        VISUAL_SILENCE,
    ),
    #[cfg(feature = "tmux_1_6")]
    (
        "word-separators",
        |o, _, s| o.word_separators = s.parse().ok(),
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
    #[cfg(feature = "tmux_2_6")]
    pub activity_action: Option<Activity>,
    //assume-paste-time milliseconds
    #[cfg(feature = "tmux_1_8")]
    pub assume_paste_time: Option<usize>,
    //base-index index
    #[cfg(feature = "tmux_1_0")]
    pub base_index: Option<usize>,
    //bell-action [any | none | current | other]
    // tmux 1.0: bell-action [any | none | other]
    #[cfg(feature = "tmux_1_0")]
    pub bell_action: Option<Action>,
    //bell-on-alert [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub bell_on_alert: Option<Switch>,
    //buffer-limit limit
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    pub buffer_limit: Option<usize>,
    //default-command shell-command
    #[cfg(feature = "tmux_1_0")]
    pub default_command: Option<String>,
    //default-shell path
    #[cfg(feature = "tmux_1_0")]
    pub default_shell: Option<String>,
    //default-path path
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub default_path: Option<String>,
    // default-terminal terminal
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub default_terminal: Option<String>,
    //default-size XxY
    #[cfg(feature = "tmux_2_9")]
    pub default_size: Option<(usize, usize)>,
    //destroy-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    pub destroy_unattached: Option<Switch>,
    //detach-on-destroy [on | off]
    #[cfg(feature = "tmux_1_4")]
    pub detach_on_destroy: Option<Switch>,
    //display-panes-active-colour colour
    #[cfg(feature = "tmux_1_2")]
    pub display_panes_active_colour: Option<String>,
    //display-panes-colour colour
    #[cfg(feature = "tmux_1_0")]
    pub display_panes_colour: Option<String>,
    //display-panes-time time
    #[cfg(feature = "tmux_1_0")]
    pub display_panes_time: Option<usize>,
    //display-time time
    #[cfg(feature = "tmux_1_0")]
    pub display_time: Option<usize>,
    //history-limit lines
    #[cfg(feature = "tmux_1_0")]
    pub history_limit: Option<usize>,
    //key-table key-table
    #[cfg(feature = "tmux_2_2")]
    pub key_table: Option<String>,
    //lock-after-time number
    #[cfg(feature = "tmux_1_0")]
    pub lock_after_time: Option<usize>,
    //lock-command shell-command
    #[cfg(feature = "tmux_1_1")]
    pub lock_command: Option<String>,
    //lock-server [on | off]
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    pub lock_server: Option<String>,
    //message-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_attr: Option<String>,
    //message-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_bg: Option<String>,
    //message-command-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_attr: Option<String>,
    //message-command-bg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_bg: Option<String>,
    //message-command-fg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_fg: Option<String>,
    //message-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_fg: Option<String>,
    //message-command-style style
    #[cfg(feature = "tmux_1_9")]
    pub message_command_style: Option<String>,
    //message-limit number
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub message_limit: Option<usize>,
    //message-style style
    #[cfg(feature = "tmux_1_9")]
    pub message_style: Option<String>,
    //mouse-resize-pane [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_resize_pane: Option<Switch>,
    //mouse-select-pane [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_select_pane: Option<Switch>,
    //mouse-select-window [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_select_window: Option<Switch>,
    //mouse [on | off]
    #[cfg(feature = "tmux_2_1")]
    pub mouse: Option<Switch>,
    // mouse-utf8 [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    pub mouse_utf8: Option<Switch>,
    //pane-active-border-bg colour
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_active_border_bg: Option<String>,
    //pane-active-border-fg colour
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_active_border_fg: Option<String>,
    //pane-border-bg colour
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_border_bg: Option<String>,
    //pane-border-fg colour
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_border_fg: Option<String>,
    //pane-active-border-style style
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub pane_active_border_style: Option<String>,
    //pane-border-style style
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub pane_border_style: Option<String>,
    //prefix key
    #[cfg(feature = "tmux_1_0")]
    pub prefix: Option<String>,
    //prefix2 key
    #[cfg(feature = "tmux_1_6")]
    pub prefix2: Option<String>,
    //renumber-windows [on | off]
    #[cfg(feature = "tmux_1_7")]
    pub renumber_windows: Option<Switch>,
    //repeat-time time
    #[cfg(feature = "tmux_1_0")]
    pub repeat_time: Option<usize>,
    //set-remain-on-exit [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub set_remain_on_exit: Option<Switch>,
    //set-titles [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub set_titles: Option<Switch>,
    //set-titles-string string
    #[cfg(feature = "tmux_1_0")]
    pub set_titles_string: Option<String>,
    //silence-action [any | none | current | other]
    #[cfg(feature = "tmux_2_6")]
    pub silence_action: Option<Action>,
    //status [off | on | 2 | 3 | 4 | 5]
    //tmux 1.0: status [off | on]
    #[cfg(feature = "tmux_1_0")]
    pub status: Option<Status>,
    //status-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_attr: Option<String>,
    //status-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_bg: Option<String>,
    //status-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_fg: Option<String>,
    //status-format[] format
    #[cfg(feature = "tmux_2_9")]
    pub status_format: Option<Vec<String>>,
    //status-interval interval
    #[cfg(feature = "tmux_1_0")]
    pub status_interval: Option<usize>,
    //status-justify [left | centre | right]
    #[cfg(feature = "tmux_1_0")]
    pub status_justify: Option<StatusJustify>,
    //status-keys [vi | emacs]
    #[cfg(feature = "tmux_1_0")]
    pub status_keys: Option<StatusKeys>,
    //status-left string
    #[cfg(feature = "tmux_1_0")]
    pub status_left: Option<String>,
    //status-left-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_attr: Option<String>,
    //status-left-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_bg: Option<String>,
    //status-left-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_fg: Option<String>,
    //status-left-length length
    #[cfg(feature = "tmux_1_0")]
    pub status_left_length: Option<usize>,
    //status-left-style style
    #[cfg(feature = "tmux_1_9")]
    pub status_left_style: Option<String>,
    //status-position [top | bottom]
    #[cfg(feature = "tmux_1_7")]
    pub status_position: Option<StatusPosition>,
    //status-right string
    #[cfg(feature = "tmux_1_0")]
    pub status_right: Option<String>,
    //status-right-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_attr: Option<String>,
    //status-right-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_bg: Option<String>,
    //status-right-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_fg: Option<String>,
    //status-right-length length
    #[cfg(feature = "tmux_1_0")]
    pub status_right_length: Option<usize>,
    //status-right-style style
    #[cfg(feature = "tmux_1_9")]
    pub status_right_style: Option<String>,
    //status-style style
    #[cfg(feature = "tmux_1_9")]
    pub status_style: Option<String>,
    //status-utf8 [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub status_utf8: Option<Switch>,
    //terminal-overrides string
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub terminal_overrides: Option<String>,
    //update-environment[] variable
    #[cfg(feature = "tmux_1_0")]
    pub update_environment: Option<Vec<String>>,
    //user-keys
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    pub user_keys: Option<Vec<String>>,
    //visual-activity [on | off | both]
    //tmux 1.0: visual-activity [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub visual_activity: Option<Activity>,
    //visual-bell [on | off | both]
    //tmux 1.0: visual-bell [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub visual_bell: Option<Activity>,
    //visual-content [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub visual_content: Option<Switch>,
    //visual-silence [on | off | both]
    #[cfg(feature = "tmux_1_4")]
    pub visual_silence: Option<Activity>,
    //word-separators string
    #[cfg(feature = "tmux_1_6")]
    pub word_separators: Option<String>,
    //pub user_options: Option<HashMap<String, String>>
}

impl SessionOptions {
    pub fn get_all() -> Result<Self, Error> {
        let s = ShowOptions::new().global().output()?.to_string();
        s.parse()
    }

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    #[cfg(feature = "tmux_1_7")]
    pub fn get(bitflags: u128) -> Result<Self, Error> {
        let selected_option = SESSION_OPTIONS
            .iter()
            .filter(|t| bitflags == t.3)
            .map(|t| t.0.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        let s = ShowOptions::new()
            .option(&selected_option)
            .output()?
            .to_string();
        s.parse()
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn get_global(bitflags: u128) -> Result<Self, Error> {
        let selected_option = SESSION_OPTIONS
            .iter()
            .filter(|t| bitflags == t.3)
            .map(|t| t.0.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        let s = ShowOptions::new()
            .option(&selected_option)
            .global()
            .output()?
            .to_string();
        s.parse()
    }

    // allows selective set by bitmask
    pub fn set(&self, bitflags: u128) -> Result<(), Error> {
        for selected_option in SESSION_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
            if let Some(selected_value) = selected_option.2(&self) {
                SetOption::new()
                    .option(selected_option.0)
                    .option(&selected_value);
            }
        }
        Ok(())
    }

    pub fn set_global(&self, bitflags: u128) -> Result<(), Error> {
        for selected_option in SESSION_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
            if let Some(selected_value) = selected_option.2(&self) {
                SetOption::new()
                    .global()
                    .option(selected_option.0)
                    .value(&selected_value);
            }
        }
        Ok(())
    }

    // XXX: single set get methods
}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?
impl FromStr for SessionOptions {
    type Err = Error;

    fn from_str(options: &str) -> Result<Self, Self::Err> {
        let mut session_options: SessionOptions = Default::default();
        let mut v: Vec<&str>;
        let mut arr: Vec<&str>;
        for option in options.lines() {
            v = option.trim().splitn(2, ' ').collect();
            arr = v[0].split(|c| c == '[' || c == ']').collect();
            for session_var in SESSION_OPTIONS.iter() {
                if session_var.0 == arr[0] {
                    session_var.1(
                        &mut session_options,
                        arr.get(1).and_then(|i| i.parse::<usize>().ok()),
                        v.get(1).unwrap_or(&""),
                    )
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
                writeln!(f, "{} {}", var.0, v)?;
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct SessionOptionsBuilder<'a> {
    #[cfg(feature = "tmux_2_6")]
    pub activity_action: Option<Activity>,
    #[cfg(feature = "tmux_1_8")]
    pub assume_paste_time: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub base_index: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub bell_action: Option<Action>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub bell_on_alert: Option<Switch>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    pub buffer_limit: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub default_command: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub default_shell: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub default_path: Option<&'a str>,
    #[cfg(feature = "tmux_2_9")]
    pub default_size: Option<(usize, usize)>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub default_terminal: Option<&'a str>,
    #[cfg(feature = "tmux_1_4")]
    pub destroy_unattached: Option<Switch>,
    #[cfg(feature = "tmux_1_4")]
    pub detach_on_destroy: Option<Switch>,
    #[cfg(feature = "tmux_1_2")]
    pub display_panes_active_colour: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub display_panes_colour: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub display_panes_time: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub display_time: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub history_limit: Option<usize>,
    #[cfg(feature = "tmux_2_2")]
    pub key_table: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub lock_after_time: Option<usize>,
    #[cfg(feature = "tmux_1_1")]
    pub lock_command: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    pub lock_server: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_fg: Option<&'a str>,
    #[cfg(feature = "tmux_1_9")]
    pub message_command_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub message_limit: Option<usize>,
    #[cfg(feature = "tmux_1_9")]
    pub message_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_resize_pane: Option<Switch>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_select_pane: Option<Switch>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_select_window: Option<Switch>,
    #[cfg(feature = "tmux_2_1")]
    pub mouse: Option<Switch>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    pub mouse_utf8: Option<Switch>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_active_border_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_active_border_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_border_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_border_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub pane_active_border_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub pane_border_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub prefix: Option<&'a str>,
    #[cfg(feature = "tmux_1_6")]
    pub prefix2: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub renumber_windows: Option<Switch>,
    #[cfg(feature = "tmux_1_0")]
    pub repeat_time: Option<usize>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub set_remain_on_exit: Option<Switch>,
    #[cfg(feature = "tmux_1_0")]
    pub set_titles: Option<Switch>,
    #[cfg(feature = "tmux_1_0")]
    pub set_titles_string: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub silence_action: Option<Action>,
    #[cfg(feature = "tmux_1_0")]
    pub status: Option<Status>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_fg: Option<&'a str>,
    #[cfg(feature = "tmux_2_9")]
    pub status_format: Option<Vec<&'a str>>,
    #[cfg(feature = "tmux_1_0")]
    pub status_interval: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub status_justify: Option<StatusJustify>,
    #[cfg(feature = "tmux_1_0")]
    pub status_keys: Option<StatusKeys>,
    #[cfg(feature = "tmux_1_0")]
    pub status_left: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_fg: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub status_left_length: Option<usize>,
    #[cfg(feature = "tmux_1_9")]
    pub status_left_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub status_position: Option<StatusPosition>,
    #[cfg(feature = "tmux_1_0")]
    pub status_right: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_fg: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub status_right_length: Option<usize>,
    #[cfg(feature = "tmux_1_9")]
    pub status_right_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_9")]
    pub status_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub status_utf8: Option<Switch>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub terminal_overrides: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub update_environment: Option<Vec<&'a str>>,
    #[cfg(feature = "tmux_1_0")]
    pub visual_activity: Option<Activity>,
    #[cfg(feature = "tmux_1_0")]
    pub visual_bell: Option<Activity>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub visual_content: Option<Switch>,
    #[cfg(feature = "tmux_1_4")]
    pub visual_silence: Option<Activity>,
    #[cfg(feature = "tmux_1_6")]
    pub word_separators: Option<&'a str>,
}

impl<'a> SessionOptionsBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn activity_action(&mut self, activity_action: Activity) -> &mut Self {
        self.activity_action = Some(activity_action);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn assume_paste_time(&mut self, assume_paste_time: usize) -> &mut Self {
        self.assume_paste_time = Some(assume_paste_time);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn base_index(&mut self, base_index: usize) -> &mut Self {
        self.base_index = Some(base_index);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn bell_action(&mut self, bell_action: Action) -> &mut Self {
        self.bell_action = Some(bell_action);
        self
    }

    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub fn bell_on_alert(&mut self, bell_on_alert: Switch) -> &mut Self {
        self.bell_on_alert = Some(bell_on_alert);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn default_command(&mut self, default_command: &'a str) -> &mut Self {
        self.default_command = Some(default_command);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn default_shell(&mut self, default_shell: &'a str) -> &mut Self {
        self.default_shell = Some(default_shell);
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn default_size(&mut self, default_size: (usize, usize)) -> &mut Self {
        self.default_size = Some(default_size);
        self
    }

    //#[cfg(feature = "tmux_2_9")]
    //pub fn default_terminal(&mut self, default_terminal: (usize, usize)) -> &mut Self {
    //self.default_size = Some(default_size);
    //self
    //}

    #[cfg(feature = "tmux_1_4")]
    pub fn destroy_unattached(&mut self, destroy_unattached: Switch) -> &mut Self {
        self.destroy_unattached = Some(destroy_unattached);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn detach_on_destroy(&mut self, detach_on_destroy: Switch) -> &mut Self {
        self.detach_on_destroy = Some(detach_on_destroy);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn display_panes_active_colour(
        &mut self,
        display_panes_active_colour: &'a str,
    ) -> &mut Self {
        self.display_panes_active_colour = Some(display_panes_active_colour);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_colour(&mut self, display_panes_colour: &'a str) -> &mut Self {
        self.display_panes_colour = Some(display_panes_colour);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_time(&mut self, display_panes_time: usize) -> &mut Self {
        self.display_panes_time = Some(display_panes_time);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_time(&mut self, display_time: usize) -> &mut Self {
        self.display_time = Some(display_time);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn history_limit(&mut self, history_limit: usize) -> &mut Self {
        self.history_limit = Some(history_limit);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn key_table(&mut self, key_table: &'a str) -> &mut Self {
        self.key_table = Some(key_table);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn lock_after_time(&mut self, lock_after_time: usize) -> &mut Self {
        self.lock_after_time = Some(lock_after_time);
        self
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn lock_command(&mut self, lock_command: &'a str) -> &mut Self {
        self.lock_command = Some(lock_command);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn message_command_style(&mut self, message_command_style: &'a str) -> &mut Self {
        self.message_command_style = Some(message_command_style);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn message_style(&mut self, message_style: &'a str) -> &mut Self {
        self.message_style = Some(message_style);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn mouse(&mut self, mouse: Switch) -> &mut Self {
        self.mouse = Some(mouse);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn prefix(&mut self, prefix: &'a str) -> &mut Self {
        self.prefix = Some(prefix);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn prefix2(&mut self, prefix2: &'a str) -> &mut Self {
        self.prefix2 = Some(prefix2);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn renumber_windows(&mut self, renumber_windows: Switch) -> &mut Self {
        self.renumber_windows = Some(renumber_windows);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn repeat_time(&mut self, repeat_time: usize) -> &mut Self {
        self.repeat_time = Some(repeat_time);
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub fn set_remain_on_exit(&mut self, set_remain_on_exit: Switch) -> &mut Self {
        self.set_remain_on_exit = Some(set_remain_on_exit);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles(&mut self, set_titles: Switch) -> &mut Self {
        self.set_titles = Some(set_titles);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles_string(&mut self, set_titles_string: &'a str) -> &mut Self {
        self.set_titles_string = Some(set_titles_string);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn silence_action(&mut self, silence_action: Action) -> &mut Self {
        self.silence_action = Some(silence_action);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn status_format(&mut self, status_format: Vec<&'a str>) -> &mut Self {
        self.status_format = Some(status_format);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_interval(&mut self, status_interval: usize) -> &mut Self {
        self.status_interval = Some(status_interval);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_justify(&mut self, status_justify: StatusJustify) -> &mut Self {
        self.status_justify = Some(status_justify);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_keys(&mut self, status_keys: StatusKeys) -> &mut Self {
        self.status_keys = Some(status_keys);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_left(&mut self, status_left: &'a str) -> &mut Self {
        self.status_left = Some(status_left);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_left_length(&mut self, status_left_length: usize) -> &mut Self {
        self.status_left_length = Some(status_left_length);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_left_style(&mut self, status_left_style: &'a str) -> &mut Self {
        self.status_left_style = Some(status_left_style);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn status_position(&mut self, status_position: StatusPosition) -> &mut Self {
        self.status_position = Some(status_position);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_right(&mut self, status_right: &'a str) -> &mut Self {
        self.status_right = Some(status_right);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_right_length(&mut self, status_right_length: usize) -> &mut Self {
        self.status_right_length = Some(status_right_length);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_right_style(&mut self, status_right_style: &'a str) -> &mut Self {
        self.status_right_style = Some(status_right_style);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_style(&mut self, status_style: &'a str) -> &mut Self {
        self.status_style = Some(status_style);
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub fn status_utf8(&mut self, status_utf8: Switch) -> &mut Self {
        self.status_utf8 = Some(status_utf8);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn update_environment(&mut self, update_environment: Vec<&'a str>) -> &mut Self {
        self.update_environment = Some(update_environment);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn visual_activity(&mut self, visual_activity: Activity) -> &mut Self {
        self.visual_activity = Some(visual_activity);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn visual_bell(&mut self, visual_bell: Activity) -> &mut Self {
        self.visual_bell = Some(visual_bell);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn visual_silence(&mut self, visual_silence: Activity) -> &mut Self {
        self.visual_silence = Some(visual_silence);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn word_separators(&mut self, word_separators: &'a str) -> &mut Self {
        self.word_separators = Some(word_separators);
        self
    }

    // XXX: clone rly needed?
    pub fn build(&self) -> SessionOptions {
        SessionOptions {
            #[cfg(feature = "tmux_2_6")]
            activity_action: self.activity_action.clone(),
            #[cfg(feature = "tmux_1_8")]
            assume_paste_time: self.assume_paste_time,
            #[cfg(feature = "tmux_1_0")]
            base_index: self.base_index,
            #[cfg(feature = "tmux_1_0")]
            bell_action: self.bell_action.clone(),
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
            bell_on_alert: self.bell_on_alert.clone(),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
            buffer_limit: self.buffer_limit,
            #[cfg(feature = "tmux_1_0")]
            default_command: self.default_command.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_0")]
            default_shell: self.default_shell.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            default_path: self.default_path.map(|s| s.to_string()),
            #[cfg(feature = "tmux_2_9")]
            default_size: self.default_size,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
            default_terminal: self.default_terminal.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_4")]
            destroy_unattached: self.destroy_unattached.clone(),
            #[cfg(feature = "tmux_1_4")]
            detach_on_destroy: self.detach_on_destroy.clone(),
            #[cfg(feature = "tmux_1_2")]
            display_panes_active_colour: self.display_panes_active_colour.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_0")]
            display_panes_colour: self.display_panes_colour.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_0")]
            display_panes_time: self.display_panes_time,
            #[cfg(feature = "tmux_1_0")]
            display_time: self.display_time,
            #[cfg(feature = "tmux_1_0")]
            history_limit: self.history_limit,
            #[cfg(feature = "tmux_2_2")]
            key_table: self.key_table.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_1")]
            lock_after_time: self.lock_after_time,
            #[cfg(feature = "tmux_1_1")]
            lock_command: self.lock_command.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
            lock_server: self.lock_server.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            message_attr: self.message_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            message_bg: self.message_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            message_command_attr: self.message_command_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            message_command_bg: self.message_command_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            message_command_fg: self.message_command_fg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            message_fg: self.message_fg.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_9")]
            message_command_style: self.message_command_style.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            message_limit: self.message_limit,
            #[cfg(feature = "tmux_1_9")]
            message_style: self.message_style.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
            mouse_resize_pane: self.mouse_resize_pane.clone(),
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
            mouse_select_pane: self.mouse_select_pane.clone(),
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
            mouse_select_window: self.mouse_select_window.clone(),
            #[cfg(feature = "tmux_2_1")]
            mouse: self.mouse.clone(),
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
            mouse_utf8: self.mouse_utf8.clone(),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
            pane_active_border_bg: self.pane_active_border_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
            pane_active_border_fg: self.pane_active_border_fg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
            pane_border_bg: self.pane_border_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
            pane_border_fg: self.pane_border_fg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            pane_active_border_style: self.pane_active_border_style.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            pane_border_style: self.pane_border_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_0")]
            prefix: self.prefix.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_6")]
            prefix2: self.prefix2.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_7")]
            renumber_windows: self.renumber_windows.clone(),
            #[cfg(feature = "tmux_1_0")]
            repeat_time: self.repeat_time,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
            set_remain_on_exit: self.set_remain_on_exit.clone(),
            #[cfg(feature = "tmux_1_0")]
            set_titles: self.set_titles.clone(),
            #[cfg(feature = "tmux_1_0")]
            set_titles_string: self.set_titles_string.map(|s| s.to_string()),
            #[cfg(feature = "tmux_2_6")]
            silence_action: self.silence_action.clone(),
            #[cfg(feature = "tmux_1_0")]
            status: self.status.clone(),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_attr: self.status_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_bg: self.status_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_fg: self.status_fg.map(|s| s.to_string()),
            #[cfg(feature = "tmux_2_9")]
            status_format: None,
            #[cfg(feature = "tmux_1_0")]
            status_interval: self.status_interval,
            #[cfg(feature = "tmux_1_0")]
            status_justify: self.status_justify.clone(),
            #[cfg(feature = "tmux_1_0")]
            status_keys: self.status_keys.clone(),
            #[cfg(feature = "tmux_1_0")]
            status_left: self.status_left.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_left_attr: self.status_left_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_left_bg: self.status_left_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_left_fg: self.status_left_fg.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_0")]
            status_left_length: self.status_left_length,
            #[cfg(feature = "tmux_1_9")]
            status_left_style: self.status_left_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_7")]
            status_position: self.status_position.clone(),
            #[cfg(feature = "tmux_1_0")]
            status_right: self.status_right.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_right_attr: self.status_right_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_right_bg: self.status_right_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_right_fg: self.status_right_fg.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_0")]
            status_right_length: self.status_right_length,
            #[cfg(feature = "tmux_1_9")]
            status_right_style: self.status_right_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_9")]
            status_style: self.status_style.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
            status_utf8: self.status_utf8.clone(),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
            terminal_overrides: self.terminal_overrides.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_0")]
            update_environment: None,
            #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
            user_keys: None,
            #[cfg(feature = "tmux_1_0")]
            visual_activity: self.visual_activity.clone(),
            #[cfg(feature = "tmux_1_0")]
            visual_bell: self.visual_bell.clone(),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
            visual_content: self.visual_content.clone(),
            #[cfg(feature = "tmux_1_4")]
            visual_silence: self.visual_silence.clone(),
            #[cfg(feature = "tmux_1_6")]
            word_separators: self.word_separators.map(|s| s.to_string()),
        }
    }
}
