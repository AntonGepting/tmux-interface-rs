#[test]
fn clock_mode() {
    use crate::ClockMode;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // clock-mode [-t target-pane]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // clock-mode [-t target-window]
    // ```
    let clock_mode = ClockMode::new();
    #[cfg(feature = "tmux_1_0")]
    let clock_mode = clock_mode.target_pane("1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    let clock_mode = clock_mode.target_window("2");

    let cmd = "clock-mode";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let clock_mode = clock_mode.build().to_vec();

    assert_eq!(clock_mode, s);
}
