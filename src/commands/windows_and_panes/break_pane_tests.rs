#[test]
fn break_pane() {
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    use crate::TargetWindow;
    use crate::{BreakPane, TargetPane};
    use std::borrow::Cow;

    // Break `src-pane` off from its containing window to make it the only pane in `dst-window`
    //
    // # Manual
    //
    // tmux ^2.4:
    // ```text
    // tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^2.2:
    // ```text
    // tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
    // (alias: breakp)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux break-pane [-dP] [-F format] [-t target-pane]
    // (alias: breakp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux break-pane [-d] [-t target-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux break-pane [-d] [-p pane-index] [-t target-window]
    // (alias: breakp)
    // ```
    let src_pane = TargetPane::Raw("3").to_string();
    let dst_pane = TargetPane::Raw("4").to_string();
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let target_window = TargetWindow::Raw("4").to_string();

    let mut break_pane = BreakPane::new();
    #[cfg(feature = "tmux_0_8")]
    break_pane.detached();
    #[cfg(feature = "tmux_1_7")]
    break_pane.print();
    #[cfg(feature = "tmux_1_7")]
    break_pane.format("1");
    #[cfg(feature = "tmux_2_4")]
    break_pane.window_name("2");
    #[cfg(feature = "tmux_2_1")]
    break_pane.src_pane(&src_pane);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    break_pane.dst_pane(&dst_pane);
    #[cfg(feature = "tmux_2_2")]
    break_pane.dst_window(&dst_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    break_pane.target_pane(&dst_pane);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    break_pane.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "break-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "breakp";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_7")]
    s.push("-P");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-n", "2"]);
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-s", "3"]);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_2_2")]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    s.extend_from_slice(&["-t", "4"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(break_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(break_pane.0.bin_args, None);
    assert_eq!(break_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(break_pane.0.cmd_args, Some(s));
}
