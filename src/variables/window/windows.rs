use crate::formats::formats::Formats;
use crate::variables::window::window::WINDOW_VARS_SEPARATOR;
use crate::{Error, ListWindows, Tmux, Window};
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

    // XXX: generic
    pub fn get<S: ToString>(target_session: S) -> Result<Self, Error> {
        let mut format = Formats::new();
        format.separator(WINDOW_VARS_SEPARATOR);

        let lsw_format = format.to_string();

        let output = Tmux::new()
            .command(
                ListWindows::new()
                    .format(&lsw_format)
                    .target_session(target_session.to_string()),
            )
            .output()?
            .to_string();

        Windows::from_str(&output)
    }

    pub fn from_str<S: AsRef<str>>(windows_str: S) -> Result<Self, Error> {
        let mut windows = Windows::new();
        for line in windows_str.as_ref().lines() {
            windows.push(Window::from_str(line)?);
        }
        Ok(windows)
    }
}
