// auto-generated file
//

// Make pane `target-pane` the active pane in window `target-window`
//
// # Manual
//
// tmux >=3.1:
// ```text
// select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
// (alias: selectp)
// ```
//
// tmux >=2.6:
// ```text
// select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
// (alias: selectp)
// ```
//
// tmux >=2.1:
// ```text
// select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
// (alias: selectp)
// ```
//
// tmux >=2.0:
// ```text
// select-pane [-DdeLlRU] [-t target-pane]
// (alias: selectp)
// ```
//
// tmux >=1.5:
// ```text
// select-pane [-DLlRU] [-t target-pane]
// (alias: selectp)
// ```
//
// tmux >=0.8:
// ```text
// select-pane [-p pane-index] [-t target-window]
// (alias: selectp)
// ```
#[test]
fn select_pane() {
    use crate::SelectPane;
    use std::borrow::Cow;

    let select_pane = SelectPane::new();
    // `[-D]`
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane.down();

    // `[-d]`
    #[cfg(feature = "tmux_2_0")]
    let select_pane = select_pane.disable();

    // `[-e]`
    #[cfg(feature = "tmux_2_0")]
    let select_pane = select_pane.enable();

    // `[-g]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_1")))]
    let select_pane = select_pane.show_style();

    // `[-L]`
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane.left();

    // `[-l]`
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane.last();

    // `[-M]`
    #[cfg(feature = "tmux_2_1")]
    let select_pane = select_pane.set_marked();

    // `[-m]`
    #[cfg(feature = "tmux_2_1")]
    let select_pane = select_pane.clear_marked();

    // `[-R]`
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane.right();

    // `[-U]`
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane.up();

    // `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    let select_pane = select_pane.keep_zoomed();

    // `[-p pane-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let select_pane = select_pane.pane_index("1");

    // `[-P style]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0a")))]
    let select_pane = select_pane.style("2");

    // `[-T title]`
    #[cfg(feature = "tmux_2_6")]
    let select_pane = select_pane.title("3");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let select_pane = select_pane.target_window("4");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let select_pane = select_pane.target_pane("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectp";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-D");
    #[cfg(feature = "tmux_2_0")]
    v.push("-d");
    #[cfg(feature = "tmux_2_0")]
    v.push("-e");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_1")))]
    v.push("-g");
    #[cfg(feature = "tmux_1_5")]
    v.push("-L");
    #[cfg(feature = "tmux_1_5")]
    v.push("-l");
    #[cfg(feature = "tmux_2_1")]
    v.push("-M");
    #[cfg(feature = "tmux_2_1")]
    v.push("-m");
    #[cfg(feature = "tmux_1_5")]
    v.push("-R");
    #[cfg(feature = "tmux_1_5")]
    v.push("-U");
    #[cfg(feature = "tmux_3_1")]
    v.push("-Z");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-p", "1"]);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0a")))]
    v.extend_from_slice(&["-P", "2"]);
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-T", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "5"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let select_pane = select_pane.build().to_vec();

    assert_eq!(select_pane, v);
}
