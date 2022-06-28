#[test]
fn set_hook() {
    use crate::{SetHook, TargetSession};
    use std::borrow::Cow;

    // Structure for setting or unsetting hook `hook-name` to command.
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // set-hook [-agRu] [-t target-session] hook-name command
    // ```
    //
    // tmux ^2.8:
    // ```text
    // set-hook [-gRu] [-t target-session] hook-name command
    // ```
    //
    // tmux ^2.4:
    // ```text
    // set-hook [-gu] [-t target-session] hook-name command
    // ```
    //
    // tmux ^2.2:
    // ```text
    // set-hook [-g] [-t target-session] hook-name command
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let set_hook = SetHook::new();
    #[cfg(feature = "tmux_3_0")]
    let set_hook = set_hook.append();
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook.global();
    #[cfg(feature = "tmux_2_8")]
    let set_hook = set_hook.run();
    #[cfg(feature = "tmux_2_4")]
    let set_hook = set_hook.unset();
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook.target_session(&target_session);
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook.hook_name("2");
    #[cfg(feature = "tmux_2_2")]
    let set_hook = set_hook.command("3");

    let cmd = "set-hook";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_0")]
    s.push("-a");
    #[cfg(feature = "tmux_2_2")]
    s.push("-g");
    #[cfg(feature = "tmux_2_8")]
    s.push("-R");
    #[cfg(feature = "tmux_2_4")]
    s.push("-u");
    #[cfg(feature = "tmux_2_2")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_2_2")]
    s.push("2");
    #[cfg(feature = "tmux_2_2")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let set_hook = set_hook.build().to_vec();

    assert_eq!(set_hook, s);
}
