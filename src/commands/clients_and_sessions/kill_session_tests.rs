// auto-generated file
//

// Destroy the given session
//
// # Manual
//
// tmux >=2.2:
// ```text
// kill-session [-aC] [-t target-session]
// ```
//
// tmux >=1.9:
// ```text
// kill-session [-a] [-t target-session]
// ```
//
// tmux >=0.8:
// ```text
// kill-session [-t target-session]
// ```
#[test]
fn kill_session() {
    use crate::KillSession;
    use std::borrow::Cow;

    let kill_session = KillSession::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_9")]
    let kill_session = kill_session.all();

    // `[-C]`
    #[cfg(feature = "tmux_2_2")]
    let kill_session = kill_session.clear_alerts();

    // `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    let kill_session = kill_session.target_session("1");

    let cmd = "kill-session";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_9")]
    v.push("-a");
    #[cfg(feature = "tmux_2_2")]
    v.push("-C");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let kill_session = kill_session.build().to_vec();

    assert_eq!(kill_session, v);
}
