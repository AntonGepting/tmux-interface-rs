// auto-generated file
//

// Resize a window, up, down, left or right
//
// # Manual
//
// tmux >=2.9:
// ```text
// resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
// (alias: resizew)
// ```
#[test]
fn resize_window() {
    use crate::ResizeWindow;
    use std::borrow::Cow;

    let resize_window = ResizeWindow::new();
    // `[-a]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.smallest();

    // `[-A]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.largest();

    // `[-D]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.down();

    // `[-L]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.left();

    // `[-R]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.right();

    // `[-U]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.up();

    // `[-t target-window]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.target_window("1");

    // `[-x width]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.width(2);

    // `[-y height]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.height(3);

    // `[adjustment]`
    #[cfg(feature = "tmux_2_9")]
    let resize_window = resize_window.adjustment("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "resize-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "resizew";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    v.push("-a");
    #[cfg(feature = "tmux_2_9")]
    v.push("-A");
    #[cfg(feature = "tmux_2_9")]
    v.push("-D");
    #[cfg(feature = "tmux_2_9")]
    v.push("-L");
    #[cfg(feature = "tmux_2_9")]
    v.push("-R");
    #[cfg(feature = "tmux_2_9")]
    v.push("-U");
    #[cfg(feature = "tmux_2_9")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_2_9")]
    v.extend_from_slice(&["-x", "2"]);
    #[cfg(feature = "tmux_2_9")]
    v.extend_from_slice(&["-y", "3"]);
    #[cfg(feature = "tmux_2_9")]
    v.push("4");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let resize_window = resize_window.build().to_vec();

    assert_eq!(resize_window, v);
}
