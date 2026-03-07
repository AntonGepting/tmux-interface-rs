// auto-generated file
//

// This is similar to link-window, except the source and destination windows are swapped
//
// # Manual
//
// tmux >=0.8:
// ```text
// swap-window [-d] [-s src-window] [-t dst-window]
// (alias: swapw)
// ```
#[test]
fn swap_window() {
    use crate::SwapWindow;
    use std::borrow::Cow;

    let swap_window = SwapWindow::new();
    // `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let swap_window = swap_window.detached();

    // `[-s src-window]`
    #[cfg(feature = "tmux_0_8")]
    let swap_window = swap_window.src_window("1");

    // `[-t dst-window]`
    #[cfg(feature = "tmux_0_8")]
    let swap_window = swap_window.dst_window("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "swap-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "swapw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "2"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let swap_window = swap_window.build().to_vec();

    assert_eq!(swap_window, v);
}
