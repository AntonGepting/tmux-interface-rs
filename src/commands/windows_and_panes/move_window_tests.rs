#[test]
fn move_window() {
    use crate::{MoveWindow, TargetWindow};
    use std::borrow::Cow;

    // Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux move-window [-abrdk] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux move-window [-ardk] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux move-window [-rdk] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```
    //
    // tmux ^1.3:
    // ```text
    // tmux move-window [-dk] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux move-window [-d] [-s src-window] [-t dst-window]
    // (alias: movew)
    // ```

    let src_pane = TargetWindow::Raw("1").to_string();
    let dst_pane = TargetWindow::Raw("2").to_string();

    let move_window = MoveWindow::new();
    #[cfg(feature = "tmux_2_1")]
    let move_window = move_window.after();
    #[cfg(feature = "tmux_3_2")]
    let move_window = move_window.before();
    #[cfg(feature = "tmux_1_7")]
    let move_window = move_window.renumber();
    #[cfg(feature = "tmux_0_8")]
    let move_window = move_window.detached();
    #[cfg(feature = "tmux_1_3")]
    let move_window = move_window.kill();
    #[cfg(feature = "tmux_0_8")]
    let move_window = move_window.src_window(&src_pane);
    #[cfg(feature = "tmux_0_8")]
    let move_window = move_window.dst_window(&dst_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "move-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "movew";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    s.push("-a");
    #[cfg(feature = "tmux_3_2")]
    s.push("-b");
    #[cfg(feature = "tmux_1_7")]
    s.push("-r");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_3")]
    s.push("-k");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let move_window = move_window.build().to_vec();

    assert_eq!(move_window, s);
}
