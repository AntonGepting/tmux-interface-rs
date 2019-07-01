use std::time::Duration;
use crate::TmuxInterfaceError;


pub const WINDOW_VARS_SEPARATOR: &str = "'";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
pub const WINDOW_VARS_REGEX_VEC: [&str; 24] = [
    "window_activity",
    "window_activity_flag",
    "window_active",
    "window_bell_flag",
    "window_bigger",
    "window_end_flag",
    "window_flags",
    "window_format",
    "window_height",
    "window_id",
    "window_index",
    "window_last_flag",
    "window_layout",
    "window_linked",
    "window_name",
    "window_offset_x",
    "window_offset_y",
    "window_panes",
    "window_silence_flag",
    "window_stack_index",
    "window_start_flag",
    "window_visible_layout",
    "window_width",
    "window_zoomed_flag",
];


// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, Clone, Debug)]
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


impl Window {

    pub fn new() -> Self {
        Default::default()
    }


    // XXX: mb deserialize like serde something?
    pub fn parse(window_str: &str) -> Result<Window, TmuxInterfaceError> {
        let window_vars: Vec<&str> = window_str.split(WINDOW_VARS_SEPARATOR).collect();
        let mut window = Window::new();
        // XXX: optimize?
        window.activity = window_vars[0].parse().ok().map(Duration::from_millis);
        window.activity_flag = window_vars[1].parse::<usize>().map(|i| i == 0).ok();
        window.active = window_vars[2].parse::<usize>().map(|i| i == 1).ok();
        window.bell_flag = window_vars[3].parse::<usize>().map(|i| i == 1).ok();
        window.bigger = window_vars[4].parse().ok();
        window.end_flag = window_vars[5].parse::<usize>().map(|i| i == 1).ok();
        window.flags = window_vars[6].parse().ok();
        window.format = window_vars[7].parse().ok();
        window.height = window_vars[8].parse().ok();
        window.id = window_vars[9][1..].parse().ok();
        window.index = window_vars[10].parse().ok();
        window.last_flag = window_vars[11].parse().ok();
        window.layout = window_vars[12].parse().ok();
        window.linked = window_vars[13].parse().ok();
        window.name = window_vars[14].parse().ok();
        window.offset_x = window_vars[15].parse().ok();
        window.offset_y = window_vars[16].parse().ok();
        window.panes = window_vars[17].parse().ok();
        window.silence_flag = window_vars[18].parse().ok();
        window.stack_index = window_vars[19].parse().ok();
        window.start_flag = window_vars[20].parse().ok();
        window.visible_layout = window_vars[21].parse().ok();
        window.width = window_vars[22].parse().ok();
        window.zoomed_flag = window_vars[23].parse().ok();
        Ok(window)
    }

}
