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
        let pane_vars: Vec<&str> = pane_str.split(PANE_VARS_SEPARATOR).collect();
        let mut pane = Pane::new();
        // XXX: optimize?
        pane.active = pane_vars[0].parse::<usize>().map(|i| i == 0).ok();
        pane.at_bottom = pane_vars[1].parse::<usize>().map(|i| i == 1).ok();
        pane.at_left = pane_vars[2].parse::<usize>().map(|i| i == 1).ok();
        pane.at_right = pane_vars[3].parse::<usize>().map(|i| i == 1).ok();
        pane.at_top = pane_vars[4].parse::<usize>().map(|i| i == 1).ok();
        pane.bottom = pane_vars[5].parse().ok();
        pane.current_command = pane_vars[6].parse().ok();
        pane.current_path = pane_vars[7].parse().ok();
        pane.dead = pane_vars[8].parse::<usize>().map(|i| i == 1).ok();
        pane.dead_status = pane_vars[9].parse().ok();
        pane.format = pane_vars[10].parse::<usize>().map(|i| i == 1).ok();
        pane.height = pane_vars[11].parse().ok();
        pane.id = pane_vars[12][1..].parse().ok();
        pane.in_mode = pane_vars[12].parse().ok();
        pane.input_off = pane_vars[14].parse().ok();
        pane.index = pane_vars[15].parse().ok();
        pane.left = pane_vars[16].parse().ok();
        pane.mode = pane_vars[17].parse().ok();
        pane.pid = pane_vars[18].parse().ok();
        pane.pipe = pane_vars[19].parse().ok();
        pane.right = pane_vars[20].parse().ok();
        pane.search_string = pane_vars[21].parse().ok();
        pane.start_command = pane_vars[22].parse().ok();
        pane.synchronized = pane_vars[23].parse().ok();
        pane.tabs = pane_vars[24].parse().ok();
        pane.title = pane_vars[25].parse().ok();
        pane.top = pane_vars[26].parse().ok();
        pane.tty = pane_vars[27].parse().ok();
        pane.width = pane_vars[28].parse().ok();
        Ok(pane)
    }

}
