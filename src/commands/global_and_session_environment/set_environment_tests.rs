// auto-generated file
//

// Set or unset an environment variable
//
// # Manual
//
// tmux >=3.2:
// ```text
// set-environment [-Fhgru] [-t target-session] name [value]
// (alias: setenv)
// ```
//
// tmux >=1.0:
// ```text
// set-environment [-gru] [-t target-session] name [value]
// (alias: setenv)
// ```
#[test]
fn set_environment() {
    use crate::SetEnvironment;
    use std::borrow::Cow;

    let set_environment = SetEnvironment::new();
    // `[-F]`
    #[cfg(feature = "tmux_3_2")]
    let set_environment = set_environment.expand();

    // `[-h]`
    #[cfg(feature = "tmux_3_2")]
    let set_environment = set_environment.hidden();

    // `[-g]`
    #[cfg(feature = "tmux_1_5")]
    let set_environment = set_environment.global();

    // `[-r]`
    #[cfg(feature = "tmux_1_5")]
    let set_environment = set_environment.remove();

    // `[-u]`
    #[cfg(feature = "tmux_1_5")]
    let set_environment = set_environment.unset();

    // `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    let set_environment = set_environment.target_session("1");

    // `[name]`
    #[cfg(feature = "tmux_1_5")]
    let set_environment = set_environment.name("2");

    // `[value]`
    #[cfg(feature = "tmux_1_5")]
    let set_environment = set_environment.value("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-environment";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setenv";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    v.push("-F");
    #[cfg(feature = "tmux_3_2")]
    v.push("-h");
    #[cfg(feature = "tmux_1_5")]
    v.push("-g");
    #[cfg(feature = "tmux_1_5")]
    v.push("-r");
    #[cfg(feature = "tmux_1_5")]
    v.push("-u");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("2");
    #[cfg(feature = "tmux_1_5")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let set_environment = set_environment.build().to_vec();

    assert_eq!(set_environment, v);
}
