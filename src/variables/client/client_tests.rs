#[test]
fn client_default() {
    use crate::Client;

    let client = Client {
        ..Default::default()
    };

    let client_orig = Client {
        #[cfg(feature = "tmux_1_6")]
        activity: None,
        #[cfg(feature = "tmux_3_1")]
        cell_height: None,
        #[cfg(feature = "tmux_3_1")]
        cell_width: None,
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        activity_string: None,
        #[cfg(feature = "tmux_1_6")]
        created: None,
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        created_string: None,
        #[cfg(feature = "tmux_2_1")]
        control_mode: None,
        #[cfg(feature = "tmux_2_1")]
        discarded: None,
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        cwd: None,
        #[cfg(feature = "tmux_1_6")]
        height: None,
        #[cfg(feature = "tmux_2_2")]
        key_table: None,
        #[cfg(feature = "tmux_1_8")]
        last_session: None,
        #[cfg(feature = "tmux_2_4")]
        name: None,
        #[cfg(feature = "tmux_2_1")]
        pid: None,
        #[cfg(feature = "tmux_1_8")]
        prefix: None,
        #[cfg(feature = "tmux_1_6")]
        readonly: None,
        #[cfg(feature = "tmux_1_8")]
        session: None,
        #[cfg(feature = "tmux_1_6")]
        termname: None,
        #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
        termtype: None,
        #[cfg(feature = "tmux_1_6")]
        tty: None,
        #[cfg(feature = "tmux_1_6")]
        utf8: None,
        #[cfg(feature = "tmux_1_6")]
        width: None,
        #[cfg(feature = "tmux_2_4")]
        written: None,
    };

    assert_eq!(client_orig, client);
}

// `tmux lsc -F "#{client_activity}:#{client_cell_height}:#{client_cell_width}:#{client_activity_string}:#{client_created}:#{client_created_string}:#{client_control_mode}:#{client_discarded}:#{client_cwd}:#{client_height}:#{client_key_table}:#{client_last_session}:#{client_name}:#{client_pid}#{client_prefix}:#{client_readonly}:#{client_session}:#{client_termname}:#{client_termtype}:#{client_tty}:#{client_utf8}:#{client_width}:#{client_written}"`
// client_activity:1707509930;client_cell_height:0;client_cell_width:0;client_control_mode:0;client_created:1707479629;client_discarded:0;client_flags:attached,focused,UTF-8;client_height:65;client_key_table:root;client_last_session:;client_name:/dev/pts/0;client_pid:3215;client_prefix:0;client_readonly:0;client_session:0;client_termfeatures:bpaste,ccolour,clipboard,cstyle,focus,title;client_termname:xterm-256color;client_termtype:;client_tty:/dev/pts/0;client_uid:1000;client_user:anton;client_utf8:1;client_width:177;client_written:206823
#[test]
fn client_parse() {
    use crate::Client;
    use std::str::FromStr;

    let client_vec = vec![
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
    let client_str = client_vec.join(":");
    let client = Client::from_str(&client_str).unwrap();

    let client_orig = Client {
        #[cfg(feature = "tmux_1_6")]
        activity: Some(1707508743),
        #[cfg(feature = "tmux_3_1")]
        cell_height: Some(0),
        #[cfg(feature = "tmux_3_1")]
        cell_width: Some(0),
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        activity_string: None,
        #[cfg(feature = "tmux_1_6")]
        created: Some(1707479629),
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        created_string: None,
        #[cfg(feature = "tmux_2_1")]
        control_mode: Some(false),
        #[cfg(feature = "tmux_2_1")]
        discarded: Some("0".to_string()),
        // client_flags:attached,focused,UTF-8
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        cwd: None,
        // client_termfeatures:bpaste,ccolour,clipboard,cstyle,focus,title
        #[cfg(feature = "tmux_1_6")]
        height: Some(65),
        #[cfg(feature = "tmux_2_2")]
        key_table: Some("root".to_string()),
        #[cfg(feature = "tmux_1_8")]
        last_session: None,
        #[cfg(feature = "tmux_2_4")]
        name: Some("/dev/pts/0".to_string()),
        #[cfg(feature = "tmux_2_1")]
        pid: Some(32150),
        #[cfg(feature = "tmux_1_8")]
        prefix: Some(false),
        #[cfg(feature = "tmux_1_6")]
        readonly: Some(false),
        #[cfg(feature = "tmux_1_8")]
        session: Some("0".to_string()),
        #[cfg(feature = "tmux_1_6")]
        termname: Some("xterm-256color".to_string()),
        #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
        termtype: None,
        // client_uid:1000
        // client_user:anton
        #[cfg(feature = "tmux_1_6")]
        tty: Some("/dev/pts/0".to_string()),
        #[cfg(feature = "tmux_1_6")]
        utf8: Some(true),
        #[cfg(feature = "tmux_1_6")]
        width: Some(177),
        #[cfg(feature = "tmux_2_4")]
        written: Some(193354),
    };

    assert_eq!(client_orig, client);
}
