use std::time::Duration;
use regex::Regex;


//window_activity window_activity_flag window_active window_bell_flag window_bigger window_flags
//window_format window_height window_id window_index window_last_flag window_layout window_linked
//window_name window_offset_x window_offset_y window_panes window_silence_flag window_stack_index
//window_visible_layout window_width window_zoomed_flag
//
pub const LIST_WINDOWS_FORMAT: &str = "#{window_activity} #{window_activity_flag} \
#{window_active} #{window_bell_flag} #{window_id} #{window_index} #{window_name} #{window_panes}";
// numbered capture groups enclosed in parenthesis (...)
//
// FIXME: regex name can be anything, and other keys should be checked better
pub const WINDOW_STR_REGEX: &str = r"^(\d+) (1|0) (1|0) (1|0) @(\d+) (\d+) (\w+) (\d+)$";

// accordingly to tmux.h: Formats
//
#[derive(Clone, Debug)]
pub struct Window {
    pub activity: Duration,
    pub activity_flag: bool,
    pub active: bool,
    pub bell_flag: bool,
    //window_bigger
    //window_flags
    //window_format
    //window_height
    pub id: usize,
    pub index: usize,
    //window_last_flag
    //window_layout
    //window_linked
    pub name: String,
    //window_offset_x
    //window_offset_y
    pub panes: usize,
    //window_silence_flag
    //window_stack_index
    //window_visible_layout
    //window_width
    //window_zoomed_flag
}


impl Default for Window {
    fn default() -> Self {
        Window {
            activity: Duration::from_millis(0),
            activity_flag: false,
            active: false,
            bell_flag: false,
            id: 0,
            index: 0,
            name: "".to_string(),
            panes: 0
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
        let cap = regex.captures(window_str).unwrap();
        let mut window = Window::new();
        window.activity = Duration::from_millis(cap[1].parse().unwrap());
        window.active = cap[2].parse::<usize>().map(|i| i == 1).unwrap();
        window.activity_flag = cap[3].parse::<usize>().map(|i| i == 1).unwrap();
        window.bell_flag = cap[4].parse::<usize>().map(|i| i == 1).unwrap();
        window.id = cap[5].parse().unwrap();
        window.index = cap[6].parse().unwrap();
        window.name = cap[7].parse().unwrap();
        window.panes = cap[8].parse().unwrap();
        Ok(window)
    }

}


pub struct Windows {
    //sessions: Vec<Window>
}


impl Windows {

    pub fn parse(windows_str: &str) -> Result<Vec<Window>, ()> {
        let mut windows: Vec<Window> = Vec::new();
        for line in windows_str.lines() {
            windows.push(Window::parse(line).unwrap());
        }
        Ok(windows)
    }
}



