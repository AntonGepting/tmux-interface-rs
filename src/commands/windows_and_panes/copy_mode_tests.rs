#[test]
fn copy_mode() {
    use crate::{CopyMode, TargetPane};
    use std::borrow::Cow;

    // Enter copy mode
    //
    // # Manual
    //
    // tmux ^2.1:
    // ```text
    // tmux copy-mode [-Meu] [-t target-pane]
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux copy-mode [-u] [-t target-pane]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux copy-mode [-u] [-t target-window]
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let mut copy_mode = CopyMode::new();
    #[cfg(feature = "tmux_2_1")]
    copy_mode.mouse_drag();
    #[cfg(feature = "tmux_2_1")]
    copy_mode.bottom_exit();
    #[cfg(feature = "tmux_0_8")]
    copy_mode.page_up();
    #[cfg(feature = "tmux_1_0")]
    copy_mode.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    copy_mode.target_window(&target_pane);

    let cmd = "copy-mode";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_2_1")]
    s.push("-e");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(copy_mode.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(copy_mode.0.bin_args, None);
    assert_eq!(copy_mode.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(copy_mode.0.cmd_args, Some(s));
}
