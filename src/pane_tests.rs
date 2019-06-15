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
fn tty_parse() {
    use regex::Regex;
    let regex = Regex::new(r"^([\w/]*)$").unwrap();
    assert!(regex.is_match("/dev/pts/2"));
}


#[test]
fn tabs_parse() {
    use regex::Regex;
    let regex = Regex::new(r"^([\w,]*)$").unwrap();
    assert!(regex.is_match("8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176"));
}


#[test]
fn parse() {
    use crate::Pane;

    let pane = Pane::parse("1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0''1945'0'176'''0'8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176'asus'0'/dev/pts/2'177").unwrap();
    assert_eq!(pane.current_path, Some("/home/user".to_string()));
    assert_eq!(pane.tty, Some("/dev/pts/2".to_string()));
}

