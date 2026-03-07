// auto-generated file
//

// Resize a pane, up, down, left or right
//
// # Manual
//
// tmux >=3.2:
// ```text
// resize-pane [-DLMRTUZ] [-t target-pane] [-x width] [-y height] [adjustment]
// (alias: resizep)
// ```
//
// tmux >=2.1:
// ```text
// resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
// (alias: resizep)
// ```
//
// tmux >=1.8:
// ```text
// resize-pane [-DLRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
// (alias: resizep)
// ```
//
// tmux >=1.5:
// ```text
// resize-pane [-DLRU] [-t target-pane] [adjustment]
// (alias: resizep)
// ```
#[test]
fn resize_pane() {
    use crate::ResizePane;
    use std::borrow::Cow;

    let resize_pane = ResizePane::new();
    // `[-D]`
    #[cfg(feature = "tmux_1_5")]
    let resize_pane = resize_pane.down();

    // `[-L]`
    #[cfg(feature = "tmux_1_5")]
    let resize_pane = resize_pane.left();

    // `[-M]`
    #[cfg(feature = "tmux_2_1")]
    let resize_pane = resize_pane.mouse();

    // `[-R]`
    #[cfg(feature = "tmux_1_5")]
    let resize_pane = resize_pane.right();

    // `[-T]`
    #[cfg(feature = "tmux_3_2")]
    let resize_pane = resize_pane.trim();

    // `[-U]`
    #[cfg(feature = "tmux_1_5")]
    let resize_pane = resize_pane.up();

    // `[-Z]`
    #[cfg(feature = "tmux_1_8")]
    let resize_pane = resize_pane.zoom();

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let resize_pane = resize_pane.target_pane("2");

    // `[-x width]`
    #[cfg(feature = "tmux_1_8")]
    let resize_pane = resize_pane.width("3");

    // `[-y height]`
    #[cfg(feature = "tmux_1_8")]
    let resize_pane = resize_pane.height("4");

    // `[adjustment]`
    #[cfg(feature = "tmux_1_5")]
    let resize_pane = resize_pane.adjustment("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "resize-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "resizep";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-D");
    #[cfg(feature = "tmux_1_5")]
    v.push("-L");
    #[cfg(feature = "tmux_2_1")]
    v.push("-M");
    #[cfg(feature = "tmux_1_5")]
    v.push("-R");
    #[cfg(feature = "tmux_3_2")]
    v.push("-T");
    #[cfg(feature = "tmux_1_5")]
    v.push("-U");
    #[cfg(feature = "tmux_1_8")]
    v.push("-Z");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_8")]
    v.extend_from_slice(&["-x", "3"]);
    #[cfg(feature = "tmux_1_8")]
    v.extend_from_slice(&["-y", "4"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("5");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let resize_pane = resize_pane.build().to_vec();

    assert_eq!(resize_pane, v);
}
