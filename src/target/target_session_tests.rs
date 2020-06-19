#[test]
fn target_session() {
    use crate::TargetSession;

    let session_type = TargetSession::Id(1);
    assert_eq!(session_type.to_string(), "$1");
    let session_type = TargetSession::ExactName("exact_name");
    assert_eq!(session_type.to_string(), "=exact_name");
    let session_type = TargetSession::StartName("start_name");
    assert_eq!(session_type.to_string(), "start_name");
    let session_type = TargetSession::StartName("fn_match");
    assert_eq!(session_type.to_string(), "fn_match");
    let session_type = TargetSession::StartName("raw");
    assert_eq!(session_type.to_string(), "raw");
}
