#[test]
fn show_generated_struct() {
    use crate::Window;

    let _window = Window {
        ..Default::default()
    };
    //dbg!(_window);
}

#[test]
fn bitflags() {
    use crate::{WINDOW_ALL, WINDOW_NONE};
    let bitflags =
        // 35___31_____________1615_____________0
        0b_1111_1111111111111111_1111111111111111;
    //println!("{:b}", WINDOW_ALL);
    //println!("{:b}", &bitflags);
    assert_eq!(bitflags, WINDOW_ALL);
    assert_eq!(0, WINDOW_NONE);
}

//let window_str = "1557947146'0'1'0'''*'1'64'@0'4'0'3484,177x64,0,0{88x64,0,0,3,88x64,89,0,18}'0'bash'''2'0'0''3484,177x64,0,0{88x64,0,0,3,88x64,89,0,18}'177'0";
#[test]
fn parse1() {
    use crate::Window;
    use crate::WINDOW_ALL;

    let window_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1557947146",
        // window_active_clients
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_clients_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_activity
        #[cfg(feature = "tmux_2_1")]
        "0",
        // session_activity_string
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        "",
        // window_activity_flag
        #[cfg(any(
            all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
            feature = "tmux_2_3"
        ))]
        "",
        // window_bell_flag
        #[cfg(feature = "tmux_1_9")]
        "1",
        // window_content_flag
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        "",
        // 1 if window is larger than client
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_cell_height
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_cell_width
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_end_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_find_matches
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        "",
        // window_flags
        #[cfg(feature = "tmux_1_6")]
        "*",
        // window_format
        #[cfg(feature = "tmux_2_6")]
        "1",
        // window_height
        #[cfg(feature = "tmux_1_6")]
        "64",
        // window_id
        #[cfg(feature = "tmux_1_7")]
        "@0",
        // window_index
        #[cfg(feature = "tmux_1_6")]
        "4",
        // window_last_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
        // window_layout
        #[cfg(feature = "tmux_1_6")]
        "3484,177x64,0,0{88x64,0,0,3,88x64,89,0,18}",
        // window_linked
        #[cfg(feature = "tmux_2_1")]
        "",
        // window_linked_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_linked_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_marked_flag
        #[cfg(feature = "tmux_3_1")]
        "0",
        // window_name
        #[cfg(feature = "tmux_1_6")]
        "bash",
        // window_offset_x
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_offset_y
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_panes
        #[cfg(feature = "tmux_1_7")]
        "2",
        // window_silence_flag
        #[cfg(feature = "tmux_1_9")]
        "0",
        // window_stack_index
        #[cfg(feature = "tmux_2_5")]
        "0",
        // window_start_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_visible_layout
        #[cfg(feature = "tmux_2_2")]
        "3484,177x64,0,0{88x64,0,0,3,88x64,89,0,18}",
        // window_width
        #[cfg(feature = "tmux_1_6")]
        "177",
        // window_zoomed_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
    ];
    let window_str = window_vec.join("'");
    //println!("{}", window_str);

    let window = Window::from_str(&window_str, WINDOW_ALL).unwrap();
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(window.id, Some(0));
}

//let window_str = "1557947146'0'0'0'''*'1'64'@1'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'2''c3bd,177x64,0,0,0'177'0";
#[test]
fn parse2() {
    use crate::Window;
    use crate::WINDOW_ALL;

    let window_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1557947146",
        // window_active_clients
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_clients_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_activity
        #[cfg(feature = "tmux_2_1")]
        "0",
        // session_activity_string
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        "",
        // window_activity_flag
        #[cfg(any(
            all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
            feature = "tmux_2_3"
        ))]
        "",
        // window_bell_flag
        #[cfg(feature = "tmux_1_9")]
        "1",
        // window_content_flag
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        "",
        // 1 if window is larger than client
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_cell_height
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_cell_width
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_end_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_find_matches
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        "",
        // window_flags
        #[cfg(feature = "tmux_1_6")]
        "*",
        // window_format
        #[cfg(feature = "tmux_2_6")]
        "1",
        // window_height
        #[cfg(feature = "tmux_1_6")]
        "64",
        // window_id
        #[cfg(feature = "tmux_1_7")]
        "@1",
        // window_index
        #[cfg(feature = "tmux_1_6")]
        "4",
        // window_last_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
        // window_layout
        #[cfg(feature = "tmux_1_6")]
        "c3bd,177x64,0,0,0",
        // window_linked
        #[cfg(feature = "tmux_2_1")]
        "",
        // window_linked_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_linked_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_marked_flag
        #[cfg(feature = "tmux_3_1")]
        "0",
        // window_name
        #[cfg(feature = "tmux_1_6")]
        "bash",
        // window_offset_x
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_offset_y
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_panes
        #[cfg(feature = "tmux_1_7")]
        "2",
        // window_silence_flag
        #[cfg(feature = "tmux_1_9")]
        "0",
        // window_stack_index
        #[cfg(feature = "tmux_2_5")]
        "0",
        // window_start_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_visible_layout
        #[cfg(feature = "tmux_2_2")]
        "c3bd,177x64,0,0,0",
        // window_width
        #[cfg(feature = "tmux_1_6")]
        "177",
        // window_zoomed_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
    ];
    let window_str = window_vec.join("'");
    let window = Window::from_str(&window_str, WINDOW_ALL).unwrap();
    #[cfg(feature = "tmux_1_6")]
    assert_eq!(window.name, Some("bash".to_string()));
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(window.id, Some(1));
}

//let window_str = "1557947146'0'0'0''''1'64'@2'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'3''c3bd,177x64,0,0,0'177'0";
#[test]
fn parse3() {
    use crate::Window;
    use crate::WINDOW_ALL;

    let window_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1557947146",
        // window_active_clients
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_clients_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_activity
        #[cfg(feature = "tmux_2_1")]
        "0",
        // session_activity_string
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        "",
        // window_activity_flag
        #[cfg(any(
            all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
            feature = "tmux_2_3"
        ))]
        "",
        // window_bell_flag
        #[cfg(feature = "tmux_1_9")]
        "1",
        // window_content_flag
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        "",
        // 1 if window is larger than client
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_cell_height
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_cell_width
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_end_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_find_matches
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        "",
        // window_flags
        #[cfg(feature = "tmux_1_6")]
        "",
        // window_format
        #[cfg(feature = "tmux_2_6")]
        "1",
        // window_height
        #[cfg(feature = "tmux_1_6")]
        "64",
        // window_id
        #[cfg(feature = "tmux_1_7")]
        "@2",
        // window_index
        #[cfg(feature = "tmux_1_6")]
        "4",
        // window_last_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
        // window_layout
        #[cfg(feature = "tmux_1_6")]
        "c3bd,177x64,0,0,0",
        // window_linked
        #[cfg(feature = "tmux_2_1")]
        "",
        // window_linked_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_linked_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_marked_flag
        #[cfg(feature = "tmux_3_1")]
        "0",
        // window_name
        #[cfg(feature = "tmux_1_6")]
        "bash",
        // window_offset_x
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_offset_y
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_panes
        #[cfg(feature = "tmux_1_7")]
        "2",
        // window_silence_flag
        #[cfg(feature = "tmux_1_9")]
        "0",
        // window_stack_index
        #[cfg(feature = "tmux_2_5")]
        "0",
        // window_start_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_visible_layout
        #[cfg(feature = "tmux_2_2")]
        "c3bd,177x64,0,0,0",
        // window_width
        #[cfg(feature = "tmux_1_6")]
        "177",
        // window_zoomed_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
    ];
    let window_str = window_vec.join("'");
    let window = Window::from_str(&window_str, WINDOW_ALL).unwrap();
    #[cfg(feature = "tmux_1_6")]
    assert_eq!(window.name, Some("bash".to_string()));
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(window.id, Some(2));
}

//let window_str = "1557947146'0'0'0''''1'64'@3'2'0'8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'0'vim'''2'0'2''8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'177'0";
#[test]
fn parse4() {
    use crate::Window;
    use crate::WINDOW_ALL;

    let window_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1557947146",
        // window_active_clients
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_clients_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_activity
        #[cfg(feature = "tmux_2_1")]
        "0",
        // session_activity_string
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        "",
        // window_activity_flag
        #[cfg(any(
            all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
            feature = "tmux_2_3"
        ))]
        "",
        // window_bell_flag
        #[cfg(feature = "tmux_1_9")]
        "1",
        // window_content_flag
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        "",
        // 1 if window is larger than client
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_cell_height
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_cell_width
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_end_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_find_matches
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        "",
        // window_flags
        #[cfg(feature = "tmux_1_6")]
        "*",
        // window_format
        #[cfg(feature = "tmux_2_6")]
        "1",
        // window_height
        #[cfg(feature = "tmux_1_6")]
        "64",
        // window_id
        #[cfg(feature = "tmux_1_7")]
        "@3",
        // window_index
        #[cfg(feature = "tmux_1_6")]
        "4",
        // window_last_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
        // window_layout
        #[cfg(feature = "tmux_1_6")]
        "8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]",
        // window_linked
        #[cfg(feature = "tmux_2_1")]
        "",
        // window_linked_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_linked_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_marked_flag
        #[cfg(feature = "tmux_3_1")]
        "0",
        // window_name
        #[cfg(feature = "tmux_1_6")]
        "bash",
        // window_offset_x
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_offset_y
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_panes
        #[cfg(feature = "tmux_1_7")]
        "2",
        // window_silence_flag
        #[cfg(feature = "tmux_1_9")]
        "0",
        // window_stack_index
        #[cfg(feature = "tmux_2_5")]
        "0",
        // window_start_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_visible_layout
        #[cfg(feature = "tmux_2_2")]
        "8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]",
        // window_width
        #[cfg(feature = "tmux_1_6")]
        "177",
        // window_zoomed_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
    ];
    let window_str = window_vec.join("'");
    let window = Window::from_str(&window_str, WINDOW_ALL).unwrap();
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(window.id, Some(3));
}

//let window_str = "1557947146'0'0'0'''-'1'64'@4'3'1'7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]'0'vim'''2'0'1''7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]'177'0";
#[test]
fn parse5() {
    use crate::Window;
    use crate::WINDOW_ALL;

    let window_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1557947146",
        // window_active_clients
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_clients_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_activity
        #[cfg(feature = "tmux_2_1")]
        "0",
        // session_activity_string
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        "",
        // window_activity_flag
        #[cfg(any(
            all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
            feature = "tmux_2_3"
        ))]
        "",
        // window_bell_flag
        #[cfg(feature = "tmux_1_9")]
        "1",
        // window_content_flag
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        "",
        // 1 if window is larger than client
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_cell_height
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_cell_width
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_end_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_find_matches
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        "",
        // window_flags
        #[cfg(feature = "tmux_1_6")]
        "-",
        // window_format
        #[cfg(feature = "tmux_2_6")]
        "1",
        // window_height
        #[cfg(feature = "tmux_1_6")]
        "64",
        // window_id
        #[cfg(feature = "tmux_1_7")]
        "@4",
        // window_index
        #[cfg(feature = "tmux_1_6")]
        "4",
        // window_last_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
        // window_layout
        #[cfg(feature = "tmux_1_6")]
        "7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]",
        // window_linked
        #[cfg(feature = "tmux_2_1")]
        "",
        // window_linked_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_linked_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_marked_flag
        #[cfg(feature = "tmux_3_1")]
        "0",
        // window_name
        #[cfg(feature = "tmux_1_6")]
        "bash",
        // window_offset_x
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_offset_y
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_panes
        #[cfg(feature = "tmux_1_7")]
        "2",
        // window_silence_flag
        #[cfg(feature = "tmux_1_9")]
        "0",
        // window_stack_index
        #[cfg(feature = "tmux_2_5")]
        "0",
        // window_start_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_visible_layout
        #[cfg(feature = "tmux_2_2")]
        "7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]",
        // window_width
        #[cfg(feature = "tmux_1_6")]
        "177",
        // window_zoomed_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
    ];
    let window_str = window_vec.join("'");
    let window = Window::from_str(&window_str, WINDOW_ALL).unwrap();
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(window.id, Some(4));
}

//let window_str = "1557947146'0'1'0'''*'1'64'@5'4'0'c3c3,177x64,0,0,6'0'bash'''1'0'0''c3c3,177x64,0,0,6'177'0";
#[test]
fn parse6() {
    use crate::Window;
    use crate::WINDOW_ALL;

    let window_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1557947146",
        // window_active_clients
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_clients_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_active_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_activity
        #[cfg(feature = "tmux_2_1")]
        "0",
        // session_activity_string
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        "",
        // window_activity_flag
        #[cfg(any(
            all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
            feature = "tmux_2_3"
        ))]
        "",
        // window_bell_flag
        #[cfg(feature = "tmux_1_9")]
        "1",
        // window_content_flag
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        "",
        // 1 if window is larger than client
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_cell_height
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_cell_width
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_end_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_find_matches
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        "",
        // window_flags
        #[cfg(feature = "tmux_1_6")]
        "*",
        // window_format
        #[cfg(feature = "tmux_2_6")]
        "1",
        // window_height
        #[cfg(feature = "tmux_1_6")]
        "64",
        // window_id
        #[cfg(feature = "tmux_1_7")]
        "@5",
        // window_index
        #[cfg(feature = "tmux_1_6")]
        "4",
        // window_last_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
        // window_layout
        #[cfg(feature = "tmux_1_6")]
        "c3c3,177x64,0,0,6",
        // window_linked
        #[cfg(feature = "tmux_2_1")]
        "",
        // window_linked_sessions
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_linked_sessions_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // window_marked_flag
        #[cfg(feature = "tmux_3_1")]
        "0",
        // window_name
        #[cfg(feature = "tmux_1_6")]
        "bash",
        // window_offset_x
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_offset_y
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_panes
        #[cfg(feature = "tmux_1_7")]
        "2",
        // window_silence_flag
        #[cfg(feature = "tmux_1_9")]
        "0",
        // window_stack_index
        #[cfg(feature = "tmux_2_5")]
        "0",
        // window_start_flag
        #[cfg(feature = "tmux_2_9")]
        "",
        // window_visible_layout
        #[cfg(feature = "tmux_2_2")]
        "c3c3,177x64,0,0,6",
        // window_width
        #[cfg(feature = "tmux_1_6")]
        "177",
        // window_zoomed_flag
        #[cfg(feature = "tmux_2_0")]
        "0",
    ];
    let window_str = window_vec.join("'");
    let window = Window::from_str(&window_str, WINDOW_ALL).unwrap();
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(window.id, Some(5));
}

//#[test]
//fn parse7() {
//use crate::response::window::window::WINDOW_ALL;
//use crate::Window;

// FIXME: name with dots
//let window = Window::from_str("1557947146 0 0 0 @0 1 python3.7 0").unwrap();
//assert_eq!(window.name, "asdf");
//assert_eq!(window.id, 0);
//}
