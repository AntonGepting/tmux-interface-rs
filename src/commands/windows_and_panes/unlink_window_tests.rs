// auto-generated file
//

// Unlink `target-window`
//
// # Manual
//
// tmux >=1.5:
// ```text
// unlink-window [-k] [-t target-window]
// (alias: unlinkw)
// ```
//
// tmux >=0.8:
// ```text
// unlink-window [-t target-window]
// (alias: unlinkw)
// ```
#[test]
fn unlink_window() {
    use crate::UnlinkWindow;
    use std::borrow::Cow;

    let unlink_window = UnlinkWindow::new();
    // `[-k]`
    #[cfg(feature = "tmux_1_5")]
    let unlink_window = unlink_window.detach_other();

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let unlink_window = unlink_window.target_window("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "unlink-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "unlinkw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-k");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let unlink_window = unlink_window.build().to_vec();

    assert_eq!(unlink_window, v);
}
