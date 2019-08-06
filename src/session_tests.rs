#[test]
fn parse() {
    use crate::Session;
    use crate::session::SessionStack;
    use std::time::Duration;

    let session_str = "1557947146::1:1557947146:1::::0:$0:1557947146:0:0:3,2,1:3";
    let session: Session = session_str.parse().unwrap();
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
        stack: Some(SessionStack(vec!(3,2,1))),
        windows: Some(3),
    };
    assert_eq!(session, session_sample);
}
