#[test]
fn select_pane() {
    use crate::{SelectPane, TargetPane};
    use std::borrow::Cow;

    // Make pane `target-pane` the active pane in window `target-window`
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // tmux select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // tmux select-pane [-DdeLlRU] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux select-pane [-DLlRU] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^1.3:
    // ```text
    // tmux select-pane [-DLRU] [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux select-pane [-t target-pane]
    // (alias: selectp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux select-pane [-p pane-index] [-t target-window]
    // (alias: selectp)
    // ```
    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectp";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_3")]
    s.push("-D");
    #[cfg(feature = "tmux_2_0")]
    s.push("-d");
    #[cfg(feature = "tmux_2_0")]
    s.push("-e");
    #[cfg(feature = "tmux_2_1")]
    s.push("-g");
    #[cfg(feature = "tmux_1_3")]
    s.push("-L");
    #[cfg(feature = "tmux_1_5")]
    s.push("-l");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_2_1")]
    s.push("-m");
    #[cfg(feature = "tmux_1_3")]
    s.push("-R");
    #[cfg(feature = "tmux_1_3")]
    s.push("-U");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-P", "1"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-T", "2"]);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "3"]);

    let target_pane = TargetPane::Raw("3").to_string();

    let mut select_pane = SelectPane::new();
    #[cfg(feature = "tmux_1_3")]
    select_pane.down();
    #[cfg(feature = "tmux_2_0")]
    select_pane.disable();
    #[cfg(feature = "tmux_2_0")]
    select_pane.enable();
    #[cfg(feature = "tmux_2_1")]
    select_pane.show_style();
    #[cfg(feature = "tmux_1_3")]
    select_pane.left();
    #[cfg(feature = "tmux_1_5")]
    select_pane.last();
    #[cfg(feature = "tmux_2_1")]
    select_pane.set_marked();
    #[cfg(feature = "tmux_2_1")]
    select_pane.clear_marked();
    #[cfg(feature = "tmux_1_3")]
    select_pane.right();
    #[cfg(feature = "tmux_1_3")]
    select_pane.up();
    #[cfg(feature = "tmux_3_1")]
    select_pane.keep_zoomed();
    #[cfg(feature = "tmux_2_1")]
    select_pane.style("1");
    #[cfg(feature = "tmux_2_6")]
    select_pane.title("2");
    #[cfg(feature = "tmux_1_0")]
    select_pane.target_pane(&target_pane);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(select_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(select_pane.0.bin_args, None);
    assert_eq!(select_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(select_pane.0.cmd_args, Some(s));
}
