use crate::Window;


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

