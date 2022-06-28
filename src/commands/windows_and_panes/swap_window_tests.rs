#[test]
fn swap_window() {
    use crate::{SwapWindow, TargetWindow};
    use std::borrow::Cow;

    // This is similar to link-window, except the source and destination windows are swapped
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // swap-window [-d] [-s src-window] [-t dst-window]
    // (alias: swapw)
    // ```
    let src_window = TargetWindow::Raw("1").to_string();
    let dst_window = TargetWindow::Raw("2").to_string();

    let swap_window = SwapWindow::new();
    #[cfg(feature = "tmux_0_8")]
    let swap_window = swap_window.detached();
    #[cfg(feature = "tmux_0_8")]
    let swap_window = swap_window.src_window(&src_window);
    #[cfg(feature = "tmux_0_8")]
    let swap_window = swap_window.dst_window(&dst_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "swap-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "swapw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);

    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let swap_window = swap_window.build().to_vec();

    assert_eq!(swap_window, s);
}
