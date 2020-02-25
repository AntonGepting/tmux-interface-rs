use crate::common::StatusKeys;
use crate::{Error, SetOptionBuilder, ShowOptionsBuilder, Switch, TargetPane, TmuxInterface};
use std::fmt;
use std::str::FromStr;

//clock-mode-style [12 | 24]
#[derive(PartialEq, Clone, Debug)]
pub enum ClockModeStyle {
    _12,
    _24,
}

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
pub enum PaneBorderStatus {
    Off,
    Top,
    Bottom,
}

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
pub enum WindowSize {
    Largest,
    Smallest,
    Manual,
    Latest,
}

impl FromStr for WindowSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "largest" => Ok(Self::Largest),
            "smallest" => Ok(Self::Smallest),
            "manual" => Ok(Self::Manual),
            "latest" => Ok(Self::Latest),
            _ => Err(Error::ParseWindowSize),
        }
    }
}

impl fmt::Display for WindowSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Largest => write!(f, "largest"),
            Self::Smallest => write!(f, "smallest"),
            Self::Manual => write!(f, "manual"),
            Self::Latest => write!(f, "latest"),
        }
    }
}

// NOTE: u32 mb not enough!
pub const AGGRESIVE_RESIZE: usize = 1 << 0;
pub const AUTOMATIC_RENAME: usize = 1 << 1;
pub const AUTOMATIC_RENAME_FORMAT: usize = 1 << 2;
pub const CLOCK_MODE_COLOUR: usize = 1 << 3;
pub const CLOCK_MODE_STYLE: usize = 1 << 4;
pub const MAIN_PANE_HEIGHT: usize = 1 << 5;
pub const MAIN_PANE_WIDTH: usize = 1 << 5;
pub const MODE_KEYS: usize = 1 << 7;
pub const MODE_STYLE: usize = 1 << 8;
pub const MONITOR_ACTIVITY: usize = 1 << 9;
pub const MONITOR_BELL: usize = 1 << 10;
pub const MONITOR_SILENCE: usize = 1 << 11;
pub const OTHER_PANE_HEIGHT: usize = 1 << 12;
pub const OTHER_PANE_WIDTH: usize = 1 << 13;
pub const PANE_ACTIVE_BORDER_STYLE: usize = 1 << 14;
pub const PANE_BASE_INDEX: usize = 1 << 15;
pub const PANE_BORDER_FORMAT: usize = 1 << 16;
pub const PANE_BORDER_STATUS: usize = 1 << 17;
pub const PANE_BORDER_STYLE: usize = 1 << 18;
pub const SYNCHRONIZE_PANES: usize = 1 << 19;
pub const WINDOW_STATUS_ACTIVITY_STYLE: usize = 1 << 20;
pub const WINDOW_STATUS_BELL_STYLE: usize = 1 << 21;
pub const WINDOW_STATUS_CURRENT_FORMAT: usize = 1 << 22;
pub const WINDOW_STATUS_CURRENT_STYLE: usize = 1 << 23;
pub const WINDOW_STATUS_FORMAT: usize = 1 << 24;
pub const WINDOW_STATUS_LAST_STYLE: usize = 1 << 25;
pub const WINDOW_STATUS_SEPARATOR: usize = 1 << 26;
pub const WINDOW_STATUS_STYLE: usize = 1 << 27;
pub const WINDOW_SIZE: usize = 1 << 28;
pub const WRAP_SEARCH: usize = 1 << 29;
pub const XTERM_KEYS: usize = 1 << 30;

pub const WINDOW_OPTIONS_NONE: usize = 0;
////pub const SERVER_OPTIONS_DEFAULT: usize = ;
pub const WINDOW_OPTIONS_ALL: usize = AGGRESIVE_RESIZE
    | AUTOMATIC_RENAME
    | AUTOMATIC_RENAME_FORMAT
    | CLOCK_MODE_COLOUR
    | CLOCK_MODE_STYLE
    | MAIN_PANE_HEIGHT
    | MAIN_PANE_WIDTH
    | MODE_KEYS
    | MODE_STYLE
    | MONITOR_ACTIVITY
    | MONITOR_BELL
    | MONITOR_SILENCE
    | OTHER_PANE_HEIGHT
    | OTHER_PANE_WIDTH
    | PANE_ACTIVE_BORDER_STYLE
    | PANE_BASE_INDEX
    | PANE_BORDER_FORMAT
    | PANE_BORDER_STATUS
    | PANE_BORDER_STYLE
    | SYNCHRONIZE_PANES
    | WINDOW_STATUS_ACTIVITY_STYLE
    | WINDOW_STATUS_BELL_STYLE
    | WINDOW_STATUS_CURRENT_FORMAT
    | WINDOW_STATUS_CURRENT_STYLE
    | WINDOW_STATUS_FORMAT
    | WINDOW_STATUS_LAST_STYLE
    | WINDOW_STATUS_SEPARATOR
    | WINDOW_STATUS_STYLE
    | WINDOW_SIZE
    | WRAP_SEARCH
    | XTERM_KEYS;

pub const WINDOW_OPTIONS_NUM: usize = 31;
pub const WINDOW_OPTIONS: [(
    &str,
    fn(o: &mut WindowOptions, s: &str),
    fn(o: &WindowOptions) -> Option<String>,
    usize,
); WINDOW_OPTIONS_NUM] = [
    (
        "aggressive-resize",
        |o, s| o.aggressive_resize = s.parse().ok(),
        |o| o.aggressive_resize.as_ref().map(|v| v.to_string()),
        AGGRESIVE_RESIZE,
    ),
    (
        "automatic-rename",
        |o, s| o.automatic_rename = s.parse().ok(),
        |o| o.automatic_rename.as_ref().map(|v| v.to_string()),
        AUTOMATIC_RENAME,
    ),
    (
        "automatic-rename-format",
        |o, s| o.automatic_rename_format = Some(s.to_string()),
        |o| {
            o.automatic_rename_format
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        AUTOMATIC_RENAME_FORMAT,
    ),
    (
        "clock-mode-colour",
        |o, s| o.clock_mode_colour = Some(s.to_string()),
        |o| {
            o.clock_mode_colour
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        CLOCK_MODE_COLOUR,
    ),
    (
        "clock-mode-style",
        |o, s| o.clock_mode_style = s.parse().ok(),
        |o| o.clock_mode_style.as_ref().map(|v| v.to_string()),
        CLOCK_MODE_STYLE,
    ),
    (
        "main-pane-height",
        |o, s| o.main_pane_height = s.parse().ok(),
        |o| o.main_pane_height.as_ref().map(|v| v.to_string()),
        MAIN_PANE_HEIGHT,
    ),
    (
        "main-pane-width",
        |o, s| o.main_pane_width = s.parse().ok(),
        |o| o.main_pane_width.as_ref().map(|v| v.to_string()),
        MAIN_PANE_WIDTH,
    ),
    (
        "mode-keys",
        |o, s| o.mode_keys = s.parse().ok(),
        |o| o.mode_keys.as_ref().map(|v| v.to_string()),
        MODE_KEYS,
    ),
    (
        "mode-style",
        |o, s| o.mode_style = Some(s.to_string()),
        |o| {
            o.mode_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        MODE_STYLE,
    ),
    (
        "monitor-activity",
        |o, s| o.monitor_activity = s.parse().ok(),
        |o| o.monitor_activity.as_ref().map(|v| v.to_string()),
        MONITOR_ACTIVITY,
    ),
    (
        "monitor-bell",
        |o, s| o.monitor_bell = s.parse().ok(),
        |o| o.monitor_bell.as_ref().map(|v| v.to_string()),
        MONITOR_BELL,
    ),
    (
        "monitor-silence",
        |o, s| o.monitor_silence = s.parse().ok(),
        |o| o.monitor_silence.as_ref().map(|v| v.to_string()),
        MONITOR_SILENCE,
    ),
    (
        "other-pane-height",
        |o, s| o.other_pane_height = s.parse().ok(),
        |o| o.other_pane_height.as_ref().map(|v| v.to_string()),
        OTHER_PANE_HEIGHT,
    ),
    (
        "other-pane-width",
        |o, s| o.other_pane_width = s.parse().ok(),
        |o| o.other_pane_width.as_ref().map(|v| v.to_string()),
        OTHER_PANE_WIDTH,
    ),
    (
        "pane-active-border-style",
        |o, s| o.pane_active_border_style = Some(s.to_string()),
        |o| {
            o.pane_active_border_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        PANE_ACTIVE_BORDER_STYLE,
    ),
    (
        "pane-base-index",
        |o, s| o.pane_base_index = s.parse().ok(),
        |o| o.pane_base_index.as_ref().map(|v| v.to_string()),
        PANE_BASE_INDEX,
    ),
    (
        "pane-border-format",
        |o, s| o.pane_border_format = Some(s.to_string()),
        |o| {
            o.pane_border_format
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        PANE_BORDER_FORMAT,
    ),
    (
        "pane-border-status",
        |o, s| o.pane_border_status = s.parse().ok(),
        |o| o.pane_border_status.as_ref().map(|v| v.to_string()),
        PANE_BORDER_STATUS,
    ),
    (
        "pane-border-style",
        |o, s| o.pane_border_style = Some(s.to_string()),
        |o| {
            o.pane_border_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        PANE_BORDER_STYLE,
    ),
    (
        "synchronize-panes",
        |o, s| o.synchronize_panes = s.parse().ok(),
        |o| o.synchronize_panes.as_ref().map(|v| v.to_string()),
        SYNCHRONIZE_PANES,
    ),
    (
        "window-status-activity-style",
        |o, s| o.window_status_activity_style = Some(s.to_string()),
        |o| {
            o.window_status_activity_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_ACTIVITY_STYLE,
    ),
    (
        "window-status-bell-style",
        |o, s| o.window_status_bell_style = Some(s.to_string()),
        |o| {
            o.window_status_bell_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_BELL_STYLE,
    ),
    (
        "window-status-current-format",
        |o, s| o.window_status_current_format = Some(s.to_string()),
        |o| {
            o.window_status_current_format
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CURRENT_FORMAT,
    ),
    (
        "window-status-current-style",
        |o, s| o.window_status_current_style = Some(s.to_string()),
        |o| {
            o.window_status_current_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_CURRENT_STYLE,
    ),
    (
        "window-status-format",
        |o, s| o.window_status_format = Some(s.to_string()),
        |o| {
            o.window_status_format
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_FORMAT,
    ),
    (
        "window-status-last-style",
        |o, s| o.window_status_last_style = Some(s.to_string()),
        |o| {
            o.window_status_last_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_LAST_STYLE,
    ),
    (
        "window-status-separator",
        |o, s| o.window_status_separator = Some(s.to_string()),
        |o| {
            o.window_status_separator
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_SEPARATOR,
    ),
    (
        "window-status-style",
        |o, s| o.window_status_style = Some(s.to_string()),
        |o| {
            o.window_status_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STATUS_STYLE,
    ),
    (
        "window-size",
        |o, s| o.window_size = s.parse().ok(),
        |o| o.window_size.as_ref().map(|v| v.to_string()),
        WINDOW_SIZE,
    ),
    (
        "wrap-search",
        |o, s| o.wrap_search = s.parse().ok(),
        |o| o.wrap_search.as_ref().map(|v| v.to_string()),
        WRAP_SEARCH,
    ),
    (
        "xterm-keys",
        |o, s| o.xterm_keys = s.parse().ok(),
        |o| o.xterm_keys.as_ref().map(|v| v.to_string()),
        XTERM_KEYS,
    ),
];

// TODO: check types
// 31 Available window options are:
#[derive(Default, PartialEq, Clone, Debug)]
pub struct WindowOptions {
    //aggressive-resize [on | off]
    pub aggressive_resize: Option<Switch>,
    //automatic-rename [on | off]
    pub automatic_rename: Option<Switch>,
    //automatic-rename-format format
    pub automatic_rename_format: Option<String>,
    //clock-mode-colour colour
    pub clock_mode_colour: Option<String>,
    //clock-mode-style [12 | 24]
    pub clock_mode_style: Option<ClockModeStyle>,
    //main-pane-height height
    pub main_pane_height: Option<usize>,
    //main-pane-width width
    pub main_pane_width: Option<usize>,
    //mode-keys [vi | emacs]
    pub mode_keys: Option<StatusKeys>,
    //mode-style style
    pub mode_style: Option<String>,
    //monitor-activity [on | off]
    pub monitor_activity: Option<Switch>,
    //monitor-bell [on | off]
    pub monitor_bell: Option<Switch>,
    //monitor-silence [interval]
    pub monitor_silence: Option<usize>,
    //other-pane-height height
    pub other_pane_height: Option<usize>,
    //other-pane-width width
    pub other_pane_width: Option<usize>,
    //pane-active-border-style style
    pub pane_active_border_style: Option<String>,
    //pane-base-index index
    pub pane_base_index: Option<usize>,
    //pane-border-format format
    pub pane_border_format: Option<String>,
    //pane-border-status [off | top | bottom]
    pub pane_border_status: Option<PaneBorderStatus>,
    //pane-border-style style
    pub pane_border_style: Option<String>,
    //synchronize-panes [on | off]
    pub synchronize_panes: Option<Switch>,
    //window-status-activity-style style
    pub window_status_activity_style: Option<String>,
    //window-status-bell-style style
    pub window_status_bell_style: Option<String>,
    //window-status-current-format string
    pub window_status_current_format: Option<String>,
    //window-status-current-style style
    pub window_status_current_style: Option<String>,
    //window-status-format string
    pub window_status_format: Option<String>,
    //window-status-last-style style
    pub window_status_last_style: Option<String>,
    //window-status-separator string
    pub window_status_separator: Option<String>,
    //window-status-style style
    pub window_status_style: Option<String>,
    //window-size largest | smallest | manual | latest
    pub window_size: Option<WindowSize>,
    //wrap-search [on | off]
    pub wrap_search: Option<Switch>,
    //xterm-keys [on | off]
    pub xterm_keys: Option<Switch>,
}

impl WindowOptions {
    pub fn get_all() -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let show_options = ShowOptionsBuilder::<TargetPane>::new()
            .global_options()
            .window()
            .build();
        let s = tmux.show_options(Some(&show_options))?;
        s.parse()
    }

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    pub fn get(bitflags: usize) -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let selected_option = WINDOW_OPTIONS
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
        for selected_option in WINDOW_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
            if let Some(selected_value) = selected_option.2(&self) {
                let set_option = SetOptionBuilder::<TargetPane>::new().server().build();
                tmux.set_option(Some(&set_option), selected_option.0, &selected_value)?;
            }
        }
        Ok(())
    }
}

impl FromStr for WindowOptions {
    type Err = Error;

    fn from_str(options: &str) -> Result<Self, Self::Err> {
        let mut window_options: WindowOptions = Default::default();
        let mut v: Vec<&str>;
        for option in options.lines() {
            v = option.trim().split(' ').collect();
            for window_var in WINDOW_OPTIONS.iter() {
                if window_var.0 == v[0] {
                    window_var.1(&mut window_options, v[1])
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
                write!(f, "{} {}\n", var.0, v)?;
            }
        }
        write!(f, "{}", "")
    }
}

#[derive(Default, Debug)]
pub struct WindowOptionsBuilder<'a> {
    pub aggressive_resize: Option<Switch>,
    pub automatic_rename: Option<Switch>,
    pub automatic_rename_format: Option<&'a str>,
    pub clock_mode_colour: Option<&'a str>,
    pub clock_mode_style: Option<ClockModeStyle>,
    pub main_pane_height: Option<usize>,
    pub main_pane_width: Option<usize>,
    pub mode_keys: Option<StatusKeys>,
    pub mode_style: Option<&'a str>,
    pub monitor_activity: Option<Switch>,
    pub monitor_bell: Option<Switch>,
    pub monitor_silence: Option<usize>,
    pub other_pane_height: Option<usize>,
    pub other_pane_width: Option<usize>,
    pub pane_active_border_style: Option<&'a str>,
    pub pane_base_index: Option<usize>,
    pub pane_border_format: Option<&'a str>,
    pub pane_border_status: Option<PaneBorderStatus>,
    pub pane_border_style: Option<&'a str>,
    pub synchronize_panes: Option<Switch>,
    pub window_status_activity_style: Option<&'a str>,
    pub window_status_bell_style: Option<&'a str>,
    pub window_status_current_format: Option<&'a str>,
    pub window_status_current_style: Option<&'a str>,
    pub window_status_format: Option<&'a str>,
    pub window_status_last_style: Option<&'a str>,
    pub window_status_separator: Option<&'a str>,
    pub window_status_style: Option<&'a str>,
    pub window_size: Option<WindowSize>,
    pub wrap_search: Option<Switch>,
    pub xterm_keys: Option<Switch>,
}

impl<'a> WindowOptionsBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn aggressive_resize(&mut self, aggressive_resize: Switch) -> &mut Self {
        self.aggressive_resize = Some(aggressive_resize);
        self
    }

    pub fn automatic_rename(&mut self, automatic_rename: Switch) -> &mut Self {
        self.automatic_rename = Some(automatic_rename);
        self
    }

    pub fn automatic_rename_format(&mut self, automatic_rename_format: &'a str) -> &mut Self {
        self.automatic_rename_format = Some(automatic_rename_format);
        self
    }

    pub fn clock_mode_colour(&mut self, clock_mode_colour: &'a str) -> &mut Self {
        self.clock_mode_colour = Some(clock_mode_colour);
        self
    }

    pub fn clock_mode_style(&mut self, clock_mode_style: ClockModeStyle) -> &mut Self {
        self.clock_mode_style = Some(clock_mode_style);
        self
    }

    pub fn main_pane_height(&mut self, main_pane_height: usize) -> &mut Self {
        self.main_pane_height = Some(main_pane_height);
        self
    }

    pub fn main_pane_width(&mut self, main_pane_width: usize) -> &mut Self {
        self.main_pane_width = Some(main_pane_width);
        self
    }

    pub fn mode_keys(&mut self, mode_keys: StatusKeys) -> &mut Self {
        self.mode_keys = Some(mode_keys);
        self
    }

    pub fn mode_style(&mut self, mode_style: &'a str) -> &mut Self {
        self.mode_style = Some(mode_style);
        self
    }

    pub fn monitor_activity(&mut self, monitor_activity: Switch) -> &mut Self {
        self.monitor_activity = Some(monitor_activity);
        self
    }

    pub fn monitor_bell(&mut self, monitor_bell: Switch) -> &mut Self {
        self.monitor_bell = Some(monitor_bell);
        self
    }

    pub fn monitor_silence(&mut self, monitor_silence: usize) -> &mut Self {
        self.monitor_silence = Some(monitor_silence);
        self
    }

    pub fn other_pane_height(&mut self, other_pane_height: usize) -> &mut Self {
        self.other_pane_height = Some(other_pane_height);
        self
    }

    pub fn other_pane_width(&mut self, other_pane_width: usize) -> &mut Self {
        self.other_pane_width = Some(other_pane_width);
        self
    }

    pub fn pane_active_border_style(&mut self, pane_active_border_style: &'a str) -> &mut Self {
        self.pane_active_border_style = Some(pane_active_border_style);
        self
    }

    pub fn pane_base_index(&mut self, pane_base_index: usize) -> &mut Self {
        self.pane_base_index = Some(pane_base_index);
        self
    }

    pub fn pane_border_format(&mut self, pane_border_format: &'a str) -> &mut Self {
        self.pane_border_format = Some(pane_border_format);
        self
    }

    pub fn pane_border_status(&mut self, pane_border_status: PaneBorderStatus) -> &mut Self {
        self.pane_border_status = Some(pane_border_status);
        self
    }

    pub fn synchronize_panes(&mut self, synchronize_panes: Switch) -> &mut Self {
        self.synchronize_panes = Some(synchronize_panes);
        self
    }

    pub fn window_status_activity_style(
        &mut self,
        window_status_activity_style: &'a str,
    ) -> &mut Self {
        self.window_status_activity_style = Some(window_status_activity_style);
        self
    }

    pub fn window_status_bell_style(&mut self, window_status_bell_style: &'a str) -> &mut Self {
        self.window_status_bell_style = Some(window_status_bell_style);
        self
    }

    pub fn window_status_current_format(
        &mut self,
        window_status_current_format: &'a str,
    ) -> &mut Self {
        self.window_status_current_format = Some(window_status_current_format);
        self
    }

    pub fn window_status_format(&mut self, window_status_format: &'a str) -> &mut Self {
        self.window_status_format = Some(window_status_format);
        self
    }

    pub fn window_status_last_style(&mut self, window_status_last_style: &'a str) -> &mut Self {
        self.window_status_last_style = Some(window_status_last_style);
        self
    }

    pub fn window_status_separator(&mut self, window_status_separator: &'a str) -> &mut Self {
        self.window_status_separator = Some(window_status_separator);
        self
    }

    pub fn window_status_style(&mut self, window_status_style: &'a str) -> &mut Self {
        self.window_status_style = Some(window_status_style);
        self
    }

    pub fn window_size(&mut self, window_size: WindowSize) -> &mut Self {
        self.window_size = Some(window_size);
        self
    }

    pub fn wrap_search(&mut self, wrap_search: Switch) -> &mut Self {
        self.wrap_search = Some(wrap_search);
        self
    }

    pub fn xterm_keys(&mut self, xterm_keys: Switch) -> &mut Self {
        self.xterm_keys = Some(xterm_keys);
        self
    }

    // XXX: clone rly needed?
    pub fn build(&self) -> WindowOptions {
        WindowOptions {
            aggressive_resize: self.aggressive_resize.clone(),
            automatic_rename: self.automatic_rename.clone(),
            automatic_rename_format: self.automatic_rename_format.map(|s| s.to_string()),
            clock_mode_colour: self.clock_mode_colour.map(|s| s.to_string()),
            clock_mode_style: self.clock_mode_style.clone(),
            main_pane_height: self.main_pane_height,
            main_pane_width: self.main_pane_width,
            mode_keys: self.mode_keys.clone(),
            mode_style: self.mode_style.map(|s| s.to_string()),
            monitor_activity: self.monitor_activity.clone(),
            monitor_bell: self.monitor_bell.clone(),
            monitor_silence: self.monitor_silence,
            other_pane_height: self.other_pane_height,
            other_pane_width: self.other_pane_width,
            pane_active_border_style: self.pane_active_border_style.map(|s| s.to_string()),
            pane_base_index: self.pane_base_index,
            pane_border_format: self.pane_border_format.map(|s| s.to_string()),
            pane_border_status: self.pane_border_status.clone(),
            pane_border_style: self.pane_border_style.map(|s| s.to_string()),
            synchronize_panes: self.synchronize_panes.clone(),
            window_status_activity_style: self.window_status_activity_style.map(|s| s.to_string()),
            window_status_bell_style: self.window_status_bell_style.map(|s| s.to_string()),
            window_status_current_format: self.window_status_current_format.map(|s| s.to_string()),
            window_status_current_style: self.window_status_current_style.map(|s| s.to_string()),
            window_status_format: self.window_status_format.map(|s| s.to_string()),
            window_status_last_style: self.window_status_last_style.map(|s| s.to_string()),
            window_status_separator: self.window_status_separator.map(|s| s.to_string()),
            window_status_style: self.window_status_style.map(|s| s.to_string()),
            window_size: self.window_size.clone(),
            wrap_search: self.wrap_search.clone(),
            xterm_keys: self.xterm_keys.clone(),
        }
    }
}
