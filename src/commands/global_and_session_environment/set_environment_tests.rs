#[test]
fn set_environment() {
    use crate::{SetEnvironment, TargetSession};
    use std::borrow::Cow;

    // Structure for setting or unsetting an environment variable
    //
    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // tmux set-environment [-gru] [-t target-session] name [value]
    // (alias: setenv)
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let mut set_environment = SetEnvironment::new();
    #[cfg(feature = "tmux_1_0")]
    set_environment.global();
    #[cfg(feature = "tmux_1_0")]
    set_environment.remove();
    #[cfg(feature = "tmux_1_0")]
    set_environment.unset();
    #[cfg(feature = "tmux_1_0")]
    set_environment.target_session(&target_session);
    #[cfg(feature = "tmux_1_0")]
    set_environment.name("2");
    #[cfg(feature = "tmux_1_0")]
    set_environment.value("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-environment";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setenv";

    let mut s = Vec::new();
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
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(set_environment.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(set_environment.0.bin_args, None);
    assert_eq!(set_environment.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(set_environment.0.cmd_args, Some(s));
}
