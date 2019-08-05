use std::time::Duration;
use crate::TmuxInterfaceError;
use crate::Layout;
use std::str::FromStr;


pub const WINDOW_VARS_SEPARATOR: &str = "'";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
pub const WINDOW_VARS_REGEX_VEC: [&str; 24] = [
    "window_active",
    "window_activity",
    "window_activity_flag",
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


const WINDOW_FLAG_DEFAULT: usize     = 0b0000_0000;
const WINDOW_FLAG_CURRENT: usize     = 0b0000_0001;
const WINDOW_FLAG_LAST: usize        = 0b0000_0010;
const WINDOW_FLAG_ACTIVITY: usize    = 0b0000_0100;
const WINDOW_FLAG_BELL: usize        = 0b0000_1000;
const WINDOW_FLAG_SILENCED: usize    = 0b0001_0000;
const WINDOW_FLAG_MARKED: usize      = 0b0010_0000;
const WINDOW_FLAG_ZOOMED: usize      = 0b0100_0000;


#[derive(Debug, Clone)]
pub struct WindowFlag(usize);


impl Default for WindowFlag {
    fn default() -> Self {
        WindowFlag(WINDOW_FLAG_DEFAULT)
    }
}


impl FromStr for WindowFlag {
    type Err = TmuxInterfaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wf = WindowFlag(WINDOW_FLAG_DEFAULT);
        let mut chrs = s.chars();
        loop {
            if let Some(c) = chrs.next() {
                dbg!(c);
                match c {
                    '*' => wf.0 += WINDOW_FLAG_CURRENT,
                    '-' => wf.0 += WINDOW_FLAG_LAST,
                    '#' => wf.0 += WINDOW_FLAG_ACTIVITY,
                    '!' => wf.0 += WINDOW_FLAG_BELL,
                    '~' => wf.0 += WINDOW_FLAG_SILENCED,
                    'M' => wf.0 += WINDOW_FLAG_MARKED,
                    'Z' => wf.0 += WINDOW_FLAG_ZOOMED,
                    // XXX: Error description
                    _ => return Err(TmuxInterfaceError::new("Parse WindowFlag Error"))
                }
            } else {
                break;
            }
        }
        Ok(wf)
    }
}


// accordingly to tmux.h: Formats
// XXX: check all types, optionality
#[derive(Default, Clone, Debug)]
pub struct Window {
    /// 1 if window active
    pub active: Option<bool>,
    /// Time of window last activity
    pub activity: Option<Duration>,
    /// 1 if window has activity
    pub activity_flag: Option<bool>,
    /// 1 if window has bell
    pub bell_flag: Option<bool>,
    /// 1 if window is larger than client
    pub bigger: Option<bool>,
    /// 1 if window has the highest index
    pub end_flag: Option<bool>,
    /// #F Window flags
    pub flags: Option<WindowFlag>,
    /// 1 if format is for a window (not assuming the current)
    pub format: Option<bool>,
    /// Height of window
    pub height: Option<usize>,
    /// Unique window ID
    pub id: Option<usize>,
    /// #I Index of window
    pub index: Option<usize>,
    /// 1 if window is the last used
    pub last_flag: Option<bool>,
    /// Window layout description, ignoring zoomed window panes
    pub layout: Option<Layout>,
    /// 1 if window is linked across sessions
    pub linked: Option<bool>,
    /// #W Name of window
    pub name: Option<String>,
    /// X offset into window if larger than client
    pub offset_x: Option<String>,
    /// Y offset into window if larger than client
    pub offset_y: Option<String>,
    /// Number of panes in window
    pub panes: Option<usize>,
    /// 1 if window has silence alert
    pub silence_flag: Option<bool>,
    /// Index in session most recent stack
    pub stack_index: Option<usize>,
    /// 1 if window has the lowest index
    pub start_flag: Option<bool>,
    /// Window layout description, respecting zoomed window panes
    pub visible_layout: Option<Layout>,
    /// Width of window
    pub width: Option<usize>,
    /// 1 if window is zoomed
    pub zoomed_flag: Option<bool>
}


impl FromStr for Window {
    type Err = TmuxInterfaceError;

    // XXX: mb deserialize like serde something?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let wv: Vec<&str> = s.split(WINDOW_VARS_SEPARATOR).collect();
        let mut w = Window::new();
        // XXX: optimize?
        if !wv[0].is_empty() { w.active = wv[0].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[1].is_empty() { w.activity = wv[1].parse().ok().map(Duration::from_millis); }
        if !wv[2].is_empty() { w.activity_flag = wv[2].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[3].is_empty() { w.bell_flag = wv[3].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[4].is_empty() { w.bigger = wv[4].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[5].is_empty() { w.end_flag = wv[5].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[6].is_empty() { w.flags = wv[6].parse().ok(); }
        if !wv[7].is_empty() { w.format = wv[7].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[8].is_empty() { w.height = wv[8].parse().ok(); }
        if !wv[9].is_empty() { w.id = wv[9][1..].parse().ok(); } // skip '@' char
        if !wv[10].is_empty() { w.index = wv[10].parse().ok(); }
        if !wv[11].is_empty() { w.last_flag = wv[11].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[12].is_empty() { w.layout = wv[12].parse().ok(); }
        if !wv[13].is_empty() { w.linked = wv[13].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[14].is_empty() { w.name = wv[14].parse().ok(); }
        if !wv[15].is_empty() { w.offset_x = wv[15].parse().ok(); }
        if !wv[16].is_empty() { w.offset_y = wv[16].parse().ok(); }
        if !wv[17].is_empty() { w.panes = wv[17].parse().ok(); }
        if !wv[18].is_empty() { w.silence_flag = wv[18].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[19].is_empty() { w.stack_index = wv[19].parse().ok(); }
        if !wv[20].is_empty() { w.start_flag = wv[20].parse::<usize>().map(|i| i == 1).ok(); }
        if !wv[21].is_empty() { w.visible_layout = wv[21].parse().ok(); }
        if !wv[22].is_empty() { w.width = wv[22].parse().ok(); }
        if !wv[23].is_empty() { w.zoomed_flag = wv[23].parse::<usize>().map(|i| i == 1).ok(); }
        Ok(w)
    }


}


impl Window {

    pub fn new() -> Self {
        Default::default()
    }

}
