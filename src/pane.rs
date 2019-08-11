use crate::TmuxInterfaceError;
use std::str::FromStr;


pub const PANE_VARS_SEPARATOR: &str = "'";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
pub const PANE_VARS_REGEX_VEC: [&str; 31] = [
    "pane_active",
    "pane_at_bottom",
    "pane_at_left",
    "pane_at_right",
    "pane_at_top",
    "pane_bottom",
    "pane_current_command",
    "pane_current_path",
    "pane_dead",
    "pane_dead_status",
    "pane_format",
    "pane_height",
    "pane_id",
    "pane_in_mode",
    "pane_index",
    "pane_input_off",
    "pane_left",
    "pane_marked",
    "pane_marked_set",
    "pane_mode",
    "pane_pid",
    "pane_pipe",
    "pane_right",
    "pane_search_string",
    "pane_start_command",
    "pane_synchronized",
    "pane_tabs",
    "pane_title",
    "pane_top",
    "pane_tty",
    "pane_width",
];


// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, Clone, Debug)]
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
    pub width: Option<usize>
}


#[derive(Default, PartialEq, Clone, Debug)]
pub struct PaneTabs(pub Vec<usize>);


impl FromStr for PaneTabs {
    type Err = TmuxInterfaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let a: Vec<usize> = s.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let mut tabs = Vec::new();
        for tab in s.split(",").collect::<Vec<&str>>() {
            tabs.push(tab.parse()?);
        }
        Ok(Self(tabs))
    }
}


impl FromStr for Pane {
    type Err = TmuxInterfaceError;

    // XXX: mb serde, deserialize?
    fn from_str(pane_str: &str) -> Result<Pane, TmuxInterfaceError> {
        let pv: Vec<&str> = pane_str.split(PANE_VARS_SEPARATOR).collect();
        let mut p = Pane::new();
        // XXX: optimize?
        if !pv[0].is_empty() { p.active = pv[0].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[1].is_empty() { p.at_bottom = pv[1].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[2].is_empty() { p.at_left = pv[2].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[3].is_empty() { p.at_right = pv[3].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[4].is_empty() { p.at_top = pv[4].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[5].is_empty() { p.bottom = pv[5].parse().ok(); }
        if !pv[6].is_empty() { p.current_command = pv[6].parse().ok(); }
        if !pv[7].is_empty() { p.current_path = pv[7].parse().ok(); }
        if !pv[8].is_empty() { p.dead = pv[8].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[9].is_empty() { p.dead_status = pv[9].parse().ok(); }
        if !pv[10].is_empty() { p.format = pv[10].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[11].is_empty() { p.height = pv[11].parse().ok(); }
        if !pv[12].is_empty() { p.id = pv[12][1..].parse().ok(); } // skip '%' char
        if !pv[13].is_empty() { p.in_mode = pv[12].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[14].is_empty() { p.index = pv[14].parse().ok(); }
        if !pv[15].is_empty() { p.input_off = pv[15].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[16].is_empty() { p.left = pv[16].parse().ok(); }
        if !pv[17].is_empty() { p.marked = pv[17].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[18].is_empty() { p.marked_set = pv[18].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[19].is_empty() { p.mode = pv[19].parse().ok(); }
        if !pv[20].is_empty() { p.pid = pv[20].parse().ok(); }
        if !pv[21].is_empty() { p.pipe = pv[21].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[22].is_empty() { p.right = pv[22].parse().ok(); }
        if !pv[23].is_empty() { p.search_string = pv[23].parse().ok(); }
        if !pv[24].is_empty() { p.start_command = pv[24].parse().ok(); }
        if !pv[25].is_empty() { p.synchronized = pv[25].parse::<usize>().map(|i| i == 1).ok(); }
        if !pv[26].is_empty() { p.tabs = pv[26].parse().ok(); }
        if !pv[27].is_empty() { p.title = pv[27].parse().ok(); }
        if !pv[28].is_empty() { p.top = pv[28].parse().ok(); }
        if !pv[29].is_empty() { p.tty = pv[29].parse().ok(); }
        if !pv[30].is_empty() { p.width = pv[30].parse().ok(); }
        Ok(p)
    }


}


impl Pane {

    pub fn new() -> Self {
        Default::default()
    }


}
