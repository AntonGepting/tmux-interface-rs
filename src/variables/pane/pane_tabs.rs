use crate::Error;
use std::str::FromStr;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct PaneTabs(pub Vec<usize>);

impl FromStr for PaneTabs {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let a: Vec<usize> = s.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let mut tabs = Vec::new();
        for tab in s.split(',').collect::<Vec<&str>>() {
            tabs.push(tab.parse()?);
        }
        Ok(Self(tabs))
    }
}
