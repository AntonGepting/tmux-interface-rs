#[test]
fn show_generated_struct() {
    use crate::Pane;

    let _pane = Pane {
        ..Default::default()
    };
    //dbg!(_pane);
}

#[test]
fn bitflags() {
    use crate::{PANE_ALL, PANE_NONE};
    let bitflags =
        // _31____________16_15_____________0
        0b_11111111111111111_1111111111111111;
    //println!("{:b}", PANE_ALL);
    //println!("{:b}", &bitflags);
    assert_eq!(bitflags, PANE_ALL);
    assert_eq!(0, PANE_NONE);
}

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
    use crate::Pane;
    use crate::PANE_ALL;

    //let pane_str = "1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0'8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176'asus'0'/dev/pts/2'177";

    let pane_vec = vec![
        // pane_active
        #[cfg(feature = "tmux_1_6")]
        "1",
        // pane_at_bottom
        #[cfg(feature = "tmux_2_6")]
        "0",
        // pane_at_left
        #[cfg(feature = "tmux_2_6")]
        "1",
        // pane_at_right
        #[cfg(feature = "tmux_2_6")]
        "1",
        // pane_at_top
        #[cfg(feature = "tmux_2_6")]
        "1",
        // pane_bottom
        #[cfg(feature = "tmux_2_0")]
        "45",
        // pane_current_command
        #[cfg(feature = "tmux_1_8")]
        "bash",
        // pane_current_path
        #[cfg(feature = "tmux_1_7")]
        "/home/user",
        // pane_dead
        #[cfg(feature = "tmux_1_6")]
        "0",
        // pane_dead_status
        #[cfg(feature = "tmux_2_0")]
        "",
        // pane_format
        #[cfg(feature = "tmux_2_6")]
        "1",
        // pane_height
        #[cfg(feature = "tmux_1_6")]
        "64",
        // pane_id
        #[cfg(feature = "tmux_1_6")]
        "%0",
        // pane_in_mode
        #[cfg(feature = "tmux_1_8")]
        "0",
        // pane_index
        #[cfg(feature = "tmux_1_7")]
        "0",
        // pane_input_off
        #[cfg(feature = "tmux_2_0")]
        "0",
        // pane_left
        #[cfg(feature = "tmux_2_0")]
        "0",
        // pane_marked
        #[cfg(feature = "tmux_3_0")]
        "0",
        // pane_marked_set
        #[cfg(feature = "tmux_3_0")]
        "0",
        // pane_mode
        #[cfg(feature = "tmux_2_5")]
        "",
        // pane_path
        #[cfg(feature = "tmux_3_1")]
        "",
        // pane_pid
        #[cfg(feature = "tmux_1_6")]
        "1945",
        // pane_pipe
        #[cfg(feature = "tmux_2_6")]
        "0",
        // pane_right
        #[cfg(feature = "tmux_2_0")]
        "176",
        // pane_search_string
        #[cfg(feature = "tmux_2_5")]
        "",
        // pane_start_command
        #[cfg(feature = "tmux_1_6")]
        "",
        // pane_start_path
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
        "",
        // pane_synchronized
        #[cfg(feature = "tmux_1_9")]
        "0",
        // pane_tabs
        #[cfg(feature = "tmux_1_8")]
        "",
        // pane_title
        #[cfg(feature = "tmux_1_6")]
        "title",
        // pane_top
        #[cfg(feature = "tmux_2_0")]
        "0",
        // pane_tty
        #[cfg(feature = "tmux_1_6")]
        "/dev/pts/2",
        // pane_width
        #[cfg(feature = "tmux_1_6")]
        "177",
    ];
    //let pane_str = "1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0''asus'0'/dev/pts/2'177";
    let pane_str = pane_vec.join("'");
    let pane = Pane::from_str(&pane_str, PANE_ALL).unwrap();
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(pane.current_path, Some("/home/user".to_string()));
    #[cfg(feature = "tmux_1_6")]
    assert_eq!(pane.tty, Some("/dev/pts/2".to_string()));
}

#[test]
fn bitflag_operations() {
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
    use crate::Pane;
    use crate::PANE_ALL;

    let origin = Pane {
        #[cfg(feature = "tmux_1_6")]
        active: Some(true),
        #[cfg(feature = "tmux_2_6")]
        at_bottom: Some(true),
        #[cfg(feature = "tmux_2_6")]
        at_left: Some(true),
        #[cfg(feature = "tmux_2_6")]
        at_right: Some(true),
        #[cfg(feature = "tmux_2_6")]
        at_top: Some(true),
        #[cfg(feature = "tmux_2_0")]
        bottom: Some(63),
        #[cfg(feature = "tmux_1_8")]
        current_command: Some("bash".to_string()),
        #[cfg(feature = "tmux_1_7")]
        current_path: Some("/home/user".to_string()),
        #[cfg(feature = "tmux_1_6")]
        dead: Some(false),
        #[cfg(feature = "tmux_2_0")]
        dead_status: None,
        #[cfg(feature = "tmux_2_6")]
        format: Some(true),
        #[cfg(feature = "tmux_1_6")]
        height: Some(64),
        #[cfg(feature = "tmux_1_6")]
        id: Some(0),
        #[cfg(feature = "tmux_1_8")]
        in_mode: Some(false),
        #[cfg(feature = "tmux_1_7")]
        index: Some(0),
        #[cfg(feature = "tmux_2_0")]
        input_off: Some(false),
        #[cfg(feature = "tmux_2_0")]
        left: Some(0),
        #[cfg(feature = "tmux_3_0")]
        marked: Some(false),
        #[cfg(feature = "tmux_3_0")]
        marked_set: Some(false),
        #[cfg(feature = "tmux_2_5")]
        mode: None,
        #[cfg(feature = "tmux_3_1")]
        path: None,
        #[cfg(feature = "tmux_1_6")]
        pid: Some(1945),
        #[cfg(feature = "tmux_2_6")]
        pipe: Some(false),
        #[cfg(feature = "tmux_2_0")]
        right: Some(176),
        #[cfg(feature = "tmux_2_5")]
        search_string: None,
        #[cfg(feature = "tmux_1_6")]
        start_command: None,
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
        start_path: None,
        #[cfg(feature = "tmux_1_9")]
        synchronized: Some(false),
        #[cfg(feature = "tmux_1_8")]
        tabs: None,
        #[cfg(feature = "tmux_1_6")]
        title: Some("title".to_string()),
        #[cfg(feature = "tmux_2_0")]
        top: Some(0),
        #[cfg(feature = "tmux_1_6")]
        tty: Some("/dev/pts/2".to_string()),
        #[cfg(feature = "tmux_1_6")]
        width: Some(177),
    };

    let pane_vec = vec![
        // pane_active
        #[cfg(feature = "tmux_1_6")]
        "1",
        // pane_at_bottom
        #[cfg(feature = "tmux_2_6")]
        "1",
        // pane_at_left
        #[cfg(feature = "tmux_2_6")]
        "1",
        // pane_at_right
        #[cfg(feature = "tmux_2_6")]
        "1",
        // pane_at_top
        #[cfg(feature = "tmux_2_6")]
        "1",
        // pane_bottom
        #[cfg(feature = "tmux_2_0")]
        "63",
        // pane_current_command
        #[cfg(feature = "tmux_1_8")]
        "bash",
        // pane_current_path
        #[cfg(feature = "tmux_1_7")]
        "/home/user",
        // pane_dead
        #[cfg(feature = "tmux_1_6")]
        "0",
        // pane_dead_status
        #[cfg(feature = "tmux_2_0")]
        "",
        // pane_format
        #[cfg(feature = "tmux_2_6")]
        "1",
        // pane_height
        #[cfg(feature = "tmux_1_6")]
        "64",
        // pane_id
        #[cfg(feature = "tmux_1_6")]
        "%0",
        // pane_in_mode
        #[cfg(feature = "tmux_1_8")]
        "0",
        // pane_index
        #[cfg(feature = "tmux_1_7")]
        "0",
        // pane_input_off
        #[cfg(feature = "tmux_2_0")]
        "0",
        // pane_left
        #[cfg(feature = "tmux_2_0")]
        "0",
        // pane_marked
        #[cfg(feature = "tmux_3_0")]
        "0",
        // pane_marked_set
        #[cfg(feature = "tmux_3_0")]
        "0",
        // pane_mode
        #[cfg(feature = "tmux_2_5")]
        "",
        // pane_path
        #[cfg(feature = "tmux_3_1")]
        "",
        // pane_pid
        #[cfg(feature = "tmux_1_6")]
        "1945",
        // pane_pipe
        #[cfg(feature = "tmux_2_6")]
        "0",
        // pane_right
        #[cfg(feature = "tmux_2_0")]
        "176",
        // pane_search_string
        #[cfg(feature = "tmux_2_5")]
        "",
        // pane_start_command
        #[cfg(feature = "tmux_1_6")]
        "",
        // pane_start_path
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
        "",
        // pane_synchronized
        #[cfg(feature = "tmux_1_9")]
        "0",
        // pane_tabs
        #[cfg(feature = "tmux_1_8")]
        "",
        // pane_title
        #[cfg(feature = "tmux_1_6")]
        "title",
        // pane_top
        #[cfg(feature = "tmux_2_0")]
        "0",
        // pane_tty
        #[cfg(feature = "tmux_1_6")]
        "/dev/pts/2",
        // pane_width
        #[cfg(feature = "tmux_1_6")]
        "177",
    ];
    //let pane_str = "1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0''asus'0'/dev/pts/2'177";
    //let pane_str = "1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0'8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176'asus'0'/dev/pts/2'177";
    let pane_str = pane_vec.join("'");
    let pane = Pane::from_str(&pane_str, PANE_ALL).unwrap();
    //assert_eq!(pane.current_path, Some("/home/user".to_string()));
    //assert_eq!(pane.tty, Some("/dev/pts/2".to_string()));
    assert_eq!(origin, pane);
}
