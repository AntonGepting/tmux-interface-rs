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
    // tmux swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
    // (alias: swapp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
    // (alias: swapp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
    // (alias: swapp)
    // ```
    let src_pane = TargetPane::Raw("1").to_string();
    let dst_pane = TargetPane::Raw("2").to_string();

    let mut swap_pane = SwapPane::new();
    #[cfg(feature = "tmux_0_8")]
    swap_pane.detached();
    #[cfg(feature = "tmux_0_8")]
    swap_pane.previous_pane();
    #[cfg(feature = "tmux_0_8")]
    swap_pane.next_pane();
    #[cfg(feature = "tmux_3_1")]
    swap_pane.keep_zoomed();
    #[cfg(feature = "tmux_1_0")]
    swap_pane.src_pane(&src_pane);
    #[cfg(feature = "tmux_1_0")]
    swap_pane.dst_pane(&dst_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "swap-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "swapp";

    let mut s = Vec::new();
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
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(swap_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(swap_pane.0.bin_args, None);
    assert_eq!(swap_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(swap_pane.0.cmd_args, Some(s));
}
