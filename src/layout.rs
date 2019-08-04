use crate::LayoutCell;
use crate::TmuxInterfaceError;
use std::str::FromStr;


#[derive(Default, PartialEq, Clone, Debug)]
pub struct Layout {
    pub checksum: usize,       // layout checksum (ref: layout-custom.c -> layout_checksum())
    pub cell: LayoutCell
}


// NOTE: tmux source: layout_custom.c
// XXX: Optimize?
impl FromStr for Layout {
    type Err = TmuxInterfaceError;

    fn from_str(s: &str) -> Result<Layout, TmuxInterfaceError> {
        let mut layout = Layout::new();
        let ls: Vec<&str> = s.split(',').collect();
        layout.checksum = usize::from_str_radix(ls[0], 16)?;
        layout.cell = ls[1].parse()?;
        Ok(layout)
    }

}


impl Layout {

    pub fn new() -> Self {
        Default::default()
    }

}
