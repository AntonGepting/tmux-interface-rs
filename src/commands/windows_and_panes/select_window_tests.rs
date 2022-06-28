#[test]
fn select_window() {
    use crate::{SelectWindow, TargetWindow};
    use std::borrow::Cow;

    // Select the window at target-window.
    //
    // # Manual
    //
    // tmux ^1.8:
    // ```text
    // select-window [-lnpT] [-t target-window]
    // (alias: selectw)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // select-window [-lnp] [-t target-window]
    // (alias: selectw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // select-window [-t target-window]
    // (alias: selectw)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let select_window = SelectWindow::new();
    #[cfg(feature = "tmux_1_5")]
    let select_window = select_window.last();
    #[cfg(feature = "tmux_1_5")]
    let select_window = select_window.next();
    #[cfg(feature = "tmux_1_5")]
    let select_window = select_window.previous();
    #[cfg(feature = "tmux_1_8")]
    let select_window = select_window.switch();
    #[cfg(feature = "tmux_0_8")]
    let select_window = select_window.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-l");
    #[cfg(feature = "tmux_1_5")]
    s.push("-n");
    #[cfg(feature = "tmux_1_5")]
    s.push("-p");
    #[cfg(feature = "tmux_1_8")]
    s.push("-T");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let select_window = select_window.build().to_vec();

    assert_eq!(select_window, s);
}
