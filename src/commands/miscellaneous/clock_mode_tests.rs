#[test]
fn clock_mode() {
    use crate::ClockMode;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // tmux clock-mode [-t target-pane]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux clock-mode [-t target-window]
    // ```
    let mut clock_mode = ClockMode::new();
    #[cfg(feature = "tmux_1_0")]
    clock_mode.target_pane("1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    clock_mode.target_window("2");

    let cmd = "clock-mode";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-t", "2"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(clock_mode.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(clock_mode.0.bin_args, None);
    assert_eq!(clock_mode.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(clock_mode.0.cmd_args, Some(s));
}
