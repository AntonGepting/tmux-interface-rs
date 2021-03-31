use crate::variables::pane::pane::{PANE_VARS, PANE_VARS_SEPARATOR};
use crate::Error;
use crate::TargetWindowExt;
use crate::{ListPanes, Pane};
use std::ops::Index;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Panes(pub Vec<Pane>);

impl IntoIterator for Panes {
    type Item = Pane;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Index<usize> for Panes {
    type Output = Pane;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

// TODO: Option as Result
impl Panes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, pane: Pane) {
        self.0.push(pane);
    }

    pub fn get(target_window: &TargetWindowExt, bitflags: u64) -> Result<Self, Error> {
        let lsp_format = PANE_VARS
            .iter()
            .filter(|t| bitflags & t.1 == t.1)
            .map(|t| format!("#{{{}}}", t.0))
            .collect::<Vec<String>>()
            .join(PANE_VARS_SEPARATOR);
        let target_window_str = target_window.to_string();
        let panes_str = ListPanes::new()
            .format(&lsp_format)
            .target(&target_window_str)
            .output()?
            .to_string();
        Panes::from_str(&panes_str, bitflags)
    }

    pub fn get_all(target_session: &TargetWindowExt, bitflags: u64) -> Result<Self, Error> {
        let lsp_format = PANE_VARS
            .iter()
            .filter(|t| bitflags & t.1 == t.1)
            .map(|t| format!("#{{{}}}", t.0))
            .collect::<Vec<String>>()
            .join(PANE_VARS_SEPARATOR);
        let target_session_str = target_session.to_string();
        let panes_str = ListPanes::new()
            .format(&lsp_format)
            .target(&target_session_str)
            .output()?
            .to_string();
        Panes::from_str(&panes_str, bitflags)
    }

    pub fn from_str(panes_str: &str, bitflags: u64) -> Result<Self, Error> {
        let mut panes = Panes::new();
        for line in panes_str.lines() {
            panes.push(Pane::from_str(line, bitflags)?);
        }
        Ok(panes)
    }

    //pub fn find(&self, id: usize) -> Result<Pane, Error> {
    //}
}
