#[test]
fn resize_pane() {
    use crate::{ResizePane, TargetPane};
    use std::borrow::Cow;

    // Resize a pane, up, down, left or right
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux resize-pane [-DLMRTUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    // (alias: resizep)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    // (alias: resizep)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux resize-pane [-DLRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    // (alias: resizep)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux resize-pane [-DLRU] [-t target-pane] [adjustment]
    // (alias: resizep)
    // ```
    //
    // tmux ^0.9:
    // ```text
    // tmux resize-pane [-DU] [-p pane-index] [-t target-pane] [adjustment]
    // (alias: resizep)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();
    let mut resize_pane = ResizePane::new();
    #[cfg(feature = "tmux_0_9")]
    resize_pane.down();
    #[cfg(feature = "tmux_1_8")]
    resize_pane.left();
    #[cfg(feature = "tmux_2_1")]
    resize_pane.mouse();
    #[cfg(feature = "tmux_1_8")]
    resize_pane.right();
    #[cfg(feature = "tmux_3_2")]
    resize_pane.trim();
    #[cfg(feature = "tmux_0_9")]
    resize_pane.up();
    #[cfg(feature = "tmux_1_8")]
    resize_pane.zoom();
    #[cfg(feature = "tmux_0_9")]
    resize_pane.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_8")]
    resize_pane.width(2);
    #[cfg(feature = "tmux_1_8")]
    resize_pane.height(3);
    #[cfg(feature = "tmux_0_9")]
    resize_pane.adjustment("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "resize-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "resizep";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_9")]
    s.push("-D");
    #[cfg(feature = "tmux_1_8")]
    s.push("-L");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_1_8")]
    s.push("-R");
    #[cfg(feature = "tmux_3_2")]
    s.push("-T");
    #[cfg(feature = "tmux_0_9")]
    s.push("-U");
    #[cfg(feature = "tmux_1_8")]
    s.push("-Z");
    #[cfg(feature = "tmux_0_9")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-x", "2"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-y", "3"]);
    #[cfg(feature = "tmux_0_9")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let resize_pane = resize_pane.build().to_vec();

    assert_eq!(resize_pane, s);
}
