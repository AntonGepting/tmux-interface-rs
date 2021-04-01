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
    // tmux set-hook [-agRu] [-t target-session] hook-name command
    // ```
    //
    // tmux ^2.8:
    // ```text
    // tmux set-hook [-gRu] [-t target-session] hook-name command
    // ```
    //
    // tmux ^2.4:
    // ```text
    // tmux set-hook [-gu] [-t target-session] hook-name command
    // ```
    //
    // tmux ^2.2:
    // ```text
    // tmux set-hook [-g] [-t target-session] hook-name command
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let mut set_hook = SetHook::new();
    #[cfg(feature = "tmux_3_0")]
    set_hook.append();
    #[cfg(feature = "tmux_2_2")]
    set_hook.global();
    #[cfg(feature = "tmux_2_8")]
    set_hook.run();
    #[cfg(feature = "tmux_2_4")]
    set_hook.unset();
    #[cfg(feature = "tmux_2_2")]
    set_hook.target_session(&target_session);
    #[cfg(feature = "tmux_2_2")]
    set_hook.hook_name("2");
    #[cfg(feature = "tmux_2_2")]
    set_hook.command("3");

    let cmd = "set-hook";

    let mut s = Vec::new();
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
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(set_hook.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(set_hook.0.bin_args, None);
    assert_eq!(set_hook.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(set_hook.0.cmd_args, Some(s));
}
