use crate::Error;
use std::str::FromStr;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct SessionStack(pub Vec<usize>);

impl FromStr for SessionStack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let a: Vec<usize> = s.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let mut sv = Vec::new();
        for id in s.split(',').collect::<Vec<&str>>() {
            sv.push(id.parse()?);
        }
        Ok(Self(sv))
    }
}
