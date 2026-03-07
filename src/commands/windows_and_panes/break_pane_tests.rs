// auto-generated file
//

// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
//
// # Manual
//
// tmux >=3.2:
// ```text
// break-pane [-abdP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
// (alias: breakp)
// ```
//
// tmux >=2.4:
// ```text
// break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
// (alias: breakp)
// ```
//
// tmux >=2.2:
// ```text
// break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
// (alias: breakp)
// ```
//
// tmux >=2.1:
// ```text
// break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
// (alias: breakp)
// ```
//
// tmux >=1.7:
// ```text
// break-pane [-dP] [-F format] [-t target-pane]
// (alias: breakp)
// ```
//
// tmux >=1.5:
// ```text
// break-pane [-d] [-t target-pane]
// (alias: breakp)
// ```
//
// tmux >=0.8:
// ```text
// break-pane [-d] [-p pane-index] [-t target-window]
// (alias: breakp)
// ```
#[test]
fn break_pane() {
    use crate::BreakPane;
    use std::borrow::Cow;

    let break_pane = BreakPane::new();
    // `[-a]`
    #[cfg(feature = "tmux_3_2")]
    let break_pane = break_pane.after();

    // `[-b]`
    #[cfg(feature = "tmux_3_2")]
    let break_pane = break_pane.before();

    // `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let break_pane = break_pane.detached();

    // `[-P]`
    #[cfg(feature = "tmux_1_7")]
    let break_pane = break_pane.print();

    // `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    let break_pane = break_pane.format("1");

    // `[-n window-name]`
    #[cfg(feature = "tmux_2_4")]
    let break_pane = break_pane.window_name("2");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let break_pane = break_pane.target_window("3");

    // `[-t target-pane]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let break_pane = break_pane.target_pane("4");

    // `[-s src-pane]`
    #[cfg(feature = "tmux_2_1")]
    let break_pane = break_pane.src_pane("5");

    // `[-t dst-pane]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_1")))]
    let break_pane = break_pane.dst_pane("6");

    // `[-t dst-window]`
    #[cfg(feature = "tmux_2_2")]
    let break_pane = break_pane.dst_window("7");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "break-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "breakp";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    v.push("-a");
    #[cfg(feature = "tmux_3_2")]
    v.push("-b");
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_1_7")]
    v.push("-P");
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_4")]
    v.extend_from_slice(&["-n", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    v.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_2_1")]
    v.extend_from_slice(&["-s", "5"]);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_1")))]
    v.extend_from_slice(&["-t", "6"]);
    #[cfg(feature = "tmux_2_2")]
    v.extend_from_slice(&["-t", "7"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let break_pane = break_pane.build().to_vec();

    assert_eq!(break_pane, v);
}
