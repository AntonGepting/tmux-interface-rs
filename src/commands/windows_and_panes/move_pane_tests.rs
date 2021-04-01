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
    // tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
    // (alias: movep)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: movep)
    // ```
    let src_pane = TargetPane::Raw("2").to_string();
    let dst_pane = TargetPane::Raw("3").to_string();

    let mut move_pane = MovePane::new();
    #[cfg(feature = "tmux_1_7")]
    move_pane.left_above();
    #[cfg(feature = "tmux_1_7")]
    move_pane.detached();
    #[cfg(feature = "tmux_1_7")]
    move_pane.horizontal();
    #[cfg(feature = "tmux_1_7")]
    move_pane.vertical();
    #[cfg(feature = "tmux_1_7")]
    move_pane.size(&PaneSize::Size(1));
    #[cfg(feature = "tmux_1_7")]
    move_pane.src_pane(&src_pane);
    #[cfg(feature = "tmux_1_7")]
    move_pane.dst_pane(&dst_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "move-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "movep";

    let mut s = Vec::new();
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
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(move_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(move_pane.0.bin_args, None);
    assert_eq!(move_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(move_pane.0.cmd_args, Some(s));
}
