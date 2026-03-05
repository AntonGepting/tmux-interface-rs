// auto-generated file
//

// Set or unset hook `hook-name` to command.
//
// # Manual
//
// tmux >=3.2:
// ```text
// set-hook [-agpRuw] [-t target-session] hook-name command
// ```
//
// tmux >=3.0:
// ```text
// set-hook [-agRu] [-t target-session] hook-name command
// ```
//
// tmux >=2.8:
// ```text
// set-hook [-gRu] [-t target-session] hook-name command
// ```
//
// tmux >=2.4:
// ```text
// set-hook [-gu] [-t target-session] hook-name command
// ```
//
// tmux >=2.2:
// ```text
// set-hook [-g] [-t target-session] hook-name command
// ```
#[test]
fn set_hook() {
    use crate::SetHook;
    use std::borrow::Cow;

    let set_hook = SetHook::new();
    // `[-a]`
    #[cfg(feature = "tmux_3_0")]
    let set_hook = set_hook.append();

    // `[-g]`
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook.global();

    // `[-p]`
    #[cfg(feature = "tmux_3_2")]
    let set_hook = set_hook.pane();

    // `[-R]`
    #[cfg(feature = "tmux_2_8")]
    let set_hook = set_hook.run();

    // `[-u]`
    #[cfg(feature = "tmux_2_4")]
    let set_hook = set_hook.unset();

    // `[-w]`
    #[cfg(feature = "tmux_3_2")]
    let set_hook = set_hook.window();

    // `[-t target-session]`
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook.target_session("1");

    // `[hook-name]`
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook.hook_name("2");

    // `[command]`
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook.command("3");

    let cmd = "set-hook";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_0")]
    v.push("-a");
    #[cfg(feature = "tmux_2_2")]
    v.push("-g");
    #[cfg(feature = "tmux_3_2")]
    v.push("-p");
    #[cfg(feature = "tmux_2_8")]
    v.push("-R");
    #[cfg(feature = "tmux_2_4")]
    v.push("-u");
    #[cfg(feature = "tmux_3_2")]
    v.push("-w");
    #[cfg(feature = "tmux_2_2")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_2_2")]
    v.push("2");
    #[cfg(feature = "tmux_2_2")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let set_hook = set_hook.build().to_vec();

    assert_eq!(set_hook, v);
}
