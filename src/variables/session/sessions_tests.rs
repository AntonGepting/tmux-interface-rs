#[test]
fn parse() {
    use crate::{Sessions, SESSION_ALL};

    //"1557947146::1:1557947146:1::::0::0:$0:1557947146:0:0:3,2,1:3\n\
    //1557947146::1:1557947146:1::::0::0:$0:1557947146:0:0:3,2,1:3";
    let session1_vec = vec![
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
    let session2_vec = vec![
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
        "$1",
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
    let session1_str = session1_vec.join(":");
    let session2_str = session2_vec.join(":");
    let sessions_str = format!("{}\n{}", session1_str, session2_str);
    let sessions = Sessions::from_str(&sessions_str, SESSION_ALL).unwrap();
    #[cfg(feature = "tmux_1_8")]
    assert_eq!(sessions[0].id, Some(0));

    //"1557947146::1:1557947146:1::::0::0:$0:1557947146:0:0:4,3,2,1:4\n\
    //1557947146::0:1557947146:1::::0::0:$40:1557947146:0:test_has_session:1:1";
    let session1_vec = vec![
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
    let session2_vec = vec![
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
        "$40",
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
        "test_has_session",
        // session_stack
        #[cfg(feature = "tmux_2_5")]
        "1",
        // session_windows
        #[cfg(feature = "tmux_1_6")]
        "1",
    ];
    let session1_str = session1_vec.join(":");
    let session2_str = session2_vec.join(":");
    let sessions_str = format!("{}\n{}", session1_str, session2_str);
    let sessions = Sessions::from_str(&sessions_str, SESSION_ALL).unwrap();
    #[cfg(feature = "tmux_1_8")]
    assert_eq!(sessions[1].id, Some(40));
}
