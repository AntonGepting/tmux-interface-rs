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
    // tmux rename-session [-t target-session] new-name
    // (alias: rename)
    // ```
    let target_session = TargetSession::Raw("1").to_string();
    let mut rename_session = RenameSession::new();
    #[cfg(feature = "tmux_0_8")]
    rename_session.target_session(&target_session);
    #[cfg(feature = "tmux_0_8")]
    rename_session.new_name("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rename-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "rename";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(rename_session.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(rename_session.0.bin_args, None);
    assert_eq!(rename_session.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(rename_session.0.cmd_args, Some(s));
}
