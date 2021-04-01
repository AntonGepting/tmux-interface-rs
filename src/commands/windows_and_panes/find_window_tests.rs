#[test]
fn find_window() {
    use crate::{FindWindow, TargetPane};
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // tmux find-window [-rCNTZ] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux ^2.6:
    // ```text
    // tmux find-window [-CNT] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux ^1.7:
    // ```text
    // tmux find-window [-CNT] [-F format] [-t target-pane] match-string
    // (alias: findw)
    //
    // tmux ^0.8:
    // ```text
    // tmux find-window [-t target-pane] match-string
    // (alias: findw)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let mut find_window = FindWindow::new();

    #[cfg(feature = "tmux_3_0")]
    find_window.regex();
    #[cfg(feature = "tmux_1_7")]
    find_window.only_visible();
    #[cfg(feature = "tmux_1_7")]
    find_window.only_name();
    #[cfg(feature = "tmux_1_7")]
    find_window.only_title();
    #[cfg(feature = "tmux_3_0")]
    find_window.zoom();
    #[cfg(feature = "tmux_0_8")]
    find_window.target_pane(&target_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "find-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "findw";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_3_0")]
    s.push("-r");
    #[cfg(feature = "tmux_1_7")]
    s.push("-C");
    #[cfg(feature = "tmux_1_7")]
    s.push("-N");
    #[cfg(feature = "tmux_1_7")]
    s.push("-T");
    #[cfg(feature = "tmux_3_0")]
    s.push("-Z");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(find_window.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(find_window.0.bin_args, None);
    assert_eq!(find_window.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(find_window.0.cmd_args, Some(s));
}
