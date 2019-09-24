use crate::window::{WINDOW_VARS_REGEX_VEC, WINDOW_VARS_SEPARATOR};
use crate::Error;
use crate::TmuxInterface;
use crate::Window;
use std::ops::Index;
use std::str::FromStr;

pub struct Windows(Vec<Window>);

impl FromStr for Windows {
    type Err = Error;

    fn from_str(windows_str: &str) -> Result<Self, Self::Err> {
        let mut windows: Vec<Window> = Vec::new();
        for line in windows_str.lines() {
            windows.push(line.parse()?);
        }
        Ok(Self(windows))
    }
}

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
    pub fn get(target_session: &str) -> Result<Self, Error> {
        let tmux = TmuxInterface::new();
        let lsw_format = WINDOW_VARS_REGEX_VEC
            .iter()
            .map(|t| format!("#{{{}}}", t))
            .collect::<Vec<String>>()
            .join(WINDOW_VARS_SEPARATOR);
        let windows_str = tmux.list_windows(false, Some(&lsw_format), Some(target_session))?;
        windows_str.parse()
    }
}
