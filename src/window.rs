use std::time::Duration;
use crate::TmuxInterfaceError;
use crate::Layout;
use std::str::FromStr;


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
    pub layout: Option<Layout>,
    pub linked: Option<usize>,
    pub name: Option<String>,
    pub offset_x: Option<String>,
    pub offset_y: Option<String>,
    pub panes: Option<usize>,
    pub silence_flag: Option<usize>,
    pub stack_index: Option<usize>,
    pub start_flag: Option<usize>,
    pub visible_layout: Option<Layout>,
    pub width: Option<usize>,
    pub zoomed_flag: Option<usize>
}



impl FromStr for Window {
    type Err = TmuxInterfaceError;

    // XXX: mb deserialize like serde something?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let wv: Vec<&str> = s.split(WINDOW_VARS_SEPARATOR).collect();
        let mut w = Window::new();
        // XXX: optimize?
        if !wv[0].is_empty() { w.activity = wv[0].parse().ok().map(Duration::from_millis); }
        if !wv[1].is_empty() { w.activity_flag = wv[1].parse::<usize>().map(|i| i == 0).ok(); }
        if !wv[2].is_empty() { w.active = wv[2].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[3].is_empty() { w.bell_flag = wv[3].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[4].is_empty() { w.bigger = wv[4].parse().ok(); }
        if !wv[5].is_empty() { w.end_flag = wv[5].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[6].is_empty() { w.flags = wv[6].parse().ok(); }
        if !wv[7].is_empty() { w.format = wv[7].parse().ok(); }
        if !wv[8].is_empty() { w.height = wv[8].parse().ok(); }
        if !wv[9].is_empty() { w.id = wv[9][1..].parse().ok(); }
        if !wv[10].is_empty() { w.index = wv[10].parse().ok(); }
        if !wv[11].is_empty() { w.last_flag = wv[11].parse().ok(); }
        if !wv[12].is_empty() { w.layout = wv[12].parse().ok(); }
        if !wv[13].is_empty() { w.linked = wv[13].parse().ok(); }
        if !wv[14].is_empty() { w.name = wv[14].parse().ok(); }
        if !wv[15].is_empty() { w.offset_x = wv[15].parse().ok(); }
        if !wv[16].is_empty() { w.offset_y = wv[16].parse().ok(); }
        if !wv[17].is_empty() { w.panes = wv[17].parse().ok(); }
        if !wv[18].is_empty() { w.silence_flag = wv[18].parse().ok(); }
        if !wv[19].is_empty() { w.stack_index = wv[19].parse().ok(); }
        if !wv[20].is_empty() { w.start_flag = wv[20].parse().ok(); }
        if !wv[21].is_empty() { w.visible_layout = wv[21].parse().ok(); }
        if !wv[22].is_empty() { w.width = wv[22].parse().ok(); }
        if !wv[23].is_empty() { w.zoomed_flag = wv[23].parse().ok(); }
        Ok(w)
    }


}


impl Window {

    pub fn new() -> Self {
        Default::default()
    }

}
