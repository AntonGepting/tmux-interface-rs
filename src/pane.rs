use regex::Regex;
use crate::TmuxInterfaceError;


pub const PANE_VARS_SEPARATOR: &str = "'";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
pub const PANE_VARS_REGEX_VEC: [(&str, &str); 29] = [
    ("pane_active",             r"(1|0)"),
    ("pane_at_bottom",          r"(1|0)"),
    ("pane_at_left",            r"(1|0)"),
    ("pane_at_right",           r"(1|0)"),
    ("pane_at_top",             r"(1|0)"),
    ("pane_bottom",             r"(\d+)"),
    ("pane_current_command",    r"(\w+)"),
    ("pane_current_path",       r"((/[^/ ]*)+/?)"),
    ("pane_dead",               r"(1|0)"),
    ("pane_dead_status",        r"(\d+)?"),
    ("pane_format",             r"(1|0)"),
    ("pane_height",             r"(\d+)"),
    ("pane_id",                 r"%(\d+)"),
    ("pane_in_mode",            r"(\d+)"),
    ("pane_input_off",          r"(\d+)"),
    ("pane_index",              r"(\d+)"),
    ("pane_left",               r"(\d+)"),
    ("pane_mode",               r"(\d+)?"),
    ("pane_pid",                r"(\d+)"),
    ("pane_pipe",               r"(\d+)"),
    ("pane_right",              r"(\d+)"),
    ("pane_search_string",      r"(\d+)?"),
    ("pane_start_command",      r"(\d+)?"),
    ("pane_synchronized",       r"(\d+)"),
    ("pane_tabs",               r"([\w,]*)"),
    ("pane_title",              r"(\w+)"),
    ("pane_top",                r"(\d+)"),
    ("pane_tty",                r"([\w/]*)"),
    ("pane_width",              r"(\d+)"),
];


// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Clone, Debug)]
pub struct Pane {
    pub active: bool,
    pub at_bottom: bool,
    pub at_left: bool,
    pub at_right: bool,
    pub at_top: bool,
    pub bottom: usize,
    pub current_command: String,
    pub current_path: String,
    pub dead: bool,
    pub dead_status: Option<usize>,
    pub format: bool,
    pub height: usize,
    pub id: usize,
    pub in_mode: usize,
    pub input_off: usize,
    pub index: usize,
    pub left: usize,
    pub mode: Option<usize>,
    pub pid: usize,
    pub pipe: usize,
    pub right: usize,
    pub search_string: Option<usize>,
    pub start_command: Option<usize>,
    pub synchronized: usize,
    pub tabs: String,
    pub title: String,
    pub top: usize,
    pub tty: String,
    pub width: usize
}


impl Default for Pane {
    fn default() -> Self {
        Pane {
            active: false,
            at_bottom: false,
            at_left: false,
            at_right: false,
            at_top: false,
            bottom: 0,
            current_command: "".to_string(),
            current_path: "".to_string(),
            dead: false,
            dead_status: None,
            format: false,
            height: 0,
            id: 0,
            in_mode: 0,
            input_off: 0,
            index: 0,
            left: 0,
            mode: None,
            pid: 0,
            pipe: 0,
            right: 0,
            search_string: None,
            start_command: None,
            synchronized: 0,
            tabs: "".to_string(),
            title: "".to_string(),
            top: 0,
            tty: "".to_string(),
            width: 0
        }
    }
}


impl Pane {

    pub fn new() -> Pane {
        Default::default()
    }


    // XXX: mb serde, deserialize?
    pub fn parse(pane_str: &str) -> Result<Pane, TmuxInterfaceError> {
        let regex_str = format!("^{}$", PANE_VARS_REGEX_VEC.iter()
                                .map(|t| t.1).collect::<Vec<&str>>().join(PANE_VARS_SEPARATOR));
        let regex = Regex::new(&regex_str)?;
        let caps = regex.captures(pane_str).unwrap();
        let mut pane = Pane::new();
        pane.active = caps[1].parse::<usize>().map(|i| i == 1)?;
        pane.at_bottom = caps[2].parse::<usize>().map(|i| i == 1)?;
        pane.at_left = caps[3].parse::<usize>().map(|i| i == 1)?;
        pane.at_right = caps[4].parse::<usize>().map(|i| i == 1)?;
        pane.at_top = caps[5].parse::<usize>().map(|i| i == 1)?;
        pane.bottom = caps[6].parse()?;
        pane.current_command = caps[7].parse()?;
        pane.current_path = caps[8].parse()?; // path needs 2 caps, acc. to regex
        pane.dead = caps[10].parse::<usize>().map(|i| i == 1)?;
        if let Some(dead_status) = caps.get(11) {
            pane.dead_status = Some(dead_status.as_str().parse()?);
        }
        pane.format = caps[12].parse::<usize>().map(|i| i == 1)?;
        pane.height = caps[13].parse()?;
        pane.id = caps[14].parse()?;
        pane.in_mode = caps[15].parse()?;
        pane.input_off = caps[16].parse()?;
        pane.index = caps[17].parse()?;
        pane.left = caps[18].parse()?;
        if let Some(mode) = caps.get(19) {
            pane.mode = Some(mode.as_str().parse()?);
        }
        pane.pid = caps[20].parse()?;
        pane.pipe = caps[21].parse()?;
        pane.right = caps[22].parse()?;
        if let Some(search_string) = caps.get(23) {
            pane.search_string = Some(search_string.as_str().parse()?);
        }
        if let Some(start_command) = caps.get(24) {
            pane.start_command = Some(start_command.as_str().parse()?);
        }
        pane.synchronized = caps[25].parse()?;
        pane.tabs = caps[26].parse()?;
        pane.title = caps[27].parse()?;
        pane.top = caps[28].parse()?;
        pane.tty = caps[29].parse()?;
        pane.width = caps[30].parse()?;
        Ok(pane)
    }

}
