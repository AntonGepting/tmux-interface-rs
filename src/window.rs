use std::time::Duration;
use regex::Regex;
use crate::TmuxInterfaceError;


pub const WINDOW_VARS_SEPARATOR: &str = "'";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
pub const WINDOW_VARS_REGEX_VEC: [(&str, &str); 22] = [
    ("window_activity",       r"(\d+)"),
    ("window_activity_flag",  r"(1|0)"),
    ("window_active",         r"(1|0)"),
    ("window_bell_flag",      r"(1|0)"),
    ("window_bigger",         r"(\w+)?"),
    ("window_flags",          r"([\w\*]*)?"),
    ("window_format",         r"(\w+)"),
    ("window_height",         r"(\d+)"),
    ("window_id",             r"@(\d+)"),
    ("window_index",          r"(\d+)"),
    ("window_last_flag",      r"(\d+)"),
    ("window_layout",         r"([\w,]*)?"),
    ("window_linked",         r"(\d+)"),
    ("window_name",           r"(\w+)"),
    ("window_offset_x",       r"(\w+)?"),
    ("window_offset_y",       r"(\w+)?"),
    ("window_panes",          r"(\d+)"),
    ("window_silence_flag",   r"(\d+)"),
    ("window_stack_index",    r"(\d+)"),
    ("window_visible_layout", r"([\w,]*)"),
    ("window_width",          r"(\d+)"),
    ("window_zoomed_flag",    r"(\d+)"),
];


// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Clone, Debug)]
pub struct Window {
    pub activity: Duration,
    pub activity_flag: bool,
    pub active: bool,
    pub bell_flag: bool,
    pub bigger: Option<String>,
    pub flags: Option<String>,
    pub format: String,
    pub height: usize,
    pub id: usize,
    pub index: usize,
    pub last_flag: usize,
    pub layout: String,
    pub linked: usize,
    pub name: String,
    pub offset_x: Option<String>,
    pub offset_y: Option<String>,
    pub panes: usize,
    pub silence_flag: usize,
    pub stack_index: usize,
    pub visible_layout: String,
    pub width: usize,
    pub zoomed_flag: usize
}


impl Default for Window {
    fn default() -> Self {
        Window {
            activity: Duration::from_millis(0),
            activity_flag: false,
            active: false,
            bell_flag: false,
            bigger: None,
            flags: None,
            format: "".to_string(),
            height: 0,
            id: 0,
            index: 0,
            last_flag: 0,
            layout: "".to_string(),
            linked: 0,
            name: "".to_string(),
            offset_x: None,
            offset_y: None,
            panes: 0,
            silence_flag: 0,
            stack_index: 0,
            visible_layout: "".to_string(),
            width: 0,
            zoomed_flag: 0
        }
    }
}


impl Window {

    pub fn new() -> Window {
        Default::default()
    }


    // XXX: mb deserialize like serde something?
    pub fn parse(window_str: &str) -> Result<Window, TmuxInterfaceError> {
        let regex_str = format!("^{}$", WINDOW_VARS_REGEX_VEC.iter()
                                .map(|t| t.1).collect::<Vec<&str>>().join(WINDOW_VARS_SEPARATOR));
        let regex = Regex::new(&regex_str)?;
        let caps = regex.captures(window_str).unwrap();
        let mut window = Window::new();
        window.activity = Duration::from_millis(caps[1].parse()?);
        window.active = caps[2].parse::<usize>().map(|i| i == 1)?;
        window.activity_flag = caps[3].parse::<usize>().map(|i| i == 1)?;
        window.bell_flag = caps[4].parse::<usize>().map(|i| i == 1)?;
        if let Some(bigger) = caps.get(5) {
            window.bigger = Some(bigger.as_str().parse()?);
        }
        if let Some(flags) = caps.get(6) {
            window.flags = Some(flags.as_str().parse()?);
        }
        window.format = caps[7].parse()?;
        window.height = caps[8].parse()?;
        window.id = caps[9].parse()?;
        window.index = caps[10].parse()?;
        window.last_flag = caps[11].parse()?;
        window.layout = caps[12].parse()?;
        window.linked = caps[13].parse()?;
        window.name = caps[14].parse()?;
        if let Some(offset_x) = caps.get(15) {
            window.offset_x = Some(offset_x.as_str().parse()?);
        }
        if let Some(offset_y) = caps.get(16) {
            window.offset_y = Some(offset_y.as_str().parse()?);
        }
        window.panes = caps[17].parse()?;
        window.silence_flag = caps[18].parse()?;
        window.stack_index = caps[19].parse()?;
        window.visible_layout = caps[20].parse()?;
        window.width = caps[21].parse()?;
        window.zoomed_flag = caps[22].parse()?;
        Ok(window)
    }

}
