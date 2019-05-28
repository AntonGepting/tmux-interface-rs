use std::time::Duration;
use regex::Regex;


pub const LIST_WINDOWS_FORMAT: &str = "#{window_activity}'#{window_activity_flag}'\
#{window_active}'#{window_bell_flag}'#{window_bigger}'#{window_flags}'#{window_format}'\
#{window_height}'#{window_id}'#{window_index}'#{window_last_flag}'#{window_layout}'\
#{window_linked}'#{window_name}'#{window_offset_x}'#{window_offset_y}'#{window_panes}'\
#{window_silence_flag}'#{window_stack_index}'#{window_visible_layout}'#{window_width}'\
#{window_zoomed_flag}";


// numbered capture groups enclosed in parenthesis (...)
//
// FIXME: regex name can be anything, and other keys should be checked better
pub const WINDOW_STR_REGEX: &str = r"^(\d+)'(1|0)'(1|0)'(1|0)'(\w+)?'([\w\*]*)?'(\w+)'(\d+)'@(\d+)'(\d+)'(\d+)'([\w,]*)?'(\d+)'(\w+)'(\w+)?'(\w+)?'(\d+)'(\d+)'(\d+)'([\w,]*)'(\d+)'(\d+)$";

// accordingly to tmux.h: Formats
//
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
    //
    pub fn parse(window_str: &str) -> Result<Window, ()> {
        let regex = Regex::new(WINDOW_STR_REGEX).unwrap();
        let caps = regex.captures(window_str).unwrap();
        let mut window = Window::new();
        window.activity = Duration::from_millis(caps[1].parse().unwrap());
        window.active = caps[2].parse::<usize>().map(|i| i == 1).unwrap();
        window.activity_flag = caps[3].parse::<usize>().map(|i| i == 1).unwrap();
        window.bell_flag = caps[4].parse::<usize>().map(|i| i == 1).unwrap();
        if let Some(bigger) = caps.get(5) {
            window.bigger = Some(bigger.as_str().parse().unwrap());
        }
        if let Some(flags) = caps.get(6) {
            window.flags = Some(flags.as_str().parse().unwrap());
        }
        window.format = caps[7].parse().unwrap();
        window.height = caps[8].parse().unwrap();
        window.id = caps[9].parse().unwrap();
        window.index = caps[10].parse().unwrap();
        window.last_flag = caps[11].parse().unwrap();
        window.layout = caps[12].parse().unwrap();
        window.linked = caps[13].parse().unwrap();
        window.name = caps[14].parse().unwrap();
        if let Some(offset_x) = caps.get(15) {
            window.offset_x = Some(offset_x.as_str().parse().unwrap());
        }
        if let Some(offset_y) = caps.get(16) {
            window.offset_y = Some(offset_y.as_str().parse().unwrap());
        }
        window.panes = caps[17].parse().unwrap();
        window.silence_flag = caps[18].parse().unwrap();
        window.stack_index = caps[19].parse().unwrap();
        window.visible_layout = caps[20].parse().unwrap();
        window.width = caps[21].parse().unwrap();
        window.zoomed_flag = caps[22].parse().unwrap();
        Ok(window)
    }

}
