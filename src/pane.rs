use crate::Error;
use crate::PaneTabs;

pub const PANE_VARS_SEPARATOR: &str = "'";
// FIXME: regex name can be anything, and other keys should be checked better
pub const PANE_VARS_REGEX_VEC: [(&str, usize); 31] = [
    ("pane_active", PANE_ACTIVE),
    ("pane_at_bottom", PANE_AT_BOTTOM),
    ("pane_at_left", PANE_AT_LEFT),
    ("pane_at_right", PANE_AT_RIGHT),
    ("pane_at_top", PANE_AT_TOP),
    ("pane_bottom", PANE_BOTTOM),
    ("pane_current_command", PANE_CURRENT_COMMAND),
    ("pane_current_path", PANE_CURRENT_PATH),
    ("pane_dead", PANE_DEAD),
    ("pane_dead_status", PANE_DEAD),
    ("pane_format", PANE_FORMAT),
    ("pane_height", PANE_HEIGHT),
    ("pane_id", PANE_ID),
    ("pane_in_mode", PANE_IN_MODE),
    ("pane_index", PANE_INDEX),
    ("pane_input_off", PANE_INPUT_OFF),
    ("pane_left", PANE_LEFT),
    ("pane_marked", PANE_MARKED),
    ("pane_marked_set", PANE_MARKED_SET),
    ("pane_mode", PANE_MODE),
    ("pane_pid", PANE_PID),
    ("pane_pipe", PANE_PIPE),
    ("pane_right", PANE_RIGHT),
    ("pane_search_string", PANE_SEARCH_STRING),
    ("pane_start_command", PANE_START_COMMMAND),
    ("pane_synchronized", PANE_SYNCHRONIZED),
    ("pane_tabs", PANE_TABS),
    ("pane_title", PANE_TITLE),
    ("pane_top", PANE_TOP),
    ("pane_tty", PANE_TTY),
    ("pane_width", PANE_WIDTH),
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

// XXX: mb macro with custom struct fields
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

// NOTE: u32 mb not enough!
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
//pub const PANE_DEFAULT: usize = PANE_ID | PANE_TITLE;
pub const PANE_ALL: usize = PANE_ACTIVE
    | PANE_AT_BOTTOM
    | PANE_AT_LEFT
    | PANE_AT_RIGHT
    | PANE_AT_TOP
    | PANE_BOTTOM
    | PANE_CURRENT_COMMAND
    | PANE_CURRENT_PATH
    | PANE_DEAD
    | PANE_DEAD_STATUS
    | PANE_FORMAT
    | PANE_HEIGHT
    | PANE_ID
    | PANE_IN_MODE
    | PANE_INDEX
    | PANE_INPUT_OFF
    | PANE_LEFT
    | PANE_MARKED
    | PANE_MARKED_SET
    | PANE_MODE
    | PANE_PID
    | PANE_PIPE
    | PANE_RIGHT
    | PANE_SEARCH_STRING
    | PANE_START_COMMMAND
    | PANE_SYNCHRONIZED
    | PANE_TABS
    | PANE_TITLE
    | PANE_TOP
    | PANE_TTY
    | PANE_WIDTH;

impl Pane {
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
                // does vector element exist?
                if let Some(part) = pv.next() {
                    // is vector element not empty
                    if !part.is_empty() {
                        // decode it and save as struct field
                        match bitflags & var.1 {
                            PANE_ACTIVE => p.active = part.parse::<usize>().map(|b| b == 1).ok(),
                            PANE_AT_BOTTOM => {
                                p.at_bottom = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            PANE_AT_LEFT => p.at_left = part.parse::<usize>().map(|b| b == 1).ok(),
                            PANE_AT_RIGHT => {
                                p.at_right = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            PANE_AT_TOP => p.at_top = part.parse::<usize>().map(|b| b == 1).ok(),
                            PANE_BOTTOM => p.bottom = part.parse().ok(),
                            PANE_CURRENT_COMMAND => p.current_command = part.parse().ok(),
                            PANE_CURRENT_PATH => p.current_path = part.parse().ok(),
                            PANE_DEAD => p.dead = part.parse::<usize>().map(|b| b == 1).ok(),
                            PANE_DEAD_STATUS => p.dead_status = part.parse().ok(),
                            PANE_FORMAT => p.format = part.parse::<usize>().map(|b| b == 1).ok(),
                            PANE_HEIGHT => p.height = part.parse().ok(),
                            PANE_ID => p.id = part[1..].parse().ok(), // skip '%' char
                            PANE_IN_MODE => p.in_mode = part.parse::<usize>().map(|b| b == 1).ok(),
                            PANE_INDEX => p.index = part.parse().ok(),
                            PANE_INPUT_OFF => {
                                p.input_off = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            PANE_LEFT => p.left = part.parse().ok(),
                            PANE_MARKED => p.marked = part.parse::<usize>().map(|b| b == 1).ok(),
                            PANE_MARKED_SET => {
                                p.marked_set = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            PANE_MODE => p.mode = part.parse().ok(),
                            PANE_PID => p.pid = part.parse().ok(),
                            PANE_PIPE => p.pipe = part.parse::<usize>().map(|b| b == 1).ok(),
                            PANE_RIGHT => p.right = part.parse().ok(),
                            PANE_SEARCH_STRING => p.search_string = part.parse().ok(),
                            PANE_START_COMMMAND => p.start_command = part.parse().ok(),
                            PANE_SYNCHRONIZED => {
                                p.synchronized = part.parse::<usize>().map(|b| b == 1).ok()
                            }
                            PANE_TABS => p.tabs = part.parse().ok(),
                            PANE_TITLE => p.title = part.parse().ok(),
                            PANE_TOP => p.top = part.parse().ok(),
                            PANE_TTY => p.tty = part.parse().ok(),
                            PANE_WIDTH => p.width = part.parse().ok(),
                            _ => (),
                        }
                    }
                }
            }
        }
        Ok(p)
    }
}
