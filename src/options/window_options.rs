use crate::options::StatusKeys;
use crate::{Error, SetOption, ShowOptions, Switch};
use std::fmt;
use std::str::FromStr;

//clock-mode-style [12 | 24]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_1_0")]
pub enum ClockModeStyle {
    _12,
    _24,
}

#[cfg(feature = "tmux_1_0")]
impl FromStr for ClockModeStyle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "12" => Ok(Self::_12),
            "24" => Ok(Self::_24),
            _ => Err(Error::ParseClockModeStyle),
        }
    }
}

#[cfg(feature = "tmux_1_0")]
impl fmt::Display for ClockModeStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::_12 => write!(f, "12"),
            Self::_24 => write!(f, "24"),
        }
    }
}

//pane-border-status [off | top | bottom]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_2_3")]
pub enum PaneBorderStatus {
    Off,
    Top,
    Bottom,
}

#[cfg(feature = "tmux_2_3")]
impl FromStr for PaneBorderStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "off" => Ok(Self::Off),
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            _ => Err(Error::ParsePaneBorderStatus),
        }
    }
}

#[cfg(feature = "tmux_2_3")]
impl fmt::Display for PaneBorderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Off => write!(f, "off"),
            Self::Top => write!(f, "top"),
            Self::Bottom => write!(f, "bottom"),
        }
    }
}

//window-size largest | smallest | manual | latest
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_2_9")]
pub enum WindowSize {
    Largest,
    Smallest,
    Manual,
    #[cfg(feature = "tmux_3_1")]
    Latest,
}

#[cfg(feature = "tmux_2_9")]
impl FromStr for WindowSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "largest" => Ok(Self::Largest),
            "smallest" => Ok(Self::Smallest),
            "manual" => Ok(Self::Manual),
            #[cfg(feature = "tmux_3_1")]
            "latest" => Ok(Self::Latest),
            _ => Err(Error::ParseWindowSize),
        }
    }
}

#[cfg(feature = "tmux_2_9")]
impl fmt::Display for WindowSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Largest => write!(f, "largest"),
            Self::Smallest => write!(f, "smallest"),
            Self::Manual => write!(f, "manual"),
            #[cfg(feature = "tmux_3_1")]
            Self::Latest => write!(f, "latest"),
        }
    }
}

// XXX: conditionals?
// NOTE: total num: 70 (u128)
pub const AGGRESIVE_RESIZE: u128 = 1;
pub const ALLOW_RENAME: u128 = 1 << 1;
pub const ALTERNAME_SCREEN: u128 = 1 << 2;
pub const AUTOMATIC_RENAME: u128 = 1 << 3;
pub const AUTOMATIC_RENAME_FORMAT: u128 = 1 << 4;
pub const C0_CHANGE_INTERVAL: u128 = 1 << 5;
pub const C0_CHANGE_TRIGGER: u128 = 1 << 6;
pub const CLOCK_MODE_COLOUR: u128 = 1 << 7;
pub const CLOCK_MODE_STYLE: u128 = 1 << 8;
pub const FORCE_HEIGHT: u128 = 1 << 9;
pub const FORCE_WIDTH: u128 = 1 << 10;
pub const LAYOUT_HISTORY_LIMIT: u128 = 1 << 11;
pub const MAIN_PANE_HEIGHT: u128 = 1 << 12;
pub const MAIN_PANE_WIDTH: u128 = 1 << 13;
pub const MODE_ATTR: u128 = 1 << 14;
pub const MODE_BG: u128 = 1 << 15;
pub const MODE_FG: u128 = 1 << 16;
pub const MODE_KEYS: u128 = 1 << 17;
pub const MODE_MOUSE: u128 = 1 << 18;
pub const MODE_STYLE: u128 = 1 << 19;
pub const MONITOR_ACTIVITY: u128 = 1 << 20;
pub const MONITOR_CONTENT: u128 = 1 << 21;
pub const MONITOR_BELL: u128 = 1 << 22;
pub const MONITOR_SILENCE: u128 = 1 << 23;
pub const OTHER_PANE_HEIGHT: u128 = 1 << 24;
pub const OTHER_PANE_WIDTH: u128 = 1 << 25;
pub const PANE_ACTIVE_BORDER_STYLE: u128 = 1 << 26;
pub const PANE_BASE_INDEX: u128 = 1 << 27;
pub const PANE_BORDER_FORMAT: u128 = 1 << 28;
pub const PANE_BORDER_STATUS: u128 = 1 << 29;
pub const PANE_BORDER_STYLE: u128 = 1 << 30;
pub const REMAIN_ON_EXIT: u128 = 1 << 31;
pub const SYNCHRONIZE_PANES: u128 = 1 << 32;
pub const UTF8: u128 = 1 << 33;
pub const WINDOW_ACTIVE_STYLE: u128 = 1 << 34;
pub const WINDOW_STATUS_BELL_ATTR: u128 = 1 << 35;
pub const WINDOW_STATUS_BELL_BG: u128 = 1 << 36;
pub const WINDOW_STATUS_BELL_FG: u128 = 1 << 37;
pub const WINDOW_STATUS_CONTENT_ATTR: u128 = 1 << 38;
pub const WINDOW_STATUS_CONTENT_BG: u128 = 1 << 39;
pub const WINDOW_STATUS_CONTENT_FG: u128 = 1 << 40;
pub const WINDOW_STATUS_ACTIVITY_ATTR: u128 = 1 << 41;
pub const WINDOW_STATUS_ACTIVITY_BG: u128 = 1 << 42;
pub const WINDOW_STATUS_ACTIVITY_FG: u128 = 1 << 43;
pub const WINDOW_STATUS_ATTR: u128 = 1 << 44;
pub const WINDOW_STATUS_BG: u128 = 1 << 45;
pub const WINDOW_STATUS_FG: u128 = 1 << 46;
pub const WINDOW_STATUS_CURRENT_ATTR: u128 = 1 << 47;
pub const WINDOW_STATUS_CURRENT_BG: u128 = 1 << 48;
pub const WINDOW_STATUS_CURRENT_FG: u128 = 1 << 49;
pub const WINDOW_STATUS_ALERT_ATTR: u128 = 1 << 50;
pub const WINDOW_STATUS_ALERT_BG: u128 = 1 << 51;
pub const WINDOW_STATUS_ALERT_FG: u128 = 1 << 52;
pub const WINDOW_STATUS_ACTIVITY_STYLE: u128 = 1 << 53;
pub const WINDOW_STATUS_BELL_STYLE: u128 = 1 << 54;
pub const WINDOW_STATUS_CONTENT_STYLE: u128 = 1 << 55;
pub const WINDOW_STATUS_CURRENT_FORMAT: u128 = 1 << 56;
pub const WINDOW_STATUS_LAST_ATTR: u128 = 1 << 57;
pub const WINDOW_STATUS_LAST_BG: u128 = 1 << 58;
pub const WINDOW_STATUS_LAST_FG: u128 = 1 << 59;
pub const WINDOW_STATUS_CURRENT_STYLE: u128 = 1 << 60;
pub const WINDOW_STATUS_FORMAT: u128 = 1 << 61;
pub const WINDOW_STATUS_LAST_STYLE: u128 = 1 << 62;
pub const WINDOW_STATUS_SEPARATOR: u128 = 1 << 63;
pub const WINDOW_STATUS_STYLE: u128 = 1 << 64;
pub const WINDOW_SIZE: u128 = 1 << 65;
pub const WORD_SEPARATORS: u128 = 1 << 66;
pub const WINDOW_STYLE: u128 = 1 << 67;
pub const WRAP_SEARCH: u128 = 1 << 68;
pub const XTERM_KEYS: u128 = 1 << 69;

pub const WINDOW_OPTIONS_NONE: u128 = 0;
////pub const SERVER_OPTIONS_DEFAULT: usize = ;
pub const WINDOW_OPTIONS_ALL: u128 = AGGRESIVE_RESIZE
    | ALLOW_RENAME
    | ALTERNAME_SCREEN
    | AUTOMATIC_RENAME
    | AUTOMATIC_RENAME_FORMAT
    | C0_CHANGE_INTERVAL
    | C0_CHANGE_TRIGGER
    | CLOCK_MODE_COLOUR
    | CLOCK_MODE_STYLE
    | FORCE_HEIGHT
    | FORCE_WIDTH
    | LAYOUT_HISTORY_LIMIT
    | MAIN_PANE_HEIGHT
    | MAIN_PANE_WIDTH
    | MODE_ATTR
    | MODE_BG
    | MODE_FG
    | MODE_KEYS
    | MODE_MOUSE
    | MODE_STYLE
    | MONITOR_ACTIVITY
    | MONITOR_CONTENT
    | MONITOR_BELL
    | MONITOR_SILENCE
    | OTHER_PANE_HEIGHT
    | OTHER_PANE_WIDTH
    | PANE_ACTIVE_BORDER_STYLE
    | PANE_BASE_INDEX
    | PANE_BORDER_FORMAT
    | PANE_BORDER_STATUS
    | PANE_BORDER_STYLE
    | REMAIN_ON_EXIT
    | SYNCHRONIZE_PANES
    | UTF8
    | WINDOW_ACTIVE_STYLE
    | WINDOW_STATUS_BELL_ATTR
    | WINDOW_STATUS_BELL_BG
    | WINDOW_STATUS_BELL_FG
    | WINDOW_STATUS_CONTENT_ATTR
    | WINDOW_STATUS_CONTENT_BG
    | WINDOW_STATUS_CONTENT_FG
    | WINDOW_STATUS_ACTIVITY_ATTR
    | WINDOW_STATUS_ACTIVITY_BG
    | WINDOW_STATUS_ACTIVITY_FG
    | WINDOW_STATUS_ATTR
    | WINDOW_STATUS_BG
    | WINDOW_STATUS_FG
    | WINDOW_STATUS_CURRENT_ATTR
    | WINDOW_STATUS_CURRENT_BG
    | WINDOW_STATUS_CURRENT_FG
    | WINDOW_STATUS_ALERT_ATTR
    | WINDOW_STATUS_ALERT_BG
    | WINDOW_STATUS_ALERT_FG
    | WINDOW_STATUS_ACTIVITY_STYLE
    | WINDOW_STATUS_BELL_STYLE
    | WINDOW_STATUS_CONTENT_STYLE
    | WINDOW_STATUS_CURRENT_FORMAT
    | WINDOW_STATUS_LAST_ATTR
    | WINDOW_STATUS_LAST_BG
    | WINDOW_STATUS_LAST_FG
    | WINDOW_STATUS_CURRENT_STYLE
    | WINDOW_STATUS_FORMAT
    | WINDOW_STATUS_LAST_STYLE
    | WINDOW_STATUS_SEPARATOR
    | WINDOW_STATUS_STYLE
    | WINDOW_SIZE
    | WORD_SEPARATORS
    | WINDOW_STYLE
    | WRAP_SEARCH
    | XTERM_KEYS;

#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
pub const WINDOW_OPTIONS_NUM: usize = 0;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_1")))]
pub const WINDOW_OPTIONS_NUM: usize = 24;
#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
pub const WINDOW_OPTIONS_NUM: usize = 25;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_3")))]
pub const WINDOW_OPTIONS_NUM: usize = 29;
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
pub const WINDOW_OPTIONS_NUM: usize = 32;
#[cfg(all(feature = "tmux_1_4", not(feature = "tmux_1_5")))]
pub const WINDOW_OPTIONS_NUM: usize = 35;
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_1_6")))]
pub const WINDOW_OPTIONS_NUM: usize = 35;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_7")))]
pub const WINDOW_OPTIONS_NUM: usize = 42;
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
pub const WINDOW_OPTIONS_NUM: usize = 47;
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_OPTIONS_NUM: usize = 49;
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_1_9a")))]
pub const WINDOW_OPTIONS_NUM: usize = 36;
#[cfg(all(feature = "tmux_1_9a", not(feature = "tmux_2_0")))]
pub const WINDOW_OPTIONS_NUM: usize = 36;
#[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_1")))]
pub const WINDOW_OPTIONS_NUM: usize = 36;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
pub const WINDOW_OPTIONS_NUM: usize = 35;
#[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_3")))]
pub const WINDOW_OPTIONS_NUM: usize = 34;
#[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
pub const WINDOW_OPTIONS_NUM: usize = 36;
#[cfg(all(feature = "tmux_2_4", not(feature = "tmux_2_5")))]
pub const WINDOW_OPTIONS_NUM: usize = 36;
#[cfg(all(feature = "tmux_2_5", not(feature = "tmux_2_6")))]
pub const WINDOW_OPTIONS_NUM: usize = 36;
#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_2_7")))]
pub const WINDOW_OPTIONS_NUM: usize = 37;
#[cfg(all(feature = "tmux_2_7", not(feature = "tmux_2_8")))]
pub const WINDOW_OPTIONS_NUM: usize = 37;
#[cfg(all(feature = "tmux_2_8", not(feature = "tmux_2_9")))]
pub const WINDOW_OPTIONS_NUM: usize = 37;
#[cfg(all(feature = "tmux_2_9", not(feature = "tmux_2_9a")))]
pub const WINDOW_OPTIONS_NUM: usize = 36; // NOTE: 37 with double window-size entry
#[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_0")))]
pub const WINDOW_OPTIONS_NUM: usize = 36; // NOTE: 37 with double window-size entry
#[cfg(all(feature = "tmux_3_0", not(feature = "tmux_3_0a")))]
pub const WINDOW_OPTIONS_NUM: usize = 31;
#[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_1")))]
pub const WINDOW_OPTIONS_NUM: usize = 31;
#[cfg(all(feature = "tmux_3_1", not(feature = "tmux_3_1a")))]
pub const WINDOW_OPTIONS_NUM: usize = 31;
#[cfg(all(feature = "tmux_3_1a", not(feature = "tmux_3_1b")))]
pub const WINDOW_OPTIONS_NUM: usize = 31;
#[cfg(all(feature = "tmux_3_1b", not(feature = "tmux_X_X")))]
pub const WINDOW_OPTIONS_NUM: usize = 31;
#[cfg(feature = "tmux_X_X")]
pub const WINDOW_OPTIONS_NUM: usize = 31; // FIXME: must be 33? 2 added

// TODO: waiting for const generics stabilization https://github.com/rust-lang/rust/issues/44580
pub const WINDOW_OPTIONS: [(
    &str,
    fn(o: &mut WindowOptions, i: Option<usize>, s: &str),
    fn(o: &WindowOptions) -> Option<String>,
    u128,
); WINDOW_OPTIONS_NUM] = [
    #[cfg(feature = "tmux_1_0")]
    (
        "aggressive-resize",
        |o, _, s| o.aggressive_resize = s.parse().ok(),
        |o| o.aggressive_resize.as_ref().map(|v| v.to_string()),
        AGGRESIVE_RESIZE,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    (
        "allow-rename",
        |o, _, s| o.allow_rename = s.parse().ok(),
        |o| o.allow_rename.as_ref().map(|v| v.to_string()),
        ALLOW_RENAME,
    ),
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    (
        "alternate-screen",
        |o, _, s| o.alternate_screen = s.parse().ok(),
        |o| o.alternate_screen.as_ref().map(|v| v.to_string()),
        ALTERNAME_SCREEN,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "automatic-rename",
        |o, _, s| o.automatic_rename = s.parse().ok(),
        |o| o.automatic_rename.as_ref().map(|v| v.to_string()),
        AUTOMATIC_RENAME,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "automatic-rename-format",
        |o, _, s| o.automatic_rename_format = Some(s.to_string()),
        |o| {
            o.automatic_rename_format
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        AUTOMATIC_RENAME_FORMAT,
    ),
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    (
        "c0-change-interval",
        |o, _, s| o.c0_change_interval = s.parse().ok(),
        |o| o.c0_change_interval.as_ref().map(|v| v.to_string()),
        C0_CHANGE_INTERVAL,
    ),
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    (
        "c0-change-trigger",
        |o, _, s| o.c0_change_trigger = s.parse().ok(),
        |o| o.c0_change_trigger.as_ref().map(|v| v.to_string()),
        C0_CHANGE_TRIGGER,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "clock-mode-colour",
        |o, _, s| o.clock_mode_colour = Some(s.to_string()),
        |o| {
            o.clock_mode_colour
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        CLOCK_MODE_COLOUR,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "clock-mode-style",
        |o, _, s| o.clock_mode_style = s.parse().ok(),
        |o| o.clock_mode_style.as_ref().map(|v| v.to_string()),
        CLOCK_MODE_STYLE,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    (
        "force-height",
        |o, _, s| o.force_height = s.parse().ok(),
        |o| o.force_height.as_ref().map(|v| v.to_string()),
        FORCE_HEIGHT,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    (
        "force-width",
        |o, _, s| o.force_width = s.parse().ok(),
        |o| o.force_width.as_ref().map(|v| v.to_string()),
        FORCE_WIDTH,
    ),
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    (
        "layout-history-limit",
        |o, _, s| o.layout_history_limit = s.parse().ok(),
        |o| o.layout_history_limit.as_ref().map(|v| v.to_string()),
        LAYOUT_HISTORY_LIMIT,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "main-pane-height",
        |o, _, s| o.main_pane_height = s.parse().ok(),
        |o| o.main_pane_height.as_ref().map(|v| v.to_string()),
        MAIN_PANE_HEIGHT,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "main-pane-width",
        |o, _, s| o.main_pane_width = s.parse().ok(),
        |o| o.main_pane_width.as_ref().map(|v| v.to_string()),
        MAIN_PANE_WIDTH,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "mode-attr",
        |o, _, s| o.mode_attr = s.parse().ok(),
        |o| o.mode_attr.as_ref().map(|v| v.to_string()),
        MODE_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "mode-bg",
        |o, _, s| o.mode_bg = s.parse().ok(),
        |o| o.mode_bg.as_ref().map(|v| v.to_string()),
        MODE_BG,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "mode-fg",
        |o, _, s| o.mode_fg = s.parse().ok(),
        |o| o.mode_fg.as_ref().map(|v| v.to_string()),
        MODE_FG,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "mode-keys",
        |o, _, s| o.mode_keys = s.parse().ok(),
        |o| o.mode_keys.as_ref().map(|v| v.to_string()),
        MODE_KEYS,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    (
        "mode-mouse",
        |o, _, s| o.mode_mouse = s.parse().ok(),
        |o| o.mode_mouse.as_ref().map(|v| v.to_string()),
        MODE_MOUSE,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "mode-style",
        |o, _, s| o.mode_style = Some(s.to_string()),
        |o| {
            o.mode_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        MODE_STYLE,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "monitor-activity",
        |o, _, s| o.monitor_activity = s.parse().ok(),
        |o| o.monitor_activity.as_ref().map(|v| v.to_string()),
        MONITOR_ACTIVITY,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    (
        "monitor-content",
        |o, _, s| o.monitor_content = s.parse().ok(),
        |o| o.monitor_content.as_ref().map(|v| v.to_string()),
        MONITOR_CONTENT,
    ),
    #[cfg(feature = "tmux_2_6")]
    (
        "monitor-bell",
        |o, _, s| o.monitor_bell = s.parse().ok(),
        |o| o.monitor_bell.as_ref().map(|v| v.to_string()),
        MONITOR_BELL,
    ),
    #[cfg(feature = "tmux_1_4")]
    (
        "monitor-silence",
        |o, _, s| o.monitor_silence = s.parse().ok(),
        |o| o.monitor_silence.as_ref().map(|v| v.to_string()),
        MONITOR_SILENCE,
    ),
    #[cfg(feature = "tmux_1_4")]
    (
        "other-pane-height",
        |o, _, s| o.other_pane_height = s.parse().ok(),
        |o| o.other_pane_height.as_ref().map(|v| v.to_string()),
        OTHER_PANE_HEIGHT,
    ),
    #[cfg(feature = "tmux_1_4")]
    (
        "other-pane-width",
        |o, _, s| o.other_pane_width = s.parse().ok(),
        |o| o.other_pane_width.as_ref().map(|v| v.to_string()),
        OTHER_PANE_WIDTH,
    ),
    #[cfg(feature = "tmux_2_0")]
    (
        "pane-active-border-style",
        |o, _, s| o.pane_active_border_style = Some(s.to_string()),
        |o| {
            o.pane_active_border_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        PANE_ACTIVE_BORDER_STYLE,
    ),
    #[cfg(feature = "tmux_1_6")]
    (
        "pane-base-index",
        |o, _, s| o.pane_base_index = s.parse().ok(),
        |o| o.pane_base_index.as_ref().map(|v| v.to_string()),
        PANE_BASE_INDEX,
    ),
    #[cfg(feature = "tmux_2_3")]
    (
        "pane-border-format",
        |o, _, s| o.pane_border_format = Some(s.to_string()),
        |o| {
            o.pane_border_format
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        PANE_BORDER_FORMAT,
    ),
    #[cfg(feature = "tmux_2_3")]
    (
        "pane-border-status",
        |o, _, s| o.pane_border_status = s.parse().ok(),
        |o| o.pane_border_status.as_ref().map(|v| v.to_string()),
        PANE_BORDER_STATUS,
    ),
    #[cfg(feature = "tmux_2_0")]
    (
        "pane-border-style",
        |o, _, s| o.pane_border_style = Some(s.to_string()),
        |o| {
            o.pane_border_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        PANE_BORDER_STYLE,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    (
        "remain-on-exit",
        |o, _, s| o.remain_on_exit = s.parse().ok(),
        |o| o.remain_on_exit.as_ref().map(|v| v.to_string()),
        REMAIN_ON_EXIT,
    ),
    #[cfg(feature = "tmux_1_0")]
    (
        "synchronize-panes",
        |o, _, s| o.synchronize_panes = s.parse().ok(),
        |o| o.synchronize_panes.as_ref().map(|v| v.to_string()),
        SYNCHRONIZE_PANES,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    (
        "utf8",
        |o, _, s| o.utf8 = s.parse().ok(),
        |o| o.utf8.as_ref().map(|v| v.to_string()),
        UTF8,
    ),
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    (
        "window-active-style",
        |o, _, s| o.window_active_style = Some(s.to_string()),
        |o| {
            o.window_active_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_ACTIVE_STYLE,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "window-status-bell-attr",
        |o, _, s| o.window_status_bell_attr = Some(s.to_string()),
        |o| {
            o.window_status_bell_attr
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_BELL_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "window-status-bell-bg",
        |o, _, s| o.window_status_bell_bg = Some(s.to_string()),
        |o| {
            o.window_status_bell_bg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_BELL_BG,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "window-status-bell-fg",
        |o, _, s| o.window_status_bell_fg = Some(s.to_string()),
        |o| {
            o.window_status_bell_fg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_BELL_FG,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "window-status-content-attr",
        |o, _, s| o.window_status_content_attr = Some(s.to_string()),
        |o| {
            o.window_status_content_attr
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CONTENT_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "window-status-content-bg",
        |o, _, s| o.window_status_content_bg = Some(s.to_string()),
        |o| {
            o.window_status_content_bg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CONTENT_BG,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "window-status-content-fg",
        |o, _, s| o.window_status_content_fg = Some(s.to_string()),
        |o| {
            o.window_status_content_fg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CONTENT_FG,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "window-status-activity-attr",
        |o, _, s| o.window_status_activity_attr = Some(s.to_string()),
        |o| {
            o.window_status_activity_attr
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_ACTIVITY_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "window-status-activity-bg",
        |o, _, s| o.window_status_activity_bg = Some(s.to_string()),
        |o| {
            o.window_status_activity_bg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_ACTIVITY_BG,
    ),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    (
        "window-status-activity-fg",
        |o, _, s| o.window_status_activity_fg = Some(s.to_string()),
        |o| {
            o.window_status_activity_fg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_ACTIVITY_FG,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "window-status-attr",
        |o, _, s| o.window_status_attr = Some(s.to_string()),
        |o| {
            o.window_status_attr
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "window-status-bg",
        |o, _, s| o.window_status_bg = Some(s.to_string()),
        |o| {
            o.window_status_bg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_BG,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "window-status-fg",
        |o, _, s| o.window_status_fg = Some(s.to_string()),
        |o| {
            o.window_status_fg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_FG,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "window-status-current-attr",
        |o, _, s| o.window_status_current_attr = Some(s.to_string()),
        |o| {
            o.window_status_current_attr
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CURRENT_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "window-status-current-bg",
        |o, _, s| o.window_status_current_bg = Some(s.to_string()),
        |o| {
            o.window_status_current_bg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CURRENT_BG,
    ),
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    (
        "window-status-current-fg",
        |o, _, s| o.window_status_current_fg = Some(s.to_string()),
        |o| {
            o.window_status_current_fg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CURRENT_FG,
    ),
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    (
        "window-status-alert-attr",
        |o, _, s| o.window_status_alert_attr = Some(s.to_string()),
        |o| {
            o.window_status_alert_attr
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_ALERT_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    (
        "window-status-alert-bg",
        |o, _, s| o.window_status_alert_bg = Some(s.to_string()),
        |o| {
            o.window_status_alert_bg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_ALERT_BG,
    ),
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    (
        "window-status-alert-fg",
        |o, _, s| o.window_status_alert_fg = Some(s.to_string()),
        |o| {
            o.window_status_alert_fg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_ALERT_FG,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "window-status-activity-style",
        |o, _, s| o.window_status_activity_style = Some(s.to_string()),
        |o| {
            o.window_status_activity_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_ACTIVITY_STYLE,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "window-status-bell-style",
        |o, _, s| o.window_status_bell_style = Some(s.to_string()),
        |o| {
            o.window_status_bell_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_BELL_STYLE,
    ),
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    (
        "window-status-content-style",
        |o, _, s| o.window_status_content_style = Some(s.to_string()),
        |o| {
            o.window_status_content_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CONTENT_STYLE,
    ),
    #[cfg(feature = "tmux_1_2")]
    (
        "window-status-current-format",
        |o, _, s| o.window_status_current_format = Some(s.to_string()),
        |o| {
            o.window_status_current_format
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CURRENT_FORMAT,
    ),
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    (
        "window-status-last-attr",
        |o, _, s| o.window_status_last_attr = Some(s.to_string()),
        |o| {
            o.window_status_last_attr
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_LAST_ATTR,
    ),
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    (
        "window-status-last-bg",
        |o, _, s| o.window_status_last_bg = Some(s.to_string()),
        |o| {
            o.window_status_last_bg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_LAST_BG,
    ),
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    (
        "window-status-last-fg",
        |o, _, s| o.window_status_last_fg = Some(s.to_string()),
        |o| {
            o.window_status_last_fg
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_LAST_FG,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "window-status-current-style",
        |o, _, s| o.window_status_current_style = Some(s.to_string()),
        |o| {
            o.window_status_current_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CURRENT_STYLE,
    ),
    #[cfg(feature = "tmux_1_2")]
    (
        "window-status-format",
        |o, _, s| o.window_status_format = Some(s.to_string()),
        |o| {
            o.window_status_format
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_FORMAT,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "window-status-last-style",
        |o, _, s| o.window_status_last_style = Some(s.to_string()),
        |o| {
            o.window_status_last_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_LAST_STYLE,
    ),
    #[cfg(feature = "tmux_1_7")]
    (
        "window-status-separator",
        |o, _, s| o.window_status_separator = Some(s.to_string()),
        |o| {
            o.window_status_separator
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_SEPARATOR,
    ),
    #[cfg(feature = "tmux_1_9")]
    (
        "window-status-style",
        |o, _, s| o.window_status_style = Some(s.to_string()),
        |o| {
            o.window_status_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_STYLE,
    ),
    #[cfg(feature = "tmux_2_9")]
    (
        "window-size",
        |o, _, s| o.window_size = s.parse().ok(),
        |o| o.window_size.as_ref().map(|v| v.to_string()),
        WINDOW_SIZE,
    ),
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    (
        "word-separators",
        |o, _, s| o.word_separators = s.parse().ok(),
        |o| o.word_separators.as_ref().map(|v| v.to_string()),
        WORD_SEPARATORS,
    ),
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    (
        "window-style",
        |o, _, s| o.window_style = s.parse().ok(),
        |o| o.window_style.as_ref().map(|v| v.to_string()),
        WINDOW_STYLE,
    ),
    #[cfg(feature = "tmux_1_7")]
    (
        "wrap-search",
        |o, _, s| o.wrap_search = s.parse().ok(),
        |o| o.wrap_search.as_ref().map(|v| v.to_string()),
        WRAP_SEARCH,
    ),
    #[cfg(feature = "tmux_1_2")]
    (
        "xterm-keys",
        |o, _, s| o.xterm_keys = s.parse().ok(),
        |o| o.xterm_keys.as_ref().map(|v| v.to_string()),
        XTERM_KEYS,
    ),
];

// TODO: check types
// 31 Available window options are:
#[derive(Default, PartialEq, Clone, Debug)]
pub struct WindowOptions {
    //aggressive-resize [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub aggressive_resize: Option<Switch>,
    //allow-rename [on | off]
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    pub allow_rename: Option<Switch>,
    //alternate-screen [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub alternate_screen: Option<Switch>,
    //automatic-rename [on | off]
    #[cfg(feature = "tmux_1_0")] // 0.8
    pub automatic_rename: Option<Switch>,
    //automatic-rename-format format
    #[cfg(feature = "tmux_1_9")]
    pub automatic_rename_format: Option<String>,
    //c0-change-interval interval
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_interval: Option<usize>,
    //c0-change-trigger trigger
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_trigger: Option<usize>,
    //clock-mode-colour colour
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_colour: Option<String>,
    //clock-mode-style [12 | 24]
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_style: Option<ClockModeStyle>,
    //force-height height
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_height: Option<usize>,
    //force-width width
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_width: Option<usize>,
    //layout-history-limit limit
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    pub layout_history_limit: Option<usize>,
    //main-pane-height height
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_height: Option<usize>,
    //main-pane-width width
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_width: Option<usize>,
    //mode-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_attr: Option<String>,
    // mode-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_bg: Option<String>,
    // mode-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_fg: Option<String>,
    //mode-keys [vi | emacs]
    #[cfg(feature = "tmux_1_0")]
    pub mode_keys: Option<StatusKeys>,
    //mode-mouse [on | off]
    //tmux 1.6: mode-mouse [on | off | copy-mode]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub mode_mouse: Option<Switch>,
    //mode-style style
    #[cfg(feature = "tmux_1_9")]
    pub mode_style: Option<String>,
    //monitor-activity [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub monitor_activity: Option<Switch>,
    //monitor-content match-string
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub monitor_content: Option<String>,
    //monitor-bell [on | off]
    #[cfg(feature = "tmux_2_6")]
    pub monitor_bell: Option<Switch>,
    //monitor-silence [interval]
    #[cfg(feature = "tmux_1_4")]
    pub monitor_silence: Option<usize>,
    //other-pane-height height
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_height: Option<usize>,
    //other-pane-width width
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_width: Option<usize>,
    //pane-active-border-style style
    #[cfg(feature = "tmux_2_0")]
    pub pane_active_border_style: Option<String>,
    //pane-base-index index
    #[cfg(feature = "tmux_1_6")]
    pub pane_base_index: Option<usize>,
    //pane-border-format format
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_format: Option<String>,
    //pane-border-status [off | top | bottom]
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_status: Option<PaneBorderStatus>,
    //pane-border-style style
    #[cfg(feature = "tmux_2_0")]
    pub pane_border_style: Option<String>,
    //remain-on-exit [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    pub remain_on_exit: Option<Switch>,
    //synchronize-panes [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub synchronize_panes: Option<Switch>,
    //utf8 [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub utf8: Option<Switch>,
    //window-active-style style
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_active_style: Option<String>,
    //window-status-bell-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_attr: Option<String>,
    //window-status-bell-bg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_bg: Option<String>,
    //window-status-bell-fg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_fg: Option<String>,
    //window-status-content-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_attr: Option<String>,
    //window-status-content-bg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_bg: Option<String>,
    //window-status-content-fg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_fg: Option<String>,
    //window-status-activity-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_attr: Option<String>,
    //window-status-activity-bg attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_bg: Option<String>,
    //window-status-activity-fg attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_fg: Option<String>,
    //window-status-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_attr: Option<String>,
    //window-status-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_bg: Option<String>,
    //window-status-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_fg: Option<String>,
    //window-status-current-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_attr: Option<String>,
    //window-status-current-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_bg: Option<String>,
    //window-status-current-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_fg: Option<String>,
    //window-status-alert-attr attributes
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_attr: Option<String>,
    //window-status-alert-bg colour
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_bg: Option<String>,
    //window-status-alert-fg colour
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_fg: Option<String>,
    //window-status-activity-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_activity_style: Option<String>,
    //window-status-bell-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_bell_style: Option<String>,
    //window-status-content-style style
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub window_status_content_style: Option<String>,
    //window-status-current-format string
    #[cfg(feature = "tmux_1_2")]
    pub window_status_current_format: Option<String>,
    //window-status-last-attr attributes
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_attr: Option<String>,
    //window-status-last-bg colour
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_bg: Option<String>,
    //window-status-last-fg colour
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_fg: Option<String>,
    //window-status-current-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_current_style: Option<String>,
    //window-status-format string
    #[cfg(feature = "tmux_1_2")]
    pub window_status_format: Option<String>,
    //window-status-last-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_last_style: Option<String>,
    //window-status-separator string
    #[cfg(feature = "tmux_1_7")]
    pub window_status_separator: Option<String>,
    //window-status-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_style: Option<String>,
    //window-size largest | smallest | manual | latest
    #[cfg(feature = "tmux_2_9")]
    pub window_size: Option<WindowSize>,
    //word-separators string
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    pub word_separators: Option<String>,
    //window-style style
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_style: Option<String>,
    //wrap-search [on | off]
    #[cfg(feature = "tmux_1_7")]
    pub wrap_search: Option<Switch>,
    //xterm-keys [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub xterm_keys: Option<Switch>,
    // XXX: user options?
    //pub user_options: Option<HashMap<String, String>>
}

impl WindowOptions {
    pub fn get_all() -> Result<Self, Error> {
        ShowOptions::new()
            .global()
            .window()
            .output()?
            .to_string()
            .parse()
    }

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    #[cfg(feature = "tmux_1_7")]
    pub fn get(bitflags: u128) -> Result<Self, Error> {
        let selected_option = WINDOW_OPTIONS
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

    // allows selective set by bitmask
    pub fn set(&self, bitflags: u128) -> Result<(), Error> {
        for selected_option in WINDOW_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
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
}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?
impl FromStr for WindowOptions {
    type Err = Error;

    fn from_str(options: &str) -> Result<Self, Self::Err> {
        let mut window_options: WindowOptions = Default::default();
        let mut v: Vec<&str>;
        let mut arr: Vec<&str>;
        for option in options.lines() {
            v = option.trim().splitn(2, ' ').collect();
            arr = v[0].split(|c| c == '[' || c == ']').collect();
            for window_var in WINDOW_OPTIONS.iter() {
                if window_var.0 == arr[0] {
                    window_var.1(
                        &mut window_options,
                        arr.get(1).and_then(|i| i.parse::<usize>().ok()),
                        v.get(1).unwrap_or(&""),
                    )
                }
            }
        }
        Ok(window_options)
    }
}

impl fmt::Display for WindowOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // pane option
        for var in WINDOW_OPTIONS.iter() {
            // if is set some - extract
            if let Some(ref v) = var.2(self) {
                writeln!(f, "{} {}", var.0, v)?;
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct WindowOptionsBuilder<'a> {
    #[cfg(feature = "tmux_1_0")]
    pub aggressive_resize: Option<Switch>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    pub allow_rename: Option<Switch>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub alternate_screen: Option<Switch>,
    #[cfg(feature = "tmux_1_0")] // 0.8
    pub automatic_rename: Option<Switch>,
    #[cfg(feature = "tmux_1_9")]
    pub automatic_rename_format: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_interval: Option<usize>,
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_trigger: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_colour: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_style: Option<ClockModeStyle>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_height: Option<usize>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_width: Option<usize>,
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    pub layout_history_limit: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_height: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_width: Option<usize>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_fg: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub mode_keys: Option<StatusKeys>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub mode_mouse: Option<Switch>,
    #[cfg(feature = "tmux_1_9")]
    pub mode_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub monitor_activity: Option<Switch>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub monitor_content: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub monitor_bell: Option<Switch>,
    #[cfg(feature = "tmux_1_4")]
    pub monitor_silence: Option<usize>,
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_height: Option<usize>,
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_width: Option<usize>,
    #[cfg(feature = "tmux_2_0")]
    pub pane_active_border_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_6")]
    pub pane_base_index: Option<usize>,
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_format: Option<&'a str>,
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_status: Option<PaneBorderStatus>,
    #[cfg(feature = "tmux_2_0")]
    pub pane_border_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    pub remain_on_exit: Option<Switch>,
    #[cfg(feature = "tmux_1_0")]
    pub synchronize_panes: Option<Switch>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub utf8: Option<Switch>,
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_active_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_fg: Option<&'a str>,
    #[cfg(feature = "tmux_1_9")]
    pub window_status_activity_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_9")]
    pub window_status_bell_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub window_status_content_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_2")]
    pub window_status_current_format: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_fg: Option<&'a str>,
    #[cfg(feature = "tmux_1_9")]
    pub window_status_current_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_2")]
    pub window_status_format: Option<&'a str>,
    #[cfg(feature = "tmux_1_9")]
    pub window_status_last_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub window_status_separator: Option<&'a str>,
    #[cfg(feature = "tmux_1_9")]
    pub window_status_style: Option<&'a str>,
    #[cfg(feature = "tmux_2_9")]
    pub window_size: Option<WindowSize>,
    #[cfg(feature = "tmux_1_2")]
    pub word_separators: Option<&'a str>,
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub wrap_search: Option<Switch>,
    #[cfg(feature = "tmux_1_0")]
    pub xterm_keys: Option<Switch>,
}

impl<'a> WindowOptionsBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn aggressive_resize(&mut self, aggressive_resize: Switch) -> &mut Self {
        self.aggressive_resize = Some(aggressive_resize);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn automatic_rename(&mut self, automatic_rename: Switch) -> &mut Self {
        self.automatic_rename = Some(automatic_rename);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn automatic_rename_format(&mut self, automatic_rename_format: &'a str) -> &mut Self {
        self.automatic_rename_format = Some(automatic_rename_format);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn clock_mode_colour(&mut self, clock_mode_colour: &'a str) -> &mut Self {
        self.clock_mode_colour = Some(clock_mode_colour);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn clock_mode_style(&mut self, clock_mode_style: ClockModeStyle) -> &mut Self {
        self.clock_mode_style = Some(clock_mode_style);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn main_pane_height(&mut self, main_pane_height: usize) -> &mut Self {
        self.main_pane_height = Some(main_pane_height);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn main_pane_width(&mut self, main_pane_width: usize) -> &mut Self {
        self.main_pane_width = Some(main_pane_width);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn mode_keys(&mut self, mode_keys: StatusKeys) -> &mut Self {
        self.mode_keys = Some(mode_keys);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn mode_style(&mut self, mode_style: &'a str) -> &mut Self {
        self.mode_style = Some(mode_style);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn monitor_activity(&mut self, monitor_activity: Switch) -> &mut Self {
        self.monitor_activity = Some(monitor_activity);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn monitor_bell(&mut self, monitor_bell: Switch) -> &mut Self {
        self.monitor_bell = Some(monitor_bell);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn monitor_silence(&mut self, monitor_silence: usize) -> &mut Self {
        self.monitor_silence = Some(monitor_silence);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn other_pane_height(&mut self, other_pane_height: usize) -> &mut Self {
        self.other_pane_height = Some(other_pane_height);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn other_pane_width(&mut self, other_pane_width: usize) -> &mut Self {
        self.other_pane_width = Some(other_pane_width);
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn pane_active_border_style(&mut self, pane_active_border_style: &'a str) -> &mut Self {
        self.pane_active_border_style = Some(pane_active_border_style);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn pane_base_index(&mut self, pane_base_index: usize) -> &mut Self {
        self.pane_base_index = Some(pane_base_index);
        self
    }

    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_format(&mut self, pane_border_format: &'a str) -> &mut Self {
        self.pane_border_format = Some(pane_border_format);
        self
    }

    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_status(&mut self, pane_border_status: PaneBorderStatus) -> &mut Self {
        self.pane_border_status = Some(pane_border_status);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn synchronize_panes(&mut self, synchronize_panes: Switch) -> &mut Self {
        self.synchronize_panes = Some(synchronize_panes);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_activity_style(
        &mut self,
        window_status_activity_style: &'a str,
    ) -> &mut Self {
        self.window_status_activity_style = Some(window_status_activity_style);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_bell_style(&mut self, window_status_bell_style: &'a str) -> &mut Self {
        self.window_status_bell_style = Some(window_status_bell_style);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_current_format(
        &mut self,
        window_status_current_format: &'a str,
    ) -> &mut Self {
        self.window_status_current_format = Some(window_status_current_format);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_format(&mut self, window_status_format: &'a str) -> &mut Self {
        self.window_status_format = Some(window_status_format);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_last_style(&mut self, window_status_last_style: &'a str) -> &mut Self {
        self.window_status_last_style = Some(window_status_last_style);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn window_status_separator(&mut self, window_status_separator: &'a str) -> &mut Self {
        self.window_status_separator = Some(window_status_separator);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_style(&mut self, window_status_style: &'a str) -> &mut Self {
        self.window_status_style = Some(window_status_style);
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn window_size(&mut self, window_size: WindowSize) -> &mut Self {
        self.window_size = Some(window_size);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn wrap_search(&mut self, wrap_search: Switch) -> &mut Self {
        self.wrap_search = Some(wrap_search);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn xterm_keys(&mut self, xterm_keys: Switch) -> &mut Self {
        self.xterm_keys = Some(xterm_keys);
        self
    }

    // XXX: clone rly needed?
    pub fn build(&self) -> WindowOptions {
        WindowOptions {
            #[cfg(feature = "tmux_1_0")]
            aggressive_resize: self.aggressive_resize.clone(),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
            allow_rename: self.allow_rename.clone(),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
            alternate_screen: self.alternate_screen.clone(),
            #[cfg(feature = "tmux_1_0")]
            automatic_rename: self.automatic_rename.clone(),
            #[cfg(feature = "tmux_1_9")]
            automatic_rename_format: self.automatic_rename_format.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
            c0_change_interval: self.c0_change_interval,
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
            c0_change_trigger: self.c0_change_trigger,
            #[cfg(feature = "tmux_1_2")]
            clock_mode_colour: self.clock_mode_colour.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_2")]
            clock_mode_style: self.clock_mode_style.clone(),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
            force_height: self.force_height,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
            force_width: self.force_width,
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
            layout_history_limit: self.layout_history_limit,
            #[cfg(feature = "tmux_1_2")]
            main_pane_height: self.main_pane_height,
            #[cfg(feature = "tmux_1_2")]
            main_pane_width: self.main_pane_width,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            mode_attr: self.mode_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            mode_bg: self.mode_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            mode_fg: self.mode_fg.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_2")]
            mode_keys: self.mode_keys.clone(),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
            mode_mouse: self.mode_mouse.clone(),
            #[cfg(feature = "tmux_1_9")]
            mode_style: self.mode_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_0")]
            monitor_activity: self.monitor_activity.clone(),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
            monitor_content: self.monitor_content.map(|s| s.to_string()),
            #[cfg(feature = "tmux_2_6")]
            monitor_bell: self.monitor_bell.clone(),
            #[cfg(feature = "tmux_1_4")]
            monitor_silence: self.monitor_silence,
            #[cfg(feature = "tmux_1_4")]
            other_pane_height: self.other_pane_height,
            #[cfg(feature = "tmux_1_4")]
            other_pane_width: self.other_pane_width,
            #[cfg(feature = "tmux_2_0")]
            pane_active_border_style: self.pane_active_border_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_6")]
            pane_base_index: self.pane_base_index,
            #[cfg(feature = "tmux_2_3")]
            pane_border_format: self.pane_border_format.map(|s| s.to_string()),
            #[cfg(feature = "tmux_2_3")]
            pane_border_status: self.pane_border_status.clone(),
            #[cfg(feature = "tmux_2_0")]
            pane_border_style: self.pane_border_style.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
            remain_on_exit: self.remain_on_exit.clone(),
            #[cfg(feature = "tmux_1_2")]
            synchronize_panes: self.synchronize_panes.clone(),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
            utf8: self.utf8.clone(),
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
            window_active_style: self.window_active_style.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_bell_attr: self.window_status_bell_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_bell_bg: self.window_status_bell_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_bell_fg: self.window_status_bell_fg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_content_attr: self.window_status_content_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_content_bg: self.window_status_content_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_content_fg: self.window_status_content_fg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_activity_attr: self.window_status_activity_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_activity_bg: self.window_status_activity_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_activity_fg: self.window_status_activity_fg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_attr: self.window_status_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_bg: self.window_status_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_fg: self.window_status_fg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_current_attr: self.window_status_current_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_current_bg: self.window_status_current_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_current_fg: self.window_status_current_fg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
            window_status_alert_attr: self.window_status_alert_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
            window_status_alert_bg: self.window_status_alert_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
            window_status_alert_fg: self.window_status_alert_fg.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_9")]
            window_status_activity_style: self.window_status_activity_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_9")]
            window_status_bell_style: self.window_status_bell_style.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            window_status_content_style: self.window_status_content_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_2")]
            window_status_current_format: self.window_status_current_format.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
            window_status_last_attr: self.window_status_last_attr.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
            window_status_last_bg: self.window_status_last_bg.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
            window_status_last_fg: self.window_status_last_fg.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_9")]
            window_status_current_style: self.window_status_current_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_2")]
            window_status_format: self.window_status_format.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_9")]
            window_status_last_style: self.window_status_last_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_7")]
            window_status_separator: self.window_status_separator.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_9")]
            window_status_style: self.window_status_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_2_9")]
            window_size: self.window_size.clone(),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
            word_separators: self.word_separators.map(|s| s.to_string()),
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
            window_style: self.window_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_1_7")]
            wrap_search: self.wrap_search.clone(),
            #[cfg(feature = "tmux_1_2")]
            xterm_keys: self.xterm_keys.clone(),
        }
    }
}
