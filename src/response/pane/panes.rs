use crate::response::pane::pane::{PANE_VARS_REGEX_VEC, PANE_VARS_SEPARATOR};
use crate::Error;
use crate::Pane;
use crate::TmuxInterface;
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
    pub fn get(target_window: &str, bitflags: usize) -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let lsp_format = PANE_VARS_REGEX_VEC
            .iter()
            .filter(|t| bitflags & t.1 == t.1)
            .map(|t| format!("#{{{}}}", t.0))
            .collect::<Vec<String>>()
            .join(PANE_VARS_SEPARATOR);
        let panes_str = tmux.list_panes(None, None, Some(&lsp_format), Some(target_window))?;
        Panes::from_str(&panes_str, bitflags)
    }

    pub fn get_all(target_session: &str, bitflags: usize) -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let lsp_format = PANE_VARS_REGEX_VEC
            .iter()
            .filter(|t| bitflags & t.1 == t.1)
            .map(|t| format!("#{{{}}}", t.0))
            .collect::<Vec<String>>()
            .join(PANE_VARS_SEPARATOR);
        let panes_str =
            tmux.list_panes(Some(true), None, Some(&lsp_format), Some(target_session))?;
        Panes::from_str(&panes_str, bitflags)
    }

    pub fn from_str(panes_str: &str, bitflags: usize) -> Result<Self, Error> {
        let mut panes: Vec<Pane> = Vec::new();
        for line in panes_str.lines() {
            panes.push(Pane::from_str(line, bitflags)?);
        }
        Ok(Self(panes))
    }

    //pub fn find(&self, id: usize) -> Result<Pane, Error> {
    //}
}
