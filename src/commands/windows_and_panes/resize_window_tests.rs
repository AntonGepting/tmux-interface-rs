#[test]
fn resize_window() {
    use crate::{ResizeWindow, TargetWindow};
    use std::borrow::Cow;

    // Resize a window, up, down, left or right
    //
    // # Manual
    //
    // tmux ^2.9a:
    // ```text
    // resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
    // (alias: resizew)
    let target_window = TargetWindow::Raw("1").to_string();

    let resize_window = ResizeWindow::new();
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.smallest();
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.largest();
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.down();
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.left();
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.right();
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.up();
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.target_window(&target_window);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.width(2);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.height(3);
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.adjustment("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "resize-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "resizew";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    s.push("-a");
    #[cfg(feature = "tmux_2_9")]
    s.push("-A");
    #[cfg(feature = "tmux_2_9")]
    s.push("-D");
    #[cfg(feature = "tmux_2_9")]
    s.push("-L");
    #[cfg(feature = "tmux_2_9")]
    s.push("-R");
    #[cfg(feature = "tmux_2_9")]
    s.push("-U");
    #[cfg(feature = "tmux_2_9")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_2_9")]
    s.extend_from_slice(&["-x", "2"]);
    #[cfg(feature = "tmux_2_9")]
    s.extend_from_slice(&["-y", "3"]);
    #[cfg(feature = "tmux_2_9")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let resize_window = resize_window.build().to_vec();

    assert_eq!(resize_window, s);
}
