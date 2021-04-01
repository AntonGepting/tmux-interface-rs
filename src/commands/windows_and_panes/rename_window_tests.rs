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

    let mut rename_window = RenameWindow::new();
    #[cfg(feature = "tmux_0_8")]
    rename_window.target_window(&target_window);
    #[cfg(feature = "tmux_0_8")]
    rename_window.new_name("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rename-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "renamew";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(rename_window.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(rename_window.0.bin_args, None);
    assert_eq!(rename_window.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(rename_window.0.cmd_args, Some(s));
}
