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
    s.push(cmd);
    #[cfg(feature = "tmux_2_2")]
    s.push("-a");
    #[cfg(feature = "tmux_1_7")]
    s.push("-C");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let kill_session = kill_session.build().to_vec();

    assert_eq!(kill_session, s);
}
