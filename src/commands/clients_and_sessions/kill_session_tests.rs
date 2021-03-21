#[test]
fn kill_session() {
    use crate::{KillSession, TargetSession};
    use std::borrow::Cow;

    // Destroy the given session
    //
    // # Manual
    //
    // tmux ^2.2:
    // ```text
    // tmux kill-session [-aC] [-t target-session]
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux kill-session [-a] [-t target-session]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux kill-session [-t target-session]
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let mut kill_session = KillSession::new();
    #[cfg(feature = "tmux_2_2")]
    kill_session.all();
    #[cfg(feature = "tmux_1_7")]
    kill_session.clear_alerts();
    #[cfg(feature = "tmux_0_8")]
    kill_session.target_session(&target_session);

    let cmd = "kill-session";
    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_2")]
    s.push("-a");
    #[cfg(feature = "tmux_1_7")]
    s.push("-C");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(kill_session.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(kill_session.0.bin_args, None);
    assert_eq!(kill_session.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(kill_session.0.cmd_args, Some(s));
}
