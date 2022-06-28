#[test]
fn move_pane() {
    use crate::{MovePane, PaneSize, TargetPane};
    use std::borrow::Cow;

    // Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
    // (alias: movep)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: movep)
    // ```
    let src_pane = TargetPane::Raw("2").to_string();
    let dst_pane = TargetPane::Raw("3").to_string();

    let move_pane = MovePane::new();
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.left_above();
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.detached();
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.horizontal();
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.vertical();
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.size(&PaneSize::Size(1));
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.src_pane(&src_pane);
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.dst_pane(&dst_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "move-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "movep";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    s.push("-b");
    #[cfg(feature = "tmux_1_7")]
    s.push("-d");
    #[cfg(feature = "tmux_1_7")]
    s.push("-h");
    #[cfg(feature = "tmux_1_7")]
    s.push("-v");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-l", "1"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-s", "2"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let move_pane = move_pane.build().to_vec();

    assert_eq!(move_pane, s);
}
