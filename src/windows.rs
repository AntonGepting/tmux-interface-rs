use crate::Window;
use crate::TmuxInterface;
use crate::window::{WINDOW_VARS_REGEX_VEC, WINDOW_VARS_SEPARATOR};


pub struct Windows {
    //sessions: Vec<Window>
}


impl Windows {

    pub fn get(target_session: &str) -> Result<Vec<Window>, ()> {
        let tmux = TmuxInterface::new();
        let lsw_format = WINDOW_VARS_REGEX_VEC.iter().map(|t| format!("#{{{}}}", t.0))
            .collect::<Vec<String>>().join(WINDOW_VARS_SEPARATOR);
        let windows_str = tmux.list_windows(false, Some(&lsw_format), Some(target_session)).unwrap();
        Windows::parse(&windows_str)
    }


    pub fn parse(windows_str: &str) -> Result<Vec<Window>, ()> {
        let mut windows: Vec<Window> = Vec::new();
        for line in windows_str.lines() {
            windows.push(Window::parse(line).unwrap());
        }
        Ok(windows)
    }
}

