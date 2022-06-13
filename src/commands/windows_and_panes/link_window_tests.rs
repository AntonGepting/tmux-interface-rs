#[test]
fn link_window() {
    use crate::{LinkWindow, TargetWindow};
    use std::borrow::Cow;

    // Link the window at src-window to the specified dst-window
    //
    // # Manual
    //
    // tmux ^2.1:
    // ```text
    // tmux link-window [-adk] [-s src-window] [-t dst-window]
    // (alias: linkw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux link-window [-dk] [-s src-window] [-t dst-window]
    // (alias: linkw)
    // ```
    let src_window = TargetWindow::Raw("1").to_string();
    let dst_window = TargetWindow::Raw("2").to_string();

    let link_window = LinkWindow::new();
    #[cfg(feature = "tmux_2_1")]
    let link_window = link_window.add();
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window.detached();
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window.kill();
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window.src_window(&src_window);
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window.dst_window(&dst_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "link-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "linkw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_0_8")]
    s.push("-k");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let link_window = link_window.build().to_vec();

    assert_eq!(link_window, s);
}
