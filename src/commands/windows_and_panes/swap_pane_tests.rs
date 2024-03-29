#[test]
fn swap_pane() {
    use crate::{SwapPane, TargetPane};
    use std::borrow::Cow;

    // Swap two panes
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
    // (alias: swapp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // swap-pane [-dDU] [-s src-pane] [-t dst-pane]
    // (alias: swapp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
    // (alias: swapp)
    // ```
    let src_pane = TargetPane::Raw("1").to_string();
    let dst_pane = TargetPane::Raw("2").to_string();

    let swap_pane = SwapPane::new();
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane.detached();
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane.previous_pane();
    #[cfg(feature = "tmux_0_8")]
    let swap_pane = swap_pane.next_pane();
    #[cfg(feature = "tmux_3_1")]
    let swap_pane = swap_pane.keep_zoomed();
    #[cfg(feature = "tmux_1_0")]
    let swap_pane = swap_pane.src_pane(&src_pane);
    #[cfg(feature = "tmux_1_0")]
    let swap_pane = swap_pane.dst_pane(&dst_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "swap-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "swapp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_0_8")]
    s.push("-D");
    #[cfg(feature = "tmux_0_8")]
    s.push("-U");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let swap_pane = swap_pane.build().to_vec();

    assert_eq!(swap_pane, s);
}
