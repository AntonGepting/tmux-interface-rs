#[test]
fn wait_for() {
    use crate::WaitFor;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // tmux wait-for [-L | -S | -U] channel
    // (alias: wait)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux wait-for -LSU channel
    // (alias: wait)
    // ```
    let wait_for = WaitFor::new();
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for.locked();
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for.woken();
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for.unlocked();
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for.channel("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "wait-for";
    #[cfg(feature = "cmd_alias")]
    let cmd = "wait";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    s.push("-L");
    #[cfg(feature = "tmux_1_8")]
    s.push("-S");
    #[cfg(feature = "tmux_1_8")]
    s.push("-U");
    #[cfg(feature = "tmux_1_8")]
    s.push("1");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let wait_for = wait_for.build().to_vec();

    assert_eq!(wait_for, s);
}
