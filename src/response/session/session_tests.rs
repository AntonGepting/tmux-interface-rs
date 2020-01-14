#[test]
fn parse() {
    use crate::session::Session;
    use crate::session::SESSION_ALL;
    use crate::session_stack::SessionStack;
    use std::time::Duration;

    let session_str = "1557947146::1:1557947146:1::::0:$0:1557947146:0:0:3,2,1:3";
    let session = Session::from_str(session_str, SESSION_ALL).unwrap();
    let session_sample = Session {
        activity: Some(Duration::from_millis(1557947146)),
        alerts: None,
        attached: Some(1),
        created: Some(Duration::from_millis(1557947146)),
        format: Some(true),
        group: None,
        group_list: None,
        group_size: None,
        grouped: Some(false),
        id: Some(0),
        last_attached: Some(Duration::from_millis(1557947146)),
        many_attached: Some(false),
        name: Some("0".to_string()),
        stack: Some(SessionStack(vec![3, 2, 1])),
        windows: Some(3),
    };
    assert_eq!(session, session_sample);
}

#[test]
fn parse2() {
    use crate::session::Session;
    use crate::session::{SESSION_ACTIVITY, SESSION_CREATED, SESSION_LAST_ATTACHED};
    use std::time::Duration;

    let session_str = "1557947146:1557947146:1557947146";
    let session = Session::from_str(
        session_str,
        SESSION_ACTIVITY | SESSION_CREATED | SESSION_LAST_ATTACHED,
    )
    .unwrap();
    let origin = Session {
        activity: Some(Duration::from_millis(1557947146)),
        alerts: None,
        attached: None,
        created: Some(Duration::from_millis(1557947146)),
        format: None,
        group: None,
        group_list: None,
        group_size: None,
        grouped: None,
        id: None,
        last_attached: Some(Duration::from_millis(1557947146)),
        many_attached: None,
        name: None,
        stack: None,
        windows: None,
    };
    assert_eq!(session, origin);
}
