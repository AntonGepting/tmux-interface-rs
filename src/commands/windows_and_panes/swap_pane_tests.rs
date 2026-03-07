// auto-generated file
//

// Swap two panes
//
// # Manual
//
// tmux >=3.1:
// ```text
// swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
// (alias: swapp)
// ```
//
// tmux >=1.5:
// ```text
// swap-pane [-dDU] [-s src-pane] [-t dst-pane]
// (alias: swapp)
// ```
//
// tmux >=0.8:
// ```text
// swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
// (alias: swapp)
// ```
#[test]
fn swap_pane() {
    use crate::SwapPane;
    use std::borrow::Cow;

    let swap_pane = SwapPane::new();
    // `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane.detached();

    // `[-D]`
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane.previous_pane();

    // `[-U]`
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane.next_pane();

    // `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    let swap_pane = swap_pane.keep_zoomed();

    // `[-p src-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let swap_pane = swap_pane.src_index("1");

    // `[-s src-pane]`
    #[cfg(feature = "tmux_1_5")]
    let swap_pane = swap_pane.src_pane("2");

    // `[-q dst-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let swap_pane = swap_pane.dst_index("3");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let swap_pane = swap_pane.target_window("4");

    // `[-t dst-pane]`
    #[cfg(feature = "tmux_1_5")]
    let swap_pane = swap_pane.dst_pane("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "swap-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "swapp";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_0_8")]
    v.push("-D");
    #[cfg(feature = "tmux_0_8")]
    v.push("-U");
    #[cfg(feature = "tmux_3_1")]
    v.push("-Z");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-p", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-s", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-q", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "5"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let swap_pane = swap_pane.build().to_vec();

    assert_eq!(swap_pane, v);
}
