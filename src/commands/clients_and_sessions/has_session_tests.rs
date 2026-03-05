// auto-generated file
//

// Report if the specified session exist
//
// # Manual
//
// tmux >=0.8:
// ```text
// has-session [-t target-session]
// (alias: has)
// ```
#[test]
fn has_session() {
    use crate::HasSession;
    use std::borrow::Cow;

    let has_session = HasSession::new();
    // `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    let has_session = has_session.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "has-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "has";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let has_session = has_session.build().to_vec();

    assert_eq!(has_session, v);
}
