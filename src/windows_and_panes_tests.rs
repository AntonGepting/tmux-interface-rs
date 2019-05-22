//#[test]
//fn list_windows() {
    //use crate::tmux_interface::TmuxInterface;
    //use crate::window::Windows;
    //use crate::LIST_WINDOWS_FORMAT;

    //let tmux = TmuxInterface::new(None);
    //let windows_str = tmux.list_windows(false, Some(LIST_WINDOWS_FORMAT), Some("0")).unwrap();
    //let windows = Windows::parse(&windows_str).unwrap();
    //assert_eq!(windows[0].id, 0);
//}


//#[test]
//fn list_panes() {
    //use crate::tmux_interface::TmuxInterface;
    //use crate::pane::Panes;
    //use crate::LIST_PANES_FORMAT;

    //let tmux = TmuxInterface::new(None);
    //let panes_str = tmux.list_panes(false, true, Some(LIST_PANES_FORMAT), Some("0")).unwrap();
    //dbg!(&panes_str);
    //let panes = Panes::parse(&panes_str).unwrap();
    //dbg!(&panes);
    //assert_eq!(panes[0].id, 0);
//}


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


#[test]
fn panes_parse() {
    use crate::Panes;

    let panes = Panes::parse("1 0 0 0 0 vim /var/log %0 0 asdf\n1 0 0 0 0 top /var/log %1 1 asdf").unwrap();
    assert_eq!(panes[0].id, 0);
}

#[test]
fn window_parse() {
    use crate::Window;

    let window = Window::parse("1557947146 0 0 0 @0 1 asdf 0").unwrap();
    assert_eq!(window.name, "asdf");
    assert_eq!(window.id, 0);
    // FIXME: name with dots
    //let window = Window::from_str("1557947146 0 0 0 @0 1 python3.7 0").unwrap();
    //assert_eq!(window.name, "asdf");
    //assert_eq!(window.id, 0);
}


#[test]
fn windows_parse() {
    use crate::Windows;

    let windows = Windows::parse("1557947146 0 0 0 @0 1 asdf 0\n1557947146 0 0 0 @1 2 fdas 0").unwrap();
    assert_eq!(windows[0].id, 0);
}
