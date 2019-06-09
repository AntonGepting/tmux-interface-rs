//extern crate tmux_interface;


//#[test]
//fn layout_parse() {
    //use regex::Regex;

    //let regex = Regex::new(r"^'([\w,]*)'$").unwrap();
    //assert!(regex.is_match("'c3bf,177x64,0,0,2'"));
//}


#[test]
fn show_options() {
    use crate::{TmuxInterface, ShowOptions};
    use std::borrow::Cow;
    use regex::Regex;

    let tmux = TmuxInterface::new();
    assert_eq!(tmux.tmux, None);
    let tmux = TmuxInterface::new();
    let show_options = ShowOptions {
        global_options: Some(true),
        option_value: Some(true),
        option: Some(Cow::Borrowed("base-index")),
        ..Default::default()
    };
    let show_options_str = tmux.show_options(&show_options).unwrap();
    let regex = Regex::new(r"^(\d+)\n$").unwrap();
    let caps = regex.captures(&show_options_str).unwrap();
    let base_index = caps[1].parse::<usize>().unwrap();
    assert_eq!(base_index, base_index);
}



