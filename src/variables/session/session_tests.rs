#[test]
fn show_generated_struct() {
    use crate::Session;

    let _session = Session {
        ..Default::default()
    };
    //dbg!(_session);
}

#[test]
fn parse() {
    use crate::Session;
    #[cfg(feature = "tmux_2_5")]
    use crate::SessionStack;
    use std::str::FromStr;

    let session_vec = vec![
        // session_activity
        #[cfg(feature = "tmux_2_1")]
        "1557947146",
        // session_activity_string
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        "",
        // session_alerts
        #[cfg(feature = "tmux_2_1")]
        "",
        // session_attached
        #[cfg(feature = "tmux_1_6")]
        "1",
        // session_attached_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // session_created
        #[cfg(feature = "tmux_1_6")]
        "1557947146",
        // session_created_string
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        "",
        // session_format
        #[cfg(feature = "tmux_2_6")]
        "1",
        // session_group
        #[cfg(feature = "tmux_1_6")]
        "",
        // session_group_attached
        #[cfg(feature = "tmux_3_1")]
        "",
        // session_group_attached_list
        #[cfg(feature = "tmux_3_1")]
        "",
        // session_group_list
        #[cfg(feature = "tmux_2_7")]
        "",
        // session_group_many_attached
        #[cfg(feature = "tmux_3_1")]
        "",
        // session_group_size
        #[cfg(feature = "tmux_2_7")]
        "",
        // session_grouped
        #[cfg(feature = "tmux_1_6")]
        "",
        // session_height
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        "",
        // session_width
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        "0",
        // session_id
        #[cfg(feature = "tmux_1_8")]
        "$0",
        // session_last_attached
        #[cfg(feature = "tmux_2_1")]
        "1557947146",
        // session_last_attached_string
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        "",
        // session_many_attached
        #[cfg(feature = "tmux_2_0")]
        "0",
        // session_name
        #[cfg(feature = "tmux_1_6")]
        "0",
        // session_stack
        #[cfg(feature = "tmux_2_5")]
        "3,2,1",
        // session_windows
        #[cfg(feature = "tmux_1_6")]
        "3",
    ];
    let session_str = session_vec.join(":");

    let session = Session::from_str(&session_str).unwrap();
    let session_sample = Session {
        #[cfg(feature = "tmux_2_1")]
        activity: Some(1557947146),
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        activity_string: None,
        #[cfg(feature = "tmux_2_1")]
        alerts: None,
        #[cfg(feature = "tmux_1_6")]
        attached: Some(1),
        #[cfg(feature = "tmux_3_1")]
        attached_list: None,
        #[cfg(feature = "tmux_1_6")]
        created: Some(1557947146),
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        created_string: None,
        #[cfg(feature = "tmux_2_6")]
        format: Some(true),
        #[cfg(feature = "tmux_1_6")]
        group: None,
        #[cfg(feature = "tmux_3_1")]
        group_attached: None,
        #[cfg(feature = "tmux_3_1")]
        group_attached_list: None,
        #[cfg(feature = "tmux_2_7")]
        group_list: None,
        #[cfg(feature = "tmux_3_1")]
        group_many_attached: None,
        #[cfg(feature = "tmux_2_7")]
        group_size: None,
        #[cfg(feature = "tmux_1_6")]
        grouped: None,
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        height: None,
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        width: Some(0),
        #[cfg(feature = "tmux_1_8")]
        id: Some(0),
        #[cfg(feature = "tmux_2_1")]
        last_attached: Some(1557947146),
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        last_attached_string: None,
        #[cfg(feature = "tmux_2_0")]
        many_attached: Some(false),
        #[cfg(feature = "tmux_1_6")]
        name: Some("0".to_string()),
        #[cfg(feature = "tmux_2_5")]
        stack: Some(SessionStack(vec![3, 2, 1])),
        #[cfg(feature = "tmux_1_6")]
        windows: Some(3),
    };
    assert_eq!(session, session_sample);
}
