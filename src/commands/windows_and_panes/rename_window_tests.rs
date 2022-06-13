#[test]
fn rename_window() {
    use crate::{RenameWindow, TargetWindow};
    use std::borrow::Cow;

    // Rename the current window, or the window at target-window if specified, to new-name
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // tmux rename-window [-t target-window] new-name
    // (alias: renamew)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let rename_window = RenameWindow::new();
    #[cfg(feature = "tmux_0_8")]
    let rename_window = rename_window.target_window(&target_window);
    #[cfg(feature = "tmux_0_8")]
    let rename_window = rename_window.new_name("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rename-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "renamew";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let rename_window = rename_window.build().to_vec();

    assert_eq!(rename_window, s);
}
