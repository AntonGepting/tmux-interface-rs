#[test]
fn show_hooks() {
    use crate::{ShowHooks, TargetSession};
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^2.2:
    // ```text
    // tmux show-hooks [-g] [-t target-session]
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let mut show_hooks = ShowHooks::new();
    #[cfg(feature = "tmux_2_2")]
    show_hooks.global();
    #[cfg(feature = "tmux_2_2")]
    show_hooks.target_session(&target_session);

    let cmd = "show-hooks";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_2")]
    s.push("-g");
    #[cfg(feature = "tmux_2_2")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let show_hooks = show_hooks.build().to_vec();

    assert_eq!(show_hooks, s);
}
