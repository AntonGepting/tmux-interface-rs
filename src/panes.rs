use crate::Pane;


pub struct Panes {
    //sessions: Vec<Pane>
}


impl Panes {
    pub fn parse(panes_str: &str) -> Result<Vec<Pane>, ()> {
        let mut panes: Vec<Pane> = Vec::new();
        for line in panes_str.lines() {
            panes.push(Pane::parse(line).unwrap());
        }
        Ok(panes)
    }
}

