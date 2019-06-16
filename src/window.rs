use std::time::Duration;
use regex::Regex;
use crate::TmuxInterfaceError;


pub const WINDOW_VARS_SEPARATOR: &str = "'";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
pub const WINDOW_VARS_REGEX_VEC: [(&str, &str); 24] = [
    ("window_activity",       r"(\d+)?"),
    ("window_activity_flag",  r"(1|0)?"),
    ("window_active",         r"(1|0)?"),
    ("window_bell_flag",      r"(1|0)?"),
    ("window_bigger",         r"(\w+)?"),
    ("window_end_flag",       r"(1|0)?"),
    ("window_flags",          r"([\w\*-]*)?"),
    ("window_format",         r"(\w+)?"),
    ("window_height",         r"(\d+)?"),
    ("window_id",             r"@(\d+)?"),
    ("window_index",          r"(\d+)?"),
    ("window_last_flag",      r"(\d+)?"),
    ("window_layout",         r"([\w,\[\]]*)?"),
    ("window_linked",         r"(\d+)?"),
    ("window_name",           r"(\w+)?"),
    ("window_offset_x",       r"(\w+)?"),
    ("window_offset_y",       r"(\w+)?"),
    ("window_panes",          r"(\d+)?"),
    ("window_silence_flag",   r"(\d+)?"),
    ("window_stack_index",    r"(\d+)?"),
    ("window_start_flag",     r"(\d+)?"),
    ("window_visible_layout", r"([\w,\[\]]*)?"),
    ("window_width",          r"(\d+)?"),
    ("window_zoomed_flag",    r"(\d+)?"),
];


// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Clone, Debug)]
pub struct Window {
    pub activity: Option<Duration>,
    pub activity_flag: Option<bool>,
    pub active: Option<bool>,
    pub bell_flag: Option<bool>,
    pub bigger: Option<String>,
    pub end_flag: Option<bool>,
    pub flags: Option<String>,
    pub format: Option<String>,
    pub height: Option<usize>,
    pub id: Option<usize>,
    pub index: Option<usize>,
    pub last_flag: Option<usize>,
    pub layout: Option<String>,
    pub linked: Option<usize>,
    pub name: Option<String>,
    pub offset_x: Option<String>,
    pub offset_y: Option<String>,
    pub panes: Option<usize>,
    pub silence_flag: Option<usize>,
    pub stack_index: Option<usize>,
    pub start_flag: Option<usize>,
    pub visible_layout: Option<String>,
    pub width: Option<usize>,
    pub zoomed_flag: Option<usize>
}


impl Default for Window {
    fn default() -> Self {
        Window {
            activity: None,
            activity_flag: None,
            active: None,
            bell_flag: None,
            bigger: None,
            end_flag: None,
            flags: None,
            format: None,
            height: None,
            id: None,
            index: None,
            last_flag: None,
            layout: None,
            linked: None,
            name: None,
            offset_x: None,
            offset_y: None,
            panes: None,
            silence_flag: None,
            stack_index: None,
            start_flag: None,
            visible_layout: None,
            width: None,
            zoomed_flag: None
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
        if let Some(activity) = caps.get(1) {
            window.activity = Some(Duration::from_millis(activity.as_str().parse()?));
        }
        if let Some(activity_flag) = caps.get(2) {
            window.activity_flag = Some(activity_flag.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(active) = caps.get(3) {
            window.active = Some(active.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(bell_flag) = caps.get(4) {
            window.bell_flag = Some(bell_flag.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(bigger) = caps.get(5) {
            window.bigger = Some(bigger.as_str().parse()?);
        }
        if let Some(end_flag) = caps.get(6) {
            window.end_flag = Some(end_flag.as_str().parse::<usize>().map(|i| i == 1)?);
        }
        if let Some(flags) = caps.get(7) {
            window.flags = Some(flags.as_str().parse()?);
        }
        if let Some(format) = caps.get(8) {
            window.format = Some(format.as_str().parse()?);
        }
        if let Some(height) = caps.get(9) {
            window.height = Some(height.as_str().parse()?);
        }
        if let Some(id) = caps.get(10) {
            window.id = Some(id.as_str().parse()?);
        }
        if let Some(index) = caps.get(11) {
            window.index = Some(index.as_str().parse()?);
        }
        if let Some(last_flag) = caps.get(12) {
            window.last_flag = Some(last_flag.as_str().parse()?);
        }
        if let Some(layout) = caps.get(13) {
            window.layout = Some(layout.as_str().parse()?);
        }
        if let Some(linked) = caps.get(14) {
            window.linked = Some(linked.as_str().parse()?);
        }
        if let Some(name) = caps.get(15) {
            window.name = Some(name.as_str().parse()?);
        }
        if let Some(offset_x) = caps.get(16) {
            window.offset_x = Some(offset_x.as_str().parse()?);
        }
        if let Some(offset_y) = caps.get(17) {
            window.offset_y = Some(offset_y.as_str().parse()?);
        }
        if let Some(panes) = caps.get(18) {
            window.panes = Some(panes.as_str().parse()?);
        }
        if let Some(silence_flag) = caps.get(19) {
            window.silence_flag = Some(silence_flag.as_str().parse()?);
        }
        if let Some(stack_index) = caps.get(20) {
            window.stack_index = Some(stack_index.as_str().parse()?);
        }
        if let Some(start_flag) = caps.get(21) {
            window.start_flag = Some(start_flag.as_str().parse()?);
        }
        if let Some(visible_layout) = caps.get(22) {
            window.visible_layout = Some(visible_layout.as_str().parse()?);
        }
        if let Some(width) = caps.get(23) {
            window.width = Some(width.as_str().parse()?);
        }
        if let Some(zoomed_flag) = caps.get(24) {
            window.zoomed_flag = Some(zoomed_flag.as_str().parse()?);
        }
        Ok(window)
    }

}
