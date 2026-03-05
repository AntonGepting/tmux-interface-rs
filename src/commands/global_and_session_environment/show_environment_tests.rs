// auto-generated file
//

// Display the environment variables
//
// # Manual
//
// tmux >=3.2:
// ```text:
// show-environment [-hgs] [-t target-session] [variable]
// (alias: showenv)
// ```
//
// tmux >=2.1:
// ```text
// show-environment [-gs] [-t target-session] [variable]
// (alias: showenv)
// ```
//
// tmux >=1.7:
// ```text
// show-environment [-g] [-t target-session] [variable]
// (alias: showenv)
// ```
//
// tmux >=1.5:
// ```text
// show-environment [-g] [-t target-session]
// (alias: showenv)
// ```
#[test]
fn show_environment() {
    use crate::ShowEnvironment;
    use std::borrow::Cow;

    let show_environment = ShowEnvironment::new();
    // `[-h]`
    #[cfg(feature = "tmux_3_2")]
    let show_environment = show_environment.hidden();

    // `[-g]`
    #[cfg(feature = "tmux_1_5")]
    let show_environment = show_environment.global();

    // `[-s]`
    #[cfg(feature = "tmux_2_1")]
    let show_environment = show_environment.as_shell_commands();

    // `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    let show_environment = show_environment.target_session("1");

    // `[variable]`
    #[cfg(feature = "tmux_1_7")]
    let show_environment = show_environment.variable("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-environment";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showenv";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    v.push("-h");
    #[cfg(feature = "tmux_1_5")]
    v.push("-g");
    #[cfg(feature = "tmux_2_1")]
    v.push("-s");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_7")]
    v.push("2");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let show_environment = show_environment.build().to_vec();

    assert_eq!(show_environment, v);
}
