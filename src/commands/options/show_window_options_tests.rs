#[test]
fn show_window_options() {
    use crate::{ShowWindowOptions, TargetWindow};
    use std::borrow::Cow;

    // tmux ^3.0:
    // ```text
    // (removed)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux show-window-options [-gv] [-t target-window] [option]
    // (alias: showw)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux show-window-options [-g] [-t target-window] [option]
    // (alias: showw)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux show-window-options [-g] [-t target-window]
    // (alias: showw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux show-window-options [-t target-window] option value
    // (alias: showw)
    // ```
    // (alias: showw)
    let target_window = TargetWindow::Raw("1").to_string();
    let mut show_window_options = ShowWindowOptions::new();
    #[cfg(feature = "tmux_1_0")]
    show_window_options.global();
    #[cfg(feature = "tmux_1_8")]
    show_window_options.only_value();
    #[cfg(feature = "tmux_0_8")]
    show_window_options.target_window(&target_window);
    #[cfg(feature = "tmux_0_8")]
    show_window_options.option("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    show_window_options.value("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-window-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showw";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_0")]
    s.push("-g");
    #[cfg(feature = "tmux_1_8")]
    s.push("-v");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.push("3");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(show_window_options.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(show_window_options.0.bin_args, None);
    assert_eq!(show_window_options.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(show_window_options.0.cmd_args, Some(s));
}
