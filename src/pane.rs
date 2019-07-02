use crate::TmuxInterfaceError;


pub const PANE_VARS_SEPARATOR: &str = "'";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
pub const PANE_VARS_REGEX_VEC: [&str; 29] = [
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
    "pane_input_off",
    "pane_index",
    "pane_left",
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
    pub active: Option<bool>,
    pub at_bottom: Option<bool>,
    pub at_left: Option<bool>,
    pub at_right: Option<bool>,
    pub at_top: Option<bool>,
    pub bottom: Option<usize>,
    pub current_command: Option<String>,
    pub current_path: Option<String>,
    pub dead: Option<bool>,
    pub dead_status: Option<usize>,
    pub format: Option<bool>,
    pub height: Option<usize>,
    pub id: Option<usize>,
    pub in_mode: Option<usize>,
    pub input_off: Option<usize>,
    pub index: Option<usize>,
    pub left: Option<usize>,
    pub mode: Option<usize>,
    pub pid: Option<usize>,
    pub pipe: Option<usize>,
    pub right: Option<usize>,
    pub search_string: Option<usize>,
    pub start_command: Option<usize>,
    pub synchronized: Option<usize>,
    pub tabs: Option<String>,
    pub title: Option<String>,
    pub top: Option<usize>,
    pub tty: Option<String>,
    pub width: Option<usize>
}


impl Pane {

    pub fn new() -> Self {
        Default::default()
    }


    // XXX: mb serde, deserialize?
    pub fn parse(pane_str: &str) -> Result<Pane, TmuxInterfaceError> {
        let pv: Vec<&str> = pane_str.split(PANE_VARS_SEPARATOR).collect();
        let mut p = Pane::new();
        // XXX: optimize?
        if !pv[0].is_empty() { p.active = pv[0].parse::<usize>().map(|i| i == 0).ok(); }
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
        if !pv[12].is_empty() { p.id = pv[12][1..].parse().ok(); }
        if !pv[13].is_empty() { p.in_mode = pv[12].parse().ok(); }
        if !pv[14].is_empty() { p.input_off = pv[14].parse().ok(); }
        if !pv[15].is_empty() { p.index = pv[15].parse().ok(); }
        if !pv[16].is_empty() { p.left = pv[16].parse().ok(); }
        if !pv[17].is_empty() { p.mode = pv[17].parse().ok(); }
        if !pv[18].is_empty() { p.pid = pv[18].parse().ok(); }
        if !pv[19].is_empty() { p.pipe = pv[19].parse().ok(); }
        if !pv[20].is_empty() { p.right = pv[20].parse().ok(); }
        if !pv[21].is_empty() { p.search_string = pv[21].parse().ok(); }
        if !pv[22].is_empty() { p.start_command = pv[22].parse().ok(); }
        if !pv[23].is_empty() { p.synchronized = pv[23].parse().ok(); }
        if !pv[24].is_empty() { p.tabs = pv[24].parse().ok(); }
        if !pv[25].is_empty() { p.title = pv[25].parse().ok(); }
        if !pv[26].is_empty() { p.top = pv[26].parse().ok(); }
        if !pv[27].is_empty() { p.tty = pv[27].parse().ok(); }
        if !pv[28].is_empty() { p.width = pv[28].parse().ok(); }
        Ok(p)
    }

}
