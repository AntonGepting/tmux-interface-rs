use crate::{Error, Pane};
//use std::borrow::Cow;
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
