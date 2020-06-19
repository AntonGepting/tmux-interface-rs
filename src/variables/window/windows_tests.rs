#[test]
fn parse() {
    use crate::{Windows, WINDOW_ALL};

    //let windows_str = "
    // 1559064235'0'0'0''''1'64'@0'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'2''c3bd,177x64,0,0,0'177'0\n\
    // 1559064235'0'0'0''''1'64'@1'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'2''c3bd,177x64,0,0,0'177'0";
    let window0_vec = vec![
        // window 0
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1559064235",
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
        "@0",
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
    let window1_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1559064235",
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
    let window0_str = window0_vec.join("'");
    let window1_str = window1_vec.join("'");
    let windows_str = format!("{}\n{}", window0_str, window1_str);
    let windows = Windows::from_str(&windows_str, WINDOW_ALL).unwrap();
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(windows[0].id, Some(0));
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(windows[1].id, Some(1));
}

#[test]
fn parse2() {
    use crate::{Windows, WINDOW_ALL};

    //let windows_str = "
    //1559064235'0'0'0''''1'64'@0'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'3''c3bd,177x64,0,0,0'177'0\n\
    //1559064235'0'0'0''''1'64'@1'2'0'8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'0'vim'''2'0'2''8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'177'0\n\
    //1559064235'0'0'0'''-'1'64'@2'3'1'7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]'0'vim'''2'0'1''7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]'177'0\n\
    //1559064235'0'1'0'''*'1'64'@4'4'0'c3c3,177x64,0,0,6'0'bash'''1'0'0''c3c3,177x64,0,0,6'177'0";

    let window0_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1559064235",
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
        "@0",
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
    let window1_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1559064235",
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
        "@1",
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
    let window2_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1559064235",
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
    let window3_vec = vec![
        // window_active
        #[cfg(feature = "tmux_1_6")]
        "1559064235",
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
        "@3",
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
    let window0_str = window0_vec.join("'");
    let window1_str = window1_vec.join("'");
    let window2_str = window2_vec.join("'");
    let window3_str = window3_vec.join("'");
    let windows_str = format!(
        "{}\n{}\n{}\n{}",
        window0_str, window1_str, window2_str, window3_str
    );
    let windows = Windows::from_str(&windows_str, WINDOW_ALL).unwrap();
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(windows[0].id, Some(0));
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(windows[1].id, Some(1));
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(windows[2].id, Some(2));
    #[cfg(feature = "tmux_1_7")]
    assert_eq!(windows[3].id, Some(3));
}
