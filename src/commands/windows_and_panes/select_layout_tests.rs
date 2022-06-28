#[test]
fn select_layout() {
    use crate::{SelectLayout, TargetPane};
    use std::borrow::Cow;

    // Choose a specific layout for a window
    //
    // # Manual
    //
    // tmux ^2.7:
    // ```text
    // select-layout [-Enop] [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // select-layout [-nop] [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // select-layout [-np] [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // select-layout [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux ^0.9:
    // ```text
    // select-layout [-t target-pane] layout-name
    // (alias: selectl)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let select_layout = SelectLayout::new();
    #[cfg(feature = "tmux_2_7")]
    let select_layout = select_layout.spread();
    #[cfg(feature = "tmux_1_5")]
    let select_layout = select_layout.next_layout();
    #[cfg(feature = "tmux_2_1")]
    let select_layout = select_layout.last_layout();
    #[cfg(feature = "tmux_1_5")]
    let select_layout = select_layout.previous_layout();
    #[cfg(feature = "tmux_0_9")]
    let select_layout = select_layout.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_0")]
    let select_layout = select_layout.layout_name("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectl";

    let mut s = Vec::new();
    s.push(cmd);
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
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let select_layout = select_layout.build().to_vec();

    assert_eq!(select_layout, s);
}
