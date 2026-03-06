// auto-generated file
//

// Prevent the client from exiting
//
// # Manual
//
// tmux >=1.9:
// ```text
// wait-for [-L | -S | -U] channel
// (alias: wait)
// ```
//
// tmux >=1.8:
// ```text
// wait-for -LSU channel
// (alias: wait)
// ```
#[test]
fn wait_for() {
    use crate::WaitFor;
    use std::borrow::Cow;

    let wait_for = WaitFor::new();
    // `[-L]`
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for.locked();

    // `[-S]`
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for.woken();

    // `[-U]`
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for.unlocked();

    // `[channel]`
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for.channel("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "wait-for";
    #[cfg(feature = "cmd_alias")]
    let cmd = "wait";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    v.push("-L");
    #[cfg(feature = "tmux_1_8")]
    v.push("-S");
    #[cfg(feature = "tmux_1_8")]
    v.push("-U");
    #[cfg(feature = "tmux_1_8")]
    v.push("1");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let wait_for = wait_for.build().to_vec();

    assert_eq!(wait_for, v);
}
