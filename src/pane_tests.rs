#[test]
fn bool() {
    //let c = "1".parse::<bool>().unwrap();
    //assert_eq!(c, true);
    let a = "1".parse::<usize>().unwrap();
    let b = if a == 1 { true } else { false };
    assert_eq!(b, true);
}

#[test]
fn parse() {
    use crate::pane::PANE_ALL;
    use crate::Pane;

    let pane_str = "1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0'8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176'asus'0'/dev/pts/2'177";
    let pane = Pane::from_str(pane_str, PANE_ALL).unwrap();
    assert_eq!(pane.current_path, Some("/home/user".to_string()));
    assert_eq!(pane.tty, Some("/dev/pts/2".to_string()));
}

#[test]
fn bitflags() {
    let a = 0b001;
    let b = 0b010;
    let c = 0b100;
    let d = a | b;
    assert!(d & a == a);
    assert!(d & c != c);
}

//#[test]
//fn make_fmt_string() {
//use crate::pane::get_fmt_string;
//use crate::Pane;
//let _a = get_fmt_string(Pane::PANE_ACTIVE | Pane::PANE_AT_LEFT);
//}

#[test]
fn parse2() {
    use crate::pane::PANE_ALL;
    use crate::Pane;

    let origin = Pane {
        active: Some(true),
        at_bottom: Some(true),
        at_left: Some(true),
        at_right: Some(true),
        at_top: Some(true),
        bottom: Some(63),
        current_command: Some("bash".to_string()),
        current_path: Some("/home/user".to_string()),
        dead: Some(false),
        dead_status: None,
        format: Some(true),
        height: Some(64),
        id: Some(0),
        in_mode: Some(false),
        index: Some(0),
        input_off: Some(false),
        left: Some(0),
        marked: Some(false),
        marked_set: Some(false),
        mode: None,
        pid: Some(1945),
        pipe: Some(false),
        right: Some(176),
        search_string: None,
        start_command: None,
        synchronized: Some(false),
        tabs: None,
        title: Some("asus".to_string()),
        top: Some(0),
        tty: Some("/dev/pts/2".to_string()),
        width: Some(177),
    };

    let pane_str = "1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0''asus'0'/dev/pts/2'177";
    //let pane_str = "1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0'8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176'asus'0'/dev/pts/2'177";
    let pane = Pane::from_str(&pane_str, PANE_ALL).unwrap();
    //assert_eq!(pane.current_path, Some("/home/user".to_string()));
    //assert_eq!(pane.tty, Some("/dev/pts/2".to_string()));
    assert_eq!(origin, pane);
}
