#[test]
fn parse() {
    use crate::{Panes, PANE_ALL};

    //"1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0'8,16,24,\
    // 32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176'asus'0'\
    // /dev/pts/2'177\n"

    let pane0_vec = vec![
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
        "",
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
        "8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176",
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
    //"1'0'1'1'1'45'vim'/home/user/Documents/abc/cde-fgh/ijk'0''1'46'%1'0'0'0'0'0'0\
    // ''13587'0'176'''0'8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176'asus'0'/dev/pts/3'1771";
    let pane1_vec = vec![
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
        "vim",
        // pane_current_path
        #[cfg(feature = "tmux_1_7")]
        "/home/user/Documents/abc/cde-fgh/ijk",
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
        "%1",
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
        "8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176",
        // pane_title
        #[cfg(feature = "tmux_1_6")]
        "title",
        // pane_top
        #[cfg(feature = "tmux_2_0")]
        "0",
        // pane_tty
        #[cfg(feature = "tmux_1_6")]
        "/dev/pts/3",
        // pane_width
        #[cfg(feature = "tmux_1_6")]
        "177",
    ];
    let pane0_str = pane0_vec.join("'");
    let pane1_str = pane1_vec.join("'");
    let panes_str = format!("{}\n{}", pane0_str, pane1_str);
    let panes = Panes::from_str(&panes_str, PANE_ALL).unwrap();
    assert_eq!(panes[0].id, Some(0));
}
