use crate::{Error, Pane};
//use std::borrow::Cow;
use std::ops::Index;
use std::str::FromStr;

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

impl FromStr for Panes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut panes = Panes::new();
        for line in s.lines() {
            panes.push(Pane::from_str(line)?);
        }
        Ok(panes)
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

    //pub fn find(&self, id: usize) -> Result<Pane, Error> {
    //}
}
