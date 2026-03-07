// auto-generated file
//

// Choose a specific layout for a window
//
// # Manual
//
// tmux >=2.7:
// ```text
// select-layout [-Enop] [-t target-pane] [layout-name]
// (alias: selectl)
// ```
//
// tmux >=2.1:
// ```text
// select-layout [-nop] [-t target-pane] [layout-name]
// (alias: selectl)
// ```
//
// tmux >=1.5:
// ```text
// select-layout [-np] [-t target-pane] [layout-name]
// (alias: selectl)
// ```
#[test]
fn select_layout() {
    use crate::SelectLayout;
    use std::borrow::Cow;

    let select_layout = SelectLayout::new();
    // `[-E]`
    #[cfg(feature = "tmux_2_7")]
    let select_layout = select_layout.spread();

    // `[-n]`
    #[cfg(feature = "tmux_1_5")]
    let select_layout = select_layout.next_layout();

    // `[-o]`
    #[cfg(feature = "tmux_2_1")]
    let select_layout = select_layout.last_layout();

    // `[-p]`
    #[cfg(feature = "tmux_1_5")]
    let select_layout = select_layout.previous_layout();

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_7")))]
    let select_layout = select_layout.target_window("1");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_2_7")]
    let select_layout = select_layout.target_pane("2");

    // `[layout-name]`
    #[cfg(feature = "tmux_1_5")]
    let select_layout = select_layout.layout_name("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectl";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_7")]
    v.push("-E");
    #[cfg(feature = "tmux_1_5")]
    v.push("-n");
    #[cfg(feature = "tmux_2_1")]
    v.push("-o");
    #[cfg(feature = "tmux_1_5")]
    v.push("-p");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_7")))]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_2_7")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let select_layout = select_layout.build().to_vec();

    assert_eq!(select_layout, v);
}
