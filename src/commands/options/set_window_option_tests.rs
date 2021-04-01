#[test]
fn set_window_option() {
    use crate::{SetWindowOption, TargetWindow};
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // (removed)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // tmux set-window-option [-aFgoqu] [-t target-window] option value
    // (alias: setw)
    // ```
    //
    // tmux ^1.9:
    // ```text
    // tmux set-window-option [-agoqu] [-t target-window] option value
    // (alias: setw)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux set-window-option [-agqu] [-t target-window] option value
    // (alias: setw)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux set-window-option [-agu] [-t target-window] option value
    // (alias: setw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux set-window-option [-gu] [-t target-window] option value
    // (alias: setw)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let mut set_window_option = SetWindowOption::new();
    #[cfg(feature = "tmux_1_0")]
    set_window_option.append();
    #[cfg(feature = "tmux_2_6")]
    set_window_option.format();
    #[cfg(feature = "tmux_0_8")]
    set_window_option.global();
    #[cfg(feature = "tmux_1_9")]
    set_window_option.not_overwrite();
    #[cfg(feature = "tmux_1_7")]
    set_window_option.quiet();
    #[cfg(feature = "tmux_0_8")]
    set_window_option.unset();
    #[cfg(feature = "tmux_0_8")]
    set_window_option.target_window(&target_window);
    #[cfg(feature = "tmux_0_8")]
    set_window_option.option("2");
    #[cfg(feature = "tmux_0_8")]
    set_window_option.value("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-window-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setw";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_0")]
    s.push("-a");
    #[cfg(feature = "tmux_2_6")]
    s.push("-F");
    #[cfg(feature = "tmux_0_8")]
    s.push("-g");
    #[cfg(feature = "tmux_1_9")]
    s.push("-o");
    #[cfg(feature = "tmux_1_7")]
    s.push("-q");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(set_window_option.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(set_window_option.0.bin_args, None);
    assert_eq!(set_window_option.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(set_window_option.0.cmd_args, Some(s));
}
