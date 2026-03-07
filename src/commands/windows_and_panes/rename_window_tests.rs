// auto-generated file
//

// Rename the current window, or the window at target-window if specified, to new-name
//
// # Manual
//
// tmux >=0.8:
// ```text
// rename-window [-t target-window] new-name
// (alias: renamew)
// ```
#[test]
fn rename_window() {
    use crate::RenameWindow;
    use std::borrow::Cow;

    let rename_window = RenameWindow::new();
    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let rename_window = rename_window.target_window("1");

    // `[new-name]`
    #[cfg(feature = "tmux_0_8")]
    let rename_window = rename_window.new_name("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rename-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "renamew";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("2");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let rename_window = rename_window.build().to_vec();

    assert_eq!(rename_window, v);
}
