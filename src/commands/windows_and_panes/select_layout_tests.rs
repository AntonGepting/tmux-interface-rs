#[test]
fn select_layout() {
    use crate::{SelectLayot, TargetPane};
    use std::borrow::Cow;

    // Choose a specific layout for a window
    //
    // # Manual
    //
    // tmux ^2.7:
    // ```text
    // tmux select-layout [-Enop] [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux select-layout [-nop] [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux select-layout [-np] [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux select-layout [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux ^0.9:
    // ```text
    // tmux select-layout [-t target-pane] layout-name
    // (alias: selectl)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let mut select_layout = SelectLayot::new();
    #[cfg(feature = "tmux_2_7")]
    select_layout.spread();
    #[cfg(feature = "tmux_1_5")]
    select_layout.next_layout();
    #[cfg(feature = "tmux_2_1")]
    select_layout.last_layout();
    #[cfg(feature = "tmux_1_5")]
    select_layout.previous_layout();
    #[cfg(feature = "tmux_0_9")]
    select_layout.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_0")]
    select_layout.layout_name("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectl";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_7")]
    s.push("-E");
    #[cfg(feature = "tmux_1_5")]
    s.push("-n");
    #[cfg(feature = "tmux_2_1")]
    s.push("-o");
    #[cfg(feature = "tmux_1_5")]
    s.push("-p");
    #[cfg(feature = "tmux_0_9")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.push("2");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(select_layout.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(select_layout.0.bin_args, None);
    assert_eq!(select_layout.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(select_layout.0.cmd_args, Some(s));
}
