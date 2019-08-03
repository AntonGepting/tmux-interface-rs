#[test]
fn parse() {
    use crate::Session;
    use std::time::Duration;

    let session: Session = ":1:1557947146:1557947146:1:1557947146::::0:$0:0:0:3,2,1:3".parse().unwrap();
    let session_sample = Session {
        alerts: None,
        attached: Some(1),
        activity: Some(Duration::from_millis(1557947146)),
        created: Some(Duration::from_millis(1557947146)),
        format: Some("1".to_string()),
        last_attached: Some(Duration::from_millis(1557947146)),
        group: None,
        group_size: None,
        group_list: None,
        grouped: Some("0".to_string()),
        id: Some(0),
        many_attached: Some("0".to_string()),
        name: Some("0".to_string()),
        stack: Some("3,2,1".to_string()),
        windows: Some(3),
    };
    assert_eq!(session, session_sample);
}
