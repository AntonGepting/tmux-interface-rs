use crate::response::window::window::{WINDOW_VARS, WINDOW_VARS_SEPARATOR};
use crate::{Error, TargetSession, TmuxInterface, Window};
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
    pub fn get(target_session: &TargetSession, bitflags: usize) -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let lsw_format = WINDOW_VARS
            .iter()
            .filter(|t| bitflags & t.1 == t.1)
            .map(|t| format!("#{{{}}}", t.0))
            .collect::<Vec<String>>()
            .join(WINDOW_VARS_SEPARATOR);
        let windows_str = tmux.list_windows(None, Some(&lsw_format), Some(target_session))?;
        Windows::from_str(&windows_str, bitflags)
    }

    pub fn from_str(windows_str: &str, bitflags: usize) -> Result<Self, Error> {
        let mut windows: Vec<Window> = Vec::new();
        for line in windows_str.lines() {
            windows.push(Window::from_str(line, bitflags)?);
        }
        Ok(Self(windows))
    }
}
