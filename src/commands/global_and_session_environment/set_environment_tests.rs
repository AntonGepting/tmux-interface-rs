#[test]
fn set_environment() {
    use crate::{SetEnvironment, TargetSession};
    use std::borrow::Cow;

    // Structure for setting or unsetting an environment variable
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux set-environment [-Fhgru] [-t target-session] name [value]
    // (alias: setenv)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux set-environment [-gru] [-t target-session] name [value]
    // (alias: setenv)
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let set_environment = SetEnvironment::new();
    #[cfg(feature = "tmux_3_2")]
    let set_environment = set_environment.expand();
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment.global();
    #[cfg(feature = "tmux_3_2")]
    let set_environment = set_environment.hidden();
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment.remove();
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment.unset();
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment.target_session(&target_session);
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment.name("2");
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment.value("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-environment";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setenv";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-F");
    #[cfg(feature = "tmux_3_2")]
    s.push("-h");
    #[cfg(feature = "tmux_1_0")]
    s.push("-g");
    #[cfg(feature = "tmux_1_0")]
    s.push("-r");
    #[cfg(feature = "tmux_1_0")]
    s.push("-u");
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.push("2");
    #[cfg(feature = "tmux_1_0")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let set_environment = set_environment.build().to_vec();

    assert_eq!(set_environment, s);
}
