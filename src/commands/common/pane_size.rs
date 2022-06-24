//use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PaneSize {
    Size(usize),
    Percentage(usize),
}

// XXX: check, need to pass flag -p, -l anyway
// affected commands:
// split_window
// move_pane
// join_pane
//
//impl fmt::Display for PaneSize {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//// `[-l size]` - specify the size of the new pane in lines
//// `[-l size | -p percentage]` - specify the size of the new pane in lines
//#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
//let s = match self {
//PaneSize::Size(size) => size.to_string(),
//PaneSize::Percentage(size) => {
//format!("{}%", size)
//}
//};

//#[cfg(feature = "tmux_3_1")]
//let s = match self {
//PaneSize::Size(size) => size.to_string(),
//PaneSize::Percentage(size) => size.to_string(),
//};

//write!(f, "{}", s)
//}
//}
