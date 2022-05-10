#[test]
fn has_session() {
    use crate::HasSession;
    use std::borrow::Cow;

    // Report if the specified session exist
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // tmux has-session [-t target-session]
    // (alias: has)
    // ```
    let mut has_session = HasSession::new();
    #[cfg(feature = "tmux_0_8")]
    has_session.target_session("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "has-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "has";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let has_session = has_session.build().to_vec();

    assert_eq!(has_session, s);
}
