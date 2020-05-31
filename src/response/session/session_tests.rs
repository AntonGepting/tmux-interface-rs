#[test]
fn bitflags() {
    use crate::{SESSION_ALL, SESSION_NONE};
    let bitflags =
        // 31_____23____16_15_____________0
        0b_000000011111111_1111111111111111;
    //println!("{:b}", SESSION_ALL);
    //println!("{:b}", &bitflags);
    assert_eq!(bitflags, SESSION_ALL);
    assert_eq!(0, SESSION_NONE);
}

#[test]
fn parse() {
    use crate::response::session::session::Session;
    use crate::response::session::session::SESSION_ALL;
    use crate::response::session::session_stack::SessionStack;
    use std::time::Duration;

    let session_str = "1557947146::1:1557947146:1::::0:$0:1557947146:0:0:3,2,1:3";
    let session = Session::from_str(session_str, SESSION_ALL).unwrap();
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
        #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_2_2")))]
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
        grouped: Some(false),
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        height: None,
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        width: None,
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
    use crate::response::session::session::Session;
    use crate::response::session::session::{
        SESSION_ACTIVITY, SESSION_CREATED, SESSION_LAST_ATTACHED,
    };
    use std::time::Duration;

    let session_str = "1557947146:1557947146:1557947146";
    let session = Session::from_str(
        session_str,
        SESSION_ACTIVITY | SESSION_CREATED | SESSION_LAST_ATTACHED,
    )
    .unwrap();
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
        #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_2_2")))]
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
