// auto-generated file
//

// Destroy the given pane
//
// # Manual
//
// tmux >=1.5:
// ```text
// kill-pane [-a] [-t target-pane]
// (alias: killp)
// ```
//
// tmux >=0.8:
// ```text
// kill-pane [-p pane-index] [-t target-window]
// (alias: killp)
// ```
#[test]
fn kill_pane() {
    use crate::KillPane;
    use std::borrow::Cow;

    let kill_pane = KillPane::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_5")]
    let kill_pane = kill_pane.all();

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let kill_pane = kill_pane.target_window("1");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let kill_pane = kill_pane.target_pane("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "kill-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "killp";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-a");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "2"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let kill_pane = kill_pane.build().to_vec();

    assert_eq!(kill_pane, v);
}
