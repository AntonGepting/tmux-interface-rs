#[test]
fn attach_session() {
    use crate::{AttachSession, TargetSession};
    use std::borrow::Cow;

    // Structure for attaching client to already existing session
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // tmux attach-session [-dErx] [-c working-directory] [-t target-session]
    // (alias: attach)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux attach-session [-dEr] [-c working-directory] [-t target-session]
    // (alias: attach)
    // ```
    //
    // tmux ^1.9:
    // ```text
    // tmux attach-session [-dr] [-c working-directory] [-t target-session]
    // (alias: attach)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux attach-session [-dr] [-t target-session]
    // (alias: attach)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux attach-session [-d] [-t target-session]
    // (alias: attach)
    // ```
    let target_session = TargetSession::Raw("2").to_string();

    let mut attach_session = AttachSession::new();
    #[cfg(feature = "tmux_0_8")]
    attach_session.detach_other();
    #[cfg(feature = "tmux_2_1")]
    attach_session.not_update_env();
    #[cfg(feature = "tmux_1_2")]
    attach_session.read_only();
    #[cfg(feature = "tmux_3_0")]
    attach_session.parent_sighup();
    #[cfg(feature = "tmux_1_9")]
    attach_session.working_directory("1");
    #[cfg(feature = "tmux_0_8")]
    attach_session.target_session(&target_session);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "attach-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "attach";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_2_1")]
    s.push("-E");
    #[cfg(feature = "tmux_1_2")]
    s.push("-r");
    #[cfg(feature = "tmux_3_0")]
    s.push("-x");
    #[cfg(feature = "tmux_1_9")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(attach_session.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(attach_session.0.bin_args, None);
    assert_eq!(attach_session.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(attach_session.0.cmd_args, Some(s));
}
