use regex::Regex;


//pane_active pane_at_bottom pane_at_left pane_at_right pane_at_top pane_bottom
//pane_current_command pane_current_path pane_dead pane_dead_status pane_format pane_height
//pane_id pane_in_mode pane_input_off pane_index pane_left pane_mode pane_pid pane_pipe pane_right
//pane_search_string pane_start_command pane_synchronized pane_tabs pane_title pane_top pane_tty
//pane_width
//
pub const LIST_PANES_FORMAT: &str = "#{pane_active} #{pane_at_bottom} #{pane_at_left} \
#{pane_at_right} #{pane_at_top} #{pane_current_command} #{pane_current_path} #{pane_id} \
#{pane_index} #{pane_title}";
// numbered capture groups enclosed in parenthesis (...)
//
// FIXME: regex name can be anything, and other keys should be checked better
pub const PANE_STR_REGEX: &str = r"^(1|0) (1|0) (1|0) (1|0) (1|0) (\w+) ((/[^/ ]*)+/?) %(\d+) (\d+) (\w+)$";

// accordingly to tmux.h: Formats
//
#[derive(Clone, Debug)]
pub struct Pane {
    pub active: bool,
    pub at_bottom: bool,
    pub at_left: bool,
    pub at_right: bool,
    pub at_top: bool,
    //pane_bottom
    pub current_command: String,
    pub current_path: String,
    //pane_dead
    //pane_dead_status
    //pane_format
    //pane_height
    pub id: usize,
    //pane_in_mode
    //pane_input_off
    pub index: usize,
    //pane_left
    //pane_mode
    //pane_pid
    //pane_pipe
    //pane_right
    //pane_search_string
    //pane_start_command
    //pane_synchronized
    //pane_tabs
    pub title: String,
    //pane_top
    //pane_tty
    //pane_width
}


impl Default for Pane {
    fn default() -> Self {
        Pane {
            active: false,
            at_bottom: false,
            at_left: false,
            at_right: false,
            at_top: false,
            current_command: "".to_string(),
            current_path: "".to_string(),
            id: 0,
            index: 0,
            title: "".to_string(),
        }
    }
}


impl Pane {

    pub fn new() -> Pane {
        Default::default()
    }


    // mb deserialize?
    //
    pub fn parse(pane_str: &str) -> Result<Pane, ()> {
        let regex = Regex::new(PANE_STR_REGEX).unwrap();
        let cap = regex.captures(pane_str).unwrap();
        let mut pane = Pane::new();
        pane.active = cap[1].parse::<usize>().map(|i| i == 1).unwrap();
        pane.at_bottom = cap[2].parse::<usize>().map(|i| i == 1).unwrap();
        pane.at_left = cap[3].parse::<usize>().map(|i| i == 1).unwrap();
        pane.at_right = cap[4].parse::<usize>().map(|i| i == 1).unwrap();
        pane.at_top = cap[5].parse::<usize>().map(|i| i == 1).unwrap();
        pane.current_command = cap[6].parse().unwrap();
        pane.current_path = cap[7].parse().unwrap();
        pane.id = cap[9].parse().unwrap();
        pane.index = cap[10].parse().unwrap();
        pane.title = cap[11].parse().unwrap();
        Ok(pane)
    }

}
