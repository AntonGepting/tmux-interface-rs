use crate::Error;
use crate::PaneTabs;

pub const PANE_VARS_SEPARATOR: &str = "'";
// FIXME: regex name can be anything, and other keys should be checked better
pub const PANE_VARS_REGEX_VEC: [(&str, usize); 31] = [
    ("pane_active", Pane::PANE_ACTIVE),
    ("pane_at_bottom", Pane::PANE_AT_BOTTOM),
    ("pane_at_left", Pane::PANE_AT_LEFT),
    ("pane_at_right", Pane::PANE_AT_RIGHT),
    ("pane_at_top", Pane::PANE_AT_TOP),
    ("pane_bottom", Pane::PANE_BOTTOM),
    ("pane_current_command", Pane::PANE_CURRENT_COMMAND),
    ("pane_current_path", Pane::PANE_CURRENT_PATH),
    ("pane_dead", Pane::PANE_DEAD),
    ("pane_dead_status", Pane::PANE_DEAD),
    ("pane_format", Pane::PANE_FORMAT),
    ("pane_height", Pane::PANE_HEIGHT),
    ("pane_id", Pane::PANE_ID),
    ("pane_in_mode", Pane::PANE_IN_MODE),
    ("pane_index", Pane::PANE_INDEX),
    ("pane_input_off", Pane::PANE_INPUT_OFF),
    ("pane_left", Pane::PANE_LEFT),
    ("pane_marked", Pane::PANE_MARKED),
    ("pane_marked_set", Pane::PANE_MARKED_SET),
    ("pane_mode", Pane::PANE_MODE),
    ("pane_pid", Pane::PANE_PID),
    ("pane_pipe", Pane::PANE_PIPE),
    ("pane_right", Pane::PANE_RIGHT),
    ("pane_search_string", Pane::PANE_SEARCH_STRING),
    ("pane_start_command", Pane::PANE_START_COMMMAND),
    ("pane_synchronized", Pane::PANE_SYNCHRONIZED),
    ("pane_tabs", Pane::PANE_TABS),
    ("pane_title", Pane::PANE_TITLE),
    ("pane_top", Pane::PANE_TOP),
    ("pane_tty", Pane::PANE_TTY),
    ("pane_width", Pane::PANE_WIDTH),
];

//pub fn get_fmt_string(bitflags: usize) -> String {
//let lsp_format = PANE_VARS_REGEX_VEC
//.iter()
//.filter(|t| bitflags & t.1 == t.1)
//.map(|t| format!("#{{{}}}", t.0))
//.collect::<Vec<String>>()
//.join(PANE_VARS_SEPARATOR);
//lsp_format
//}

// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Pane {
    /// 1 if active pane
    pub active: Option<bool>,
    /// 1 if pane is at the bottom of window
    pub at_bottom: Option<bool>,
    /// 1 if pane is at the left of window
    pub at_left: Option<bool>,
    /// 1 if pane is at the right of window
    pub at_right: Option<bool>,
    /// 1 if pane is at the top of window
    pub at_top: Option<bool>,
    /// Bottom of pane
    pub bottom: Option<usize>,
    /// Current command if available
    pub current_command: Option<String>,
    /// Current path if available
    pub current_path: Option<String>,
    /// 1 if pane is dead
    pub dead: Option<bool>,
    /// Exit status of process in dead pane
    pub dead_status: Option<usize>,
    /// 1 if format is for a pane (not assuming the current)
    pub format: Option<bool>,
    /// Height of pane
    pub height: Option<usize>,
    /// #D Unique pane ID
    pub id: Option<usize>,
    /// 1 if pane is in a mode
    pub in_mode: Option<bool>,
    /// #P Index of pane
    pub index: Option<usize>,
    /// 1 if input to pane is disabled
    pub input_off: Option<bool>,
    /// Left of pane
    pub left: Option<usize>,
    /// 1 if this is the marked pane
    pub marked: Option<bool>,
    /// 1 if a marked pane is set
    pub marked_set: Option<bool>,
    /// Name of pane mode, if any
    pub mode: Option<usize>,
    /// PID of first process in pane
    pub pid: Option<usize>,
    /// 1 if pane is being piped
    pub pipe: Option<bool>,
    /// Right of pane
    pub right: Option<usize>,
    /// Last search string in copy mode
    pub search_string: Option<usize>,
    /// Command pane started with
    pub start_command: Option<usize>,
    /// 1 if pane is synchronized
    pub synchronized: Option<bool>,
    /// Pane tab positions
    pub tabs: Option<PaneTabs>,
    /// #T Title of pane
    pub title: Option<String>,
    /// Top of pane
    pub top: Option<usize>,
    /// Pseudo terminal of pane
    pub tty: Option<String>,
    /// Width of pane
    pub width: Option<usize>,
}

impl Pane {
    pub const PANE_ACTIVE: usize = 1 << 0;
    pub const PANE_AT_BOTTOM: usize = 1 << 1;
    pub const PANE_AT_LEFT: usize = 1 << 2;
    pub const PANE_AT_RIGHT: usize = 1 << 3;
    pub const PANE_AT_TOP: usize = 1 << 4;
    pub const PANE_BOTTOM: usize = 1 << 5;
    pub const PANE_CURRENT_COMMAND: usize = 1 << 6;
    pub const PANE_CURRENT_PATH: usize = 1 << 7;
    pub const PANE_DEAD: usize = 1 << 8;
    pub const PANE_DEAD_STATUS: usize = 1 << 9;
    pub const PANE_FORMAT: usize = 1 << 10;
    pub const PANE_HEIGHT: usize = 1 << 11;
    pub const PANE_ID: usize = 1 << 12;
    pub const PANE_IN_MODE: usize = 1 << 13;
    pub const PANE_INDEX: usize = 1 << 14;
    pub const PANE_INPUT_OFF: usize = 1 << 15;
    pub const PANE_LEFT: usize = 1 << 16;
    pub const PANE_MARKED: usize = 1 << 17;
    pub const PANE_MARKED_SET: usize = 1 << 18;
    pub const PANE_MODE: usize = 1 << 19;
    pub const PANE_PID: usize = 1 << 20;
    pub const PANE_PIPE: usize = 1 << 21;
    pub const PANE_RIGHT: usize = 1 << 22;
    pub const PANE_SEARCH_STRING: usize = 1 << 23;
    pub const PANE_START_COMMMAND: usize = 1 << 24;
    pub const PANE_SYNCHRONIZED: usize = 1 << 25;
    pub const PANE_TABS: usize = 1 << 26;
    pub const PANE_TITLE: usize = 1 << 27;
    pub const PANE_TOP: usize = 1 << 28;
    pub const PANE_TTY: usize = 1 << 29;
    pub const PANE_WIDTH: usize = 1 << 30;

    pub const PANE_NONE: usize = 0;
    pub const PANE_ALL: usize = Self::PANE_ACTIVE
        | Self::PANE_AT_BOTTOM
        | Self::PANE_AT_LEFT
        | Self::PANE_AT_RIGHT
        | Self::PANE_AT_TOP
        | Self::PANE_BOTTOM
        | Self::PANE_CURRENT_COMMAND
        | Self::PANE_CURRENT_PATH
        | Self::PANE_DEAD
        | Self::PANE_DEAD_STATUS
        | Self::PANE_FORMAT
        | Self::PANE_HEIGHT
        | Self::PANE_ID
        | Self::PANE_IN_MODE
        | Self::PANE_INDEX
        | Self::PANE_INPUT_OFF
        | Self::PANE_LEFT
        | Self::PANE_MARKED
        | Self::PANE_MARKED_SET
        | Self::PANE_MODE
        | Self::PANE_PID
        | Self::PANE_PIPE
        | Self::PANE_RIGHT
        | Self::PANE_SEARCH_STRING
        | Self::PANE_START_COMMMAND
        | Self::PANE_SYNCHRONIZED
        | Self::PANE_TABS
        | Self::PANE_TITLE
        | Self::PANE_TOP
        | Self::PANE_TTY
        | Self::PANE_WIDTH;

    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str(pane_str: &str, bitflags: usize) -> Result<Self, Error> {
        let pv: Vec<&str> = pane_str.split(PANE_VARS_SEPARATOR).collect();
        let mut pv = pv.iter();
        // XXX: optimize?
        let mut p = Pane::new();
        // for all bitflags
        for var in PANE_VARS_REGEX_VEC.iter() {
            // is current bitflag given?
            if bitflags & var.1 == var.1 {
                // does vector element exit?
                if let Some(part) = pv.next() {
                    // is verctor element not empty
                    if !part.is_empty() {
                        // decode it and save as struct field
                        match bitflags & var.1 {
                            Pane::PANE_ACTIVE => {
                                p.active = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_AT_BOTTOM => {
                                p.at_bottom = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_AT_LEFT => {
                                p.at_left = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_AT_RIGHT => {
                                p.at_right = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_AT_TOP => {
                                p.at_top = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_BOTTOM => p.bottom = part.parse().ok(),
                            Pane::PANE_CURRENT_COMMAND => p.current_command = part.parse().ok(),
                            Pane::PANE_CURRENT_PATH => p.current_path = part.parse().ok(),
                            Pane::PANE_DEAD => p.dead = part.parse::<usize>().map(|b| b == 1).ok(),
                            Pane::PANE_DEAD_STATUS => p.dead_status = part.parse().ok(),
                            Pane::PANE_FORMAT => {
                                p.format = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_HEIGHT => p.height = part.parse().ok(),
                            Pane::PANE_ID => p.id = part[1..].parse().ok(), // skip '%' char
                            Pane::PANE_IN_MODE => {
                                p.in_mode = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_INDEX => p.index = part.parse().ok(),
                            Pane::PANE_INPUT_OFF => {
                                p.input_off = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_LEFT => p.left = part.parse().ok(),
                            Pane::PANE_MARKED => {
                                p.marked = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_MARKED_SET => {
                                p.marked_set = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_MODE => p.mode = part.parse().ok(),
                            Pane::PANE_PID => p.pid = part.parse().ok(),
                            Pane::PANE_PIPE => p.pipe = part.parse::<usize>().map(|b| b == 1).ok(),
                            Pane::PANE_RIGHT => p.right = part.parse().ok(),
                            Pane::PANE_SEARCH_STRING => p.search_string = part.parse().ok(),
                            Pane::PANE_START_COMMMAND => p.start_command = part.parse().ok(),
                            Pane::PANE_SYNCHRONIZED => {
                                p.synchronized = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            Pane::PANE_TABS => p.tabs = part.parse().ok(),
                            Pane::PANE_TITLE => p.title = part.parse().ok(),
                            Pane::PANE_TOP => p.top = part.parse().ok(),
                            Pane::PANE_TTY => p.tty = part.parse().ok(),
                            Pane::PANE_WIDTH => p.width = part.parse().ok(),
                            _ => (),
                        }
                    }
                }
            }
        }
        Ok(p)
    }
}
