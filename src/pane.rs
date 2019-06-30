use regex::Regex;
use crate::TmuxInterfaceError;


pub const PANE_VARS_SEPARATOR: &str = "'";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
pub const PANE_VARS_REGEX_VEC: [(&str, &str); 29] = [
    ("pane_active",             r"(1|0)?"),
    ("pane_at_bottom",          r"(1|0)?"),
    ("pane_at_left",            r"(1|0)?"),
    ("pane_at_right",           r"(1|0)?"),
    ("pane_at_top",             r"(1|0)?"),
    ("pane_bottom",             r"(\d+)?"),
    ("pane_current_command",    r"(\w+)?"),
    ("pane_current_path",       r"((/[^/ ]*)+/?)?"),
    ("pane_dead",               r"(1|0)?"),
    ("pane_dead_status",        r"(\d+)?"),
    ("pane_format",             r"(1|0)?"),
    ("pane_height",             r"(\d+)?"),
    ("pane_id",                 r"%(\d+)?"),
    ("pane_in_mode",            r"(\d+)?"),
    ("pane_input_off",          r"(\d+)?"),
    ("pane_index",              r"(\d+)?"),
    ("pane_left",               r"(\d+)?"),
    ("pane_mode",               r"(\d+)?"),
    ("pane_pid",                r"(\d+)?"),
    ("pane_pipe",               r"(\d+)?"),
    ("pane_right",              r"(\d+)?"),
    ("pane_search_string",      r"(\d+)?"),
    ("pane_start_command",      r"(\d+)?"),
    ("pane_synchronized",       r"(\d+)?"),
    ("pane_tabs",               r"([\w,]*)?"),
    ("pane_title",              r"(\w+)?"),
    ("pane_top",                r"(\d+)?"),
    ("pane_tty",                r"([\w/]*)?"),
    ("pane_width",              r"(\d+)?"),
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
        let regex_str = format!("^{}$", PANE_VARS_REGEX_VEC.iter()
                                .map(|t| t.1).collect::<Vec<&str>>().join(PANE_VARS_SEPARATOR));
        let regex = Regex::new(&regex_str)?;
        let caps = regex.captures(pane_str).unwrap();
        let mut pane = Pane::new();
        if let Some(active) = caps.get(1) {
            pane.active = Some(active.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(at_bottom) = caps.get(2) {
            pane.at_bottom = Some(at_bottom.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(at_left) = caps.get(3) {
            pane.at_left = Some(at_left.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(at_right) = caps.get(4) {
            pane.at_right = Some(at_right.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(at_top) = caps.get(5) {
            pane.at_top = Some(at_top.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(bottom) = caps.get(6) {
            pane.bottom = Some(bottom.as_str().parse()?);
        }
        if let Some(current_command) = caps.get(7) {
            pane.current_command = Some(current_command.as_str().parse()?);
        }
        if let Some(current_path) = caps.get(8) {
            pane.current_path = Some(current_path.as_str().parse()?); // path needs 2 caps, acc. to regex
        }
        if let Some(dead) = caps.get(10) {
            pane.dead = Some(dead.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(dead_status) = caps.get(11) {
            pane.dead_status = Some(dead_status.as_str().parse()?);
        }
        if let Some(format) = caps.get(12) {
            pane.format = Some(format.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(height) = caps.get(13) {
            pane.height = Some(height.as_str().parse()?);
        }
        if let Some(id) = caps.get(14) {
            pane.id = Some(id.as_str().parse()?);
        }
        if let Some(in_mode) = caps.get(15) {
            pane.in_mode = Some(in_mode.as_str().parse()?);
        }
        if let Some(input_off) = caps.get(16) {
            pane.input_off = Some(input_off.as_str().parse()?);
        }
        if let Some(index) = caps.get(17) {
            pane.index = Some(index.as_str().parse()?);
        }
        if let Some(left) = caps.get(18) {
            pane.left = Some(left.as_str().parse()?);
        }
        if let Some(mode) = caps.get(19) {
            pane.mode = Some(mode.as_str().parse()?);
        }
        if let Some(pid) = caps.get(20) {
            pane.pid = Some(pid.as_str().parse()?);
        }
        if let Some(pipe) = caps.get(21) {
            pane.pipe = Some(pipe.as_str().parse()?);
        }
        if let Some(right) = caps.get(22) {
            pane.right = Some(right.as_str().parse()?);
        }
        if let Some(search_string) = caps.get(23) {
            pane.search_string = Some(search_string.as_str().parse()?);
        }
        if let Some(start_command) = caps.get(24) {
            pane.start_command = Some(start_command.as_str().parse()?);
        }
        if let Some(synchronized) = caps.get(25) {
            pane.synchronized = Some(synchronized.as_str().parse()?);
        }
        if let Some(tabs) = caps.get(26) {
            pane.tabs = Some(tabs.as_str().parse()?);
        }
        if let Some(title) = caps.get(27) {
            pane.title = Some(title.as_str().parse()?);
        }
        if let Some(top) = caps.get(28) {
            pane.top = Some(top.as_str().parse()?);
        }
        if let Some(tty) = caps.get(29) {
            pane.tty = Some(tty.as_str().parse()?);
        }
        if let Some(width) = caps.get(30) {
            pane.width = Some(width.as_str().parse()?);
        }
        Ok(pane)
    }

}
