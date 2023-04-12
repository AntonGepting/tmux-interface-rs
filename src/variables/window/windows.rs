use crate::{Error, Window};
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

    pub fn from_str<S: AsRef<str>>(windows_str: S) -> Result<Self, Error> {
        let mut windows = Windows::new();
        for line in windows_str.as_ref().lines() {
            windows.push(Window::from_str(line)?);
        }
        Ok(windows)
    }
}
