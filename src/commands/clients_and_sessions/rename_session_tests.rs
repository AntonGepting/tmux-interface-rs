#[test]
fn rename_session() {
    use crate::{RenameSession, TargetSession};
    use std::borrow::Cow;

    // Rename the session to `new-name`
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // rename-session [-t target-session] new-name
    // (alias: rename)
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let rename_session = RenameSession::new();
    #[cfg(feature = "tmux_0_8")]
    let rename_session = rename_session.target_session(&target_session);
    #[cfg(feature = "tmux_0_8")]
    let rename_session = rename_session.new_name("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rename-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "rename";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let rename_session = rename_session.build().to_vec();

    assert_eq!(rename_session, s);
}
