use crate::Error;
use std::str::Chars;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub enum LayoutType {
    LeftRight,
    TopBottom,
    WindowPane,
}

impl Default for LayoutType {
    fn default() -> LayoutType {
        LayoutType::WindowPane
    }
}

impl FromStr for LayoutCell {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let state = LayoutFSMState::X;

        let mut layout: LayoutCell = Default::default();
        layout.fsm(&mut chars, state)?;
        Ok(layout)
    }
}

// NOTE: tmux source: layout_custom.c
// XXX: checksum can be improved using hex crate
// XXX: implemet trait parse FromStr?
#[derive(Default, PartialEq, Clone, Debug)]
pub struct LayoutCell {
    pub x: usize,
    pub y: usize,
    pub x_off: usize,
    pub y_off: usize,
    pub id: Option<usize>,
    pub style: LayoutType,
    pub cells: Option<Vec<LayoutCell>>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum LayoutFSMState {
    X,
    Y,
    XOff,
    YOff,
    Id,
    LeftRight,
    TopBottom,
    EndNested,
    EOL,
    //    Error
}

impl LayoutCell {
    pub fn new(
        x: usize,
        y: usize,
        x_off: usize,
        y_off: usize,
        id: Option<usize>,
        style: LayoutType,
        cells: Option<Vec<LayoutCell>>,
    ) -> Self {
        LayoutCell {
            x,
            y,
            x_off,
            y_off,
            id,
            style,
            cells,
        }
    }

    // TODO: optimization
    pub fn fsm(
        &mut self,
        chars: &mut Chars,
        mut state: LayoutFSMState,
    ) -> Result<LayoutFSMState, Error> {
        let mut child: LayoutCell;
        let mut buff = String::new();
        loop {
            if let Some(chr) = chars.next() {
                match (chr, &state) {
                    // end of x element
                    ('x', LayoutFSMState::X) => {
                        self.x = buff.parse()?;
                        state = LayoutFSMState::Y;
                        buff = String::from("");
                    }
                    // end of y element
                    (',', LayoutFSMState::Y) => {
                        self.y = buff.parse()?;
                        state = LayoutFSMState::XOff;
                        buff = String::from("");
                    }
                    // end of x_off element
                    (',', LayoutFSMState::XOff) => {
                        self.x_off = buff.parse()?;
                        state = LayoutFSMState::YOff;
                        buff = String::from("");
                    }
                    // end of y_off element
                    (',', LayoutFSMState::YOff) => {
                        self.y_off = buff.parse()?;
                        state = LayoutFSMState::Id;
                        buff = String::from("");
                    }
                    // end of id element
                    (',', LayoutFSMState::Id) => {
                        self.id = buff.parse().ok();
                        state = LayoutFSMState::X;
                        break;
                    }
                    // end of {} or [] group
                    (',', LayoutFSMState::EndNested) => {
                        state = LayoutFSMState::X;
                        break;
                    }
                    // end of id element inside [] group
                    (']', LayoutFSMState::Id) => {
                        self.id = buff.parse().ok();
                        state = LayoutFSMState::EndNested;
                        break;
                    }
                    // end of id element inside {} group
                    ('}', LayoutFSMState::Id) => {
                        self.id = buff.parse().ok();
                        state = LayoutFSMState::EndNested;
                        break;
                    }
                    //(' ', s) => { s },
                    // end of y_off element before [] group
                    ('[', LayoutFSMState::YOff) => {
                        self.y_off = buff.parse()?;
                        self.id = None;
                        self.style = LayoutType::TopBottom;
                        self.cells = Some(Vec::new());
                        loop {
                            child = Default::default();
                            // TODO: remove unwrap
                            state = child.fsm(chars, LayoutFSMState::X).unwrap();
                            if let Some(c) = self.cells.as_mut() {
                                c.push(child)
                            }
                            if state == LayoutFSMState::EndNested || state == LayoutFSMState::EOL {
                                break;
                            }
                        }
                    }
                    // end of y_off element before {} group
                    ('{', LayoutFSMState::YOff) => {
                        self.y_off = buff.parse()?;
                        self.id = None;
                        self.style = LayoutType::LeftRight;
                        self.cells = Some(Vec::new());
                        loop {
                            child = Default::default();
                            // TODO: remove unwrap
                            state = child.fsm(chars, LayoutFSMState::X).unwrap();
                            if let Some(c) = self.cells.as_mut() {
                                c.push(child)
                            }
                            if state == LayoutFSMState::EndNested || state == LayoutFSMState::EOL {
                                break;
                            }
                        }
                    }
                    (c, _) => {
                        buff.push(c);
                    }
                }
            } else {
                // end of line and id element
                if state == LayoutFSMState::Id {
                    self.id = buff.parse().ok();
                }
                state = LayoutFSMState::EOL;
                break;
            }
        }

        Ok(state)
    }
}
