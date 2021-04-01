#[test]
fn kill_pane() {
    use crate::{KillPane, TargetPane};
    use std::borrow::Cow;

    // Destroy the given pane
    //
    // # Manual
    //
    // tmux ^1.1:
    // ```text
    // tmux kill-pane [-a] [-t target-pane]
    // (alias: killp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux kill-pane [-t target-pane]
    // (alias: killp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux kill-pane [-p pane-index] [-t target-window]
    // (alias: killp)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let mut kill_pane = KillPane::new();
    #[cfg(feature = "tmux_1_1")]
    kill_pane.all();
    #[cfg(feature = "tmux_1_0")]
    kill_pane.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    kill_pane.pane_index("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    kill_pane.target_window("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "kill-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "killp";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_1")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-p", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-t", "3"]);

    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(kill_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(kill_pane.0.bin_args, None);
    assert_eq!(kill_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(kill_pane.0.cmd_args, Some(s));
}
