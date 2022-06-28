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
    // tmux ^3.2:
    // ```text
    // break-pane [-abdP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^2.2:
    // ```text
    // break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
    // (alias: breakp)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // break-pane [-dP] [-F format] [-t target-pane]
    // (alias: breakp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // break-pane [-d] [-t target-window]
    // (alias: breakp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // break-pane [-d] [-p pane-index] [-t target-window]
    // (alias: breakp)
    // ```
    let src_pane = TargetPane::Raw("3").to_string();
    let dst_pane = TargetPane::Raw("4").to_string();
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let target_window = TargetWindow::Raw("4").to_string();

    let break_pane = BreakPane::new();
    #[cfg(feature = "tmux_3_2")]
    let break_pane = break_pane.after();
    #[cfg(feature = "tmux_3_2")]
    let break_pane = break_pane.before();
    #[cfg(feature = "tmux_0_8")]
    let break_pane = break_pane.detached();
    #[cfg(feature = "tmux_1_7")]
    let break_pane = break_pane.print();
    #[cfg(feature = "tmux_1_7")]
    let break_pane = break_pane.format("1");
    #[cfg(feature = "tmux_2_4")]
    let break_pane = break_pane.window_name("2");
    #[cfg(feature = "tmux_2_1")]
    let break_pane = break_pane.src_pane(&src_pane);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    let break_pane = break_pane.dst_pane(&dst_pane);
    #[cfg(feature = "tmux_2_2")]
    let break_pane = break_pane.dst_window(&dst_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    let break_pane = break_pane.target_pane(&dst_pane);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let break_pane = break_pane.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "break-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "breakp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-a");
    #[cfg(feature = "tmux_3_2")]
    s.push("-b");
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
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let break_pane = break_pane.build().to_vec();

    assert_eq!(break_pane, s);
}
