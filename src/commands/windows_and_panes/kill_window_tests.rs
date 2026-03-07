// auto-generated file
//

// Kill the current window or the window at target-window, removing it from any sessions
// to which it is linked
//
// # Manual
//
// tmux >=1.7:
// ```text
// kill-window [-a] [-t target-window]
// (alias: killw)
// ```
//
// tmux >=0.8:
// ```text
// kill-window [-a] [-t target-window]
// (alias: killw)
// ```
#[test]
fn kill_window() {
    use crate::KillWindow;
    use std::borrow::Cow;

    let kill_window = KillWindow::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_7")]
    let kill_window = kill_window.parent_sighup();

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let kill_window = kill_window.target_window("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "kill-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "killw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    v.push("-a");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let kill_window = kill_window.build().to_vec();

    assert_eq!(kill_window, v);
}
