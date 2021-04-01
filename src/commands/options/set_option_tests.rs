#[test]
fn set_option() {
    use crate::{SetOption, TargetPane};
    use std::borrow::Cow;

    // Structure for setting a pane/window/session/server option
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // tmux set-option [-aFgopqsuw] [-t target-pane] option value
    // (alias: set)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux set-option [-agoqsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux set-option [-agqsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux set-option [-agsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux set-option [-agu] [-t target-session] option value
    // (alias: set)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux set-option [-gu] [-t target-session] option value
    // (alias: set)
    // ```
    // FIXME: target, target-sesion, target-window
    let target_pane = TargetPane::Raw("1").to_string();
    let mut set_option = SetOption::new();
    #[cfg(feature = "tmux_1_0")]
    set_option.append();
    #[cfg(feature = "tmux_2_6")]
    set_option.format();
    #[cfg(feature = "tmux_0_8")]
    set_option.global();
    #[cfg(feature = "tmux_1_8")]
    set_option.not_overwrite();
    #[cfg(feature = "tmux_3_0")]
    set_option.pane();
    #[cfg(feature = "tmux_1_7")]
    set_option.quiet();
    #[cfg(feature = "tmux_1_2")]
    set_option.server();
    #[cfg(feature = "tmux_0_8")]
    set_option.unset();
    #[cfg(feature = "tmux_1_2")]
    set_option.window();
    #[cfg(feature = "tmux_3_0")]
    set_option.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    set_option.target(&target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    set_option.target_session(&target_pane);
    #[cfg(feature = "tmux_0_8")]
    set_option.option("2");
    #[cfg(feature = "tmux_0_8")]
    set_option.value("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "set";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_0")]
    s.push("-a");
    #[cfg(feature = "tmux_2_6")]
    s.push("-F");
    #[cfg(feature = "tmux_0_8")]
    s.push("-g");
    #[cfg(feature = "tmux_1_8")]
    s.push("-o");
    #[cfg(feature = "tmux_3_0")]
    s.push("-p");
    #[cfg(feature = "tmux_1_7")]
    s.push("-q");
    #[cfg(feature = "tmux_1_2")]
    s.push("-s");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_1_2")]
    s.push("-w");
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(set_option.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(set_option.0.bin_args, None);
    assert_eq!(set_option.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(set_option.0.cmd_args, Some(s));
}
