use crate::format::format::Format;
use crate::variables::window::window::WINDOW_VARS_SEPARATOR;
use crate::{Error, ListWindows, TargetSession, Window};
use std::ops::Index;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Windows(pub Vec<Window>);

impl IntoIterator for Windows {
    type Item = Window;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Index<usize> for Windows {
    type Output = Window;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl Windows {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, window: Window) {
        self.0.push(window);
    }

    pub fn get(target_session: &TargetSession) -> Result<Self, Error> {
        let mut format = Format::new();
        format.separator(WINDOW_VARS_SEPARATOR);

        let target_session_str = target_session.to_string();
        let lsw_format = format.to_string();

        let output = ListWindows::new()
            .format(&lsw_format)
            .target_session(&target_session_str)
            .output()?
            .to_string();

        Windows::from_str(&output)
    }

    pub fn from_str(windows_str: &str) -> Result<Self, Error> {
        let mut windows = Windows::new();
        for line in windows_str.lines() {
            windows.push(Window::from_str(line)?);
        }
        Ok(windows)
    }
}
