use crate::format::format::Format;
use crate::Error;
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

    // XXX: generic
    pub fn get<S: ToString>(target_window: S) -> Result<Self, Error> {
        let mut format = Format::new();
        format.separator(':');

        let lsp_format = format.to_string();
        let output = ListPanes::new()
            .format(&lsp_format)
            .target(target_window.to_string())
            .build()
            .output()?
            .to_string();

        Panes::from_str(&output)
    }

    // XXX: generic
    pub fn get_all<S: ToString>(target_session: S) -> Result<Self, Error> {
        let mut format = Format::new();
        format.separator(':');

        let lsp_format = format.to_string();
        let output = ListPanes::new()
            .format(&lsp_format)
            .target(target_session.to_string())
            .build()
            .output()?
            .to_string();

        Panes::from_str(&output)
    }

    pub fn from_str<S: AsRef<str>>(panes_str: S) -> Result<Self, Error> {
        let mut panes = Panes::new();
        for line in panes_str.as_ref().lines() {
            panes.push(Pane::from_str(line)?);
        }
        Ok(panes)
    }

    //pub fn find(&self, id: usize) -> Result<Pane, Error> {
    //}
}
