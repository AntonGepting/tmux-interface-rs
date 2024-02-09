#[test]
fn clients_parse() {
    use crate::Clients;
    use std::str::FromStr;

    let client0_vec = vec![
        // client_activity
        #[cfg(feature = "tmux_1_6")]
        "1707508743",
        // client_cell_height
        #[cfg(feature = "tmux_3_1")]
        "0",
        // client_cell_width
        #[cfg(feature = "tmux_3_1")]
        "0",
        // client_activity_string
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        "",
        // client_created
        #[cfg(feature = "tmux_1_6")]
        "1707479629",
        // client_created_string
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        "",
        // client_control_mode
        #[cfg(feature = "tmux_2_1")]
        "0",
        // client_discarded
        #[cfg(feature = "tmux_2_1")]
        "0",
        // client_flags:attached,focused,UTF-8
        // client_cwd
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        "",
        // client_termfeatures:bpaste,ccolour,clipboard,cstyle,focus,title
        // client_height
        #[cfg(feature = "tmux_1_6")]
        "65",
        // client_key_table
        #[cfg(feature = "tmux_2_2")]
        "root",
        // client_last_session
        #[cfg(feature = "tmux_1_8")]
        "",
        // client_name
        #[cfg(feature = "tmux_2_4")]
        "/dev/pts/0",
        // client_pid
        #[cfg(feature = "tmux_2_1")]
        "32150",
        // client_prefix
        #[cfg(feature = "tmux_1_8")]
        "0",
        // client_readonly
        #[cfg(feature = "tmux_1_6")]
        "0",
        // client_session
        #[cfg(feature = "tmux_1_8")]
        "0",
        // client_termname
        #[cfg(feature = "tmux_1_6")]
        "xterm-256color",
        // client_termtype
        #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
        "",
        // client_uid:1000
        // client_user:anton
        // client_tty
        #[cfg(feature = "tmux_1_6")]
        "/dev/pts/0",
        // client_utf8
        #[cfg(feature = "tmux_1_6")]
        "1",
        // client_width
        #[cfg(feature = "tmux_1_6")]
        "177",
        // client_written
        #[cfg(feature = "tmux_2_4")]
        "193354",
    ];
    let client1_vec = vec![
        // client_activity
        #[cfg(feature = "tmux_1_6")]
        "1707508743",
        // client_cell_height
        #[cfg(feature = "tmux_3_1")]
        "0",
        // client_cell_width
        #[cfg(feature = "tmux_3_1")]
        "0",
        // client_activity_string
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        "",
        // client_created
        #[cfg(feature = "tmux_1_6")]
        "1707479629",
        // client_created_string
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        "",
        // client_control_mode
        #[cfg(feature = "tmux_2_1")]
        "0",
        // client_discarded
        #[cfg(feature = "tmux_2_1")]
        "0",
        // client_flags:attached,focused,UTF-8
        // client_cwd
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        "",
        // client_termfeatures:bpaste,ccolour,clipboard,cstyle,focus,title
        // client_height
        #[cfg(feature = "tmux_1_6")]
        "65",
        // client_key_table
        #[cfg(feature = "tmux_2_2")]
        "root",
        // client_last_session
        #[cfg(feature = "tmux_1_8")]
        "",
        // client_name
        #[cfg(feature = "tmux_2_4")]
        "/dev/pts/0",
        // client_pid
        #[cfg(feature = "tmux_2_1")]
        "32150",
        // client_prefix
        #[cfg(feature = "tmux_1_8")]
        "0",
        // client_readonly
        #[cfg(feature = "tmux_1_6")]
        "0",
        // client_session
        #[cfg(feature = "tmux_1_8")]
        "0",
        // client_termname
        #[cfg(feature = "tmux_1_6")]
        "xterm-256color",
        // client_termtype
        #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
        "",
        // client_uid:1000
        // client_user:anton
        // client_tty
        #[cfg(feature = "tmux_1_6")]
        "/dev/pts/0",
        // client_utf8
        #[cfg(feature = "tmux_1_6")]
        "1",
        // client_width
        #[cfg(feature = "tmux_1_6")]
        "177",
        // client_written
        #[cfg(feature = "tmux_2_4")]
        "193354",
    ];
    let client0_str = client0_vec.join(":");
    let client1_str = client1_vec.join(":");
    let clients_str = format!("{}\n{}", client0_str, client1_str);
    let clients = Clients::from_str(&clients_str).unwrap();

    dbg!(clients);
}
