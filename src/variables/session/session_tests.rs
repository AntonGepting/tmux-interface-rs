#[test]
fn show_generated_struct() {
    use crate::Session;

    let _session = Session {
        ..Default::default()
    };
    //dbg!(_session);
}

#[test]
fn bitflags() {
    use crate::{SESSION_ALL, SESSION_NONE};
    let bitflags =
        // 31______23_____1615_____________0
        0b_0000000011111111_1111111111111111;
    //println!("{:b}", SESSION_ALL);
    //println!("{:b}", &bitflags);
    assert_eq!(bitflags, SESSION_ALL);
    assert_eq!(0, SESSION_NONE);
}

#[test]
fn parse() {
    use crate::Session;
    #[cfg(feature = "tmux_2_5")]
    use crate::SessionStack;
    use crate::SESSION_ALL;
    use std::time::Duration;

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
    //println!("{}", &session_str);
    let session = Session::from_str(&session_str, SESSION_ALL).unwrap();
    let session_sample = Session {
        #[cfg(feature = "tmux_2_1")]
        activity: Some(Duration::from_millis(1557947146)),
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        activity_string: None,
        #[cfg(feature = "tmux_2_1")]
        alerts: None,
        #[cfg(feature = "tmux_1_6")]
        attached: Some(1),
        #[cfg(feature = "tmux_3_1")]
        attached_list: None,
        #[cfg(feature = "tmux_1_6")]
        created: Some(Duration::from_millis(1557947146)),
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
        last_attached: Some(Duration::from_millis(1557947146)),
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

#[test]
fn parse2() {
    use crate::Session;
    #[cfg(feature = "tmux_2_1")]
    use crate::SESSION_ACTIVITY;
    #[cfg(feature = "tmux_1_6")]
    use crate::SESSION_CREATED;
    #[cfg(feature = "tmux_2_1")]
    use crate::SESSION_LAST_ATTACHED;
    use std::time::Duration;

    let session_str = "1557947146:1557947146:1557947146";
    #[cfg(feature = "tmux_1_6")]
    let session_bitflag = SESSION_CREATED;
    #[cfg(feature = "tmux_2_1")]
    let session_bitflag = SESSION_ACTIVITY | SESSION_CREATED | SESSION_LAST_ATTACHED;
    let session = Session::from_str(session_str, session_bitflag).unwrap();
    let origin = Session {
        #[cfg(feature = "tmux_2_1")]
        activity: Some(Duration::from_millis(1557947146)),
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        activity_string: None,
        #[cfg(feature = "tmux_2_1")]
        alerts: None,
        #[cfg(feature = "tmux_1_6")]
        attached: None,
        #[cfg(feature = "tmux_3_1")]
        attached_list: None,
        #[cfg(feature = "tmux_1_6")]
        created: Some(Duration::from_millis(1557947146)),
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        created_string: None,
        #[cfg(feature = "tmux_2_6")]
        format: None,
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
        width: None,
        #[cfg(feature = "tmux_1_8")]
        id: None,
        #[cfg(feature = "tmux_2_1")]
        last_attached: Some(Duration::from_millis(1557947146)),
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        last_attached_string: None,
        #[cfg(feature = "tmux_2_0")]
        many_attached: None,
        #[cfg(feature = "tmux_1_6")]
        name: None,
        #[cfg(feature = "tmux_2_5")]
        stack: None,
        #[cfg(feature = "tmux_1_6")]
        windows: None,
    };
    assert_eq!(session, origin);
}
