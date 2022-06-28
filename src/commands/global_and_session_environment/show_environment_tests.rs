#[test]
fn show_environment() {
    use crate::{ShowEnvironment, TargetSession};
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.2:
    // ```text:
    // show-environment [-hgs] [-t target-session] [variable]
    // (alias: showenv)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // show-environment [-gs] [-t target-session] [variable]
    // (alias: showenv)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // show-environment [-g] [-t target-session] [variable]
    // (alias: showenv)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // show-environment [-g] [-t target-session]
    // (alias: showenv)
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let show_environment = ShowEnvironment::new();
    #[cfg(feature = "tmux_3_2")]
    let show_environment = show_environment.hidden();
    #[cfg(feature = "tmux_1_0")]
    let show_environment = show_environment.global();
    #[cfg(feature = "tmux_2_1")]
    let show_environment = show_environment.as_shell_commands();
    #[cfg(feature = "tmux_1_7")]
    let show_environment = show_environment.target_session(&target_session);
    #[cfg(feature = "tmux_1_7")]
    let show_environment = show_environment.variable("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-environment";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showenv";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-h");
    #[cfg(feature = "tmux_1_0")]
    s.push("-g");
    #[cfg(feature = "tmux_2_1")]
    s.push("-s");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_7")]
    s.push("2");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let show_environment = show_environment.build().to_vec();

    assert_eq!(show_environment, s);
}
