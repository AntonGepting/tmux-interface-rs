#[test]
fn show_environment() {
    use crate::{ShowEnvironment, TargetSession};
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^2.1:
    // ```text
    // tmux show-environment [-gs] [-t target-session] [variable]
    // (alias: showenv)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux show-environment [-g] [-t target-session] [variable]
    // (alias: showenv)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux show-environment [-g] [-t target-session]
    // (alias: showenv)
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let mut show_environment = ShowEnvironment::new();
    #[cfg(feature = "tmux_1_0")]
    show_environment.global();
    #[cfg(feature = "tmux_2_1")]
    show_environment.as_shell_commands();
    #[cfg(feature = "tmux_1_7")]
    show_environment.target_session(&target_session);
    #[cfg(feature = "tmux_1_7")]
    show_environment.variable("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-environment";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showenv";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_0")]
    s.push("-g");
    #[cfg(feature = "tmux_2_1")]
    s.push("-s");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_7")]
    s.push("2");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(show_environment.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(show_environment.0.bin_args, None);
    assert_eq!(show_environment.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(show_environment.0.cmd_args, Some(s));
}
