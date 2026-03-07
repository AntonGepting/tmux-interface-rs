// auto-generated file
//

// Put a window into session choice mode
//
// # Manual
//
// tmux =1.7:
// ```text
// choose-session [-F format] [-t target-window] [template]
// ```
//
// tmux >=1.5:
// ```text
// choose-session [-t target-window] [template]
// ```
//
// tmux >=0.8:
// ```text
// choose-session [-t target-window]
// ```
#[test]
fn choose_session() {
    use crate::ChooseSession;
    use std::borrow::Cow;

    let choose_session = ChooseSession::new();
    // `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    let choose_session = choose_session.format("1");

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let choose_session = choose_session.target_window("2");

    // `[template]`
    #[cfg(feature = "tmux_1_5")]
    let choose_session = choose_session.template("3");

    let cmd = "choose-session";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let choose_session = choose_session.build().to_vec();

    assert_eq!(choose_session, v);
}
