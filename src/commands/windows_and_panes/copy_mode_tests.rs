// auto-generated file
//

// Enter copy mode
//
// # Manual
//
// tmux >=3.6:
// ```text
// copy-mode [-deHMqSu] [-s src-pane] [-t target-pane]
// ```
//
// tmux >=3.5:
// ```text
// copy-mode [-deHMqu] [-s src-pane] [-t target-pane]
// ```
//
// tmux >=3.1a:
// ```text
// copy-mode [-eHMqu] [-s src-pane] [-t target-pane]
// ```
//
// tmux >=2.1:
// ```text
// copy-mode [-Meu] [-t target-pane]
// ```
//
// tmux >=1.5:
// ```text
// copy-mode [-u] [-t target-pane]
// ```
//
// tmux >=0.8:
// ```text
// copy-mode [-u] [-t target-window]
// ```
#[test]
fn copy_mode() {
    use crate::CopyMode;
    use std::borrow::Cow;

    let copy_mode = CopyMode::new();
    // `[-d]`
    #[cfg(feature = "tmux_3_5")]
    let copy_mode = copy_mode.scroll_down();

    // `[-e]`
    #[cfg(feature = "tmux_2_1")]
    let copy_mode = copy_mode.bottom_exit();

    // `[-H]`
    #[cfg(feature = "tmux_3_1a")]
    let copy_mode = copy_mode.hide_position();

    // `[-M]`
    #[cfg(feature = "tmux_2_1")]
    let copy_mode = copy_mode.mouse_drag();

    // `[-q]`
    #[cfg(feature = "tmux_3_1a")]
    let copy_mode = copy_mode.cancel();

    // `[-S]`
    #[cfg(feature = "tmux_3_6")]
    let copy_mode = copy_mode.from_src_pane();

    // `[-u]`
    #[cfg(feature = "tmux_0_8")]
    let copy_mode = copy_mode.page_up();

    // `[-s src-pane]`
    #[cfg(feature = "tmux_3_2")]
    let copy_mode = copy_mode.src_pane("1");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_mode = copy_mode.target_window("2");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let copy_mode = copy_mode.target_pane("3");

    let cmd = "copy-mode";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_5")]
    v.push("-d");
    #[cfg(feature = "tmux_2_1")]
    v.push("-e");
    #[cfg(feature = "tmux_3_1a")]
    v.push("-H");
    #[cfg(feature = "tmux_2_1")]
    v.push("-M");
    #[cfg(feature = "tmux_3_1a")]
    v.push("-q");
    #[cfg(feature = "tmux_3_6")]
    v.push("-S");
    #[cfg(feature = "tmux_0_8")]
    v.push("-u");
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-s", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let copy_mode = copy_mode.build().to_vec();

    assert_eq!(copy_mode, v);
}
