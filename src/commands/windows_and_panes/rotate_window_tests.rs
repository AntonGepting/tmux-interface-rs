// auto-generated file
//

// Rotate the positions of the panes within a window
//
// # Manual
//
// tmux >=3.1:
// ```text
// rotate-window [-DUZ] [-t target-window]
// (alias: rotatew)
// ```
//
// tmux >=0.8:
// ```text
// rotate-window [-DU] [-t target-window]
// (alias: rotatew)
// ```
#[test]
fn rotate_window() {
    use crate::RotateWindow;
    use std::borrow::Cow;

    let rotate_window = RotateWindow::new();
    // `[-D]`
    #[cfg(feature = "tmux_0_8")]
    let rotate_window = rotate_window.down();

    // `[-U]`
    #[cfg(feature = "tmux_0_8")]
    let rotate_window = rotate_window.up();

    // `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    let rotate_window = rotate_window.keep_zoomed();

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let rotate_window = rotate_window.target_window("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rotate-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "rotatew";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.push("-D");
    #[cfg(feature = "tmux_0_8")]
    v.push("-U");
    #[cfg(feature = "tmux_3_1")]
    v.push("-Z");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let rotate_window = rotate_window.build().to_vec();

    assert_eq!(rotate_window, v);
}
