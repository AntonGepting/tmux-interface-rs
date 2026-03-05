// auto-generated file
//

// Rename the session to `new-name`
//
// # Manual
//
// tmux >=0.8:
// ```text
// rename-session [-t target-session] new-name
// (alias: rename)
// ```
#[test]
fn rename_session() {
    use crate::RenameSession;
    use std::borrow::Cow;

    let rename_session = RenameSession::new();
    // `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    let rename_session = rename_session.target_session("1");

    // `[new-name]`
    #[cfg(feature = "tmux_0_8")]
    let rename_session = rename_session.new_name("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rename-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "rename";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("2");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let rename_session = rename_session.build().to_vec();

    assert_eq!(rename_session, v);
}
