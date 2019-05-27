#[test]
fn bool() {
    //let a = "1".parse::<bool>().unwrap();
    let a = "1".parse::<usize>().unwrap();
    let b = if a == 1 {
        true
    } else {
        false
    };
    assert_eq!(b, true);
}


#[test]
fn path_regex() {
    use regex::Regex;
    let regex = Regex::new(r"^(/[^/ ]*)+/?$").unwrap();
    assert!(regex.is_match("/var/log/.asd"));
    assert!(regex.is_match("/var/log"));
    assert!(regex.is_match("/var/log/asdf-asdf"));
    assert!(regex.is_match("/"));
    assert!(regex.is_match("/home/user/Documents/rust/tmux-itf"));
}


#[test]
fn pane_parse() {
    use crate::Pane;
    let pane = Pane::parse("1 0 0 0 0 vim /home/user/Documents/rust/mosaic %0 0 asdf").unwrap();
    assert_eq!(pane.id, 0);
    assert_eq!(pane.current_path, "/home/user/Documents/rust/mosaic");
}


