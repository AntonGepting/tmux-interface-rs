// auto-generated file
//

// Display a large clock
//
// # Manual
//
// tmux >=1.5:
// ```text
// clock-mode [-t target-pane]
// ```
//
// tmux >=0.8:
// ```text
// clock-mode [-t target-window]
// ```
#[test]
fn clock_mode() {
    use crate::ClockMode;
    use std::borrow::Cow;

    let clock_mode = ClockMode::new();
    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let clock_mode = clock_mode.target_window("1");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let clock_mode = clock_mode.target_pane("2");

    let cmd = "clock-mode";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "2"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let clock_mode = clock_mode.build().to_vec();

    assert_eq!(clock_mode, v);
}
