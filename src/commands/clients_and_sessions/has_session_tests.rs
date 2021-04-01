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

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "has-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "has";

    assert_eq!(has_session.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(has_session.0.bin_args, None);
    assert_eq!(has_session.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(has_session.0.cmd_args, Some(s));
}
