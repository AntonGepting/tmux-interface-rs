// auto-generated file
//

// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
//
// # Manual
//
// tmux >=3.2:
// ```text
// move-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
// (alias: movep)
// ```
//
// tmux >=3.1:
// ```text
// move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
// (alias: movep)
// ```
//
// tmux >=1.7:
// ```text
// move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
// (alias: movep)
// ```
#[test]
fn move_pane() {
    use crate::{MovePane, PaneSize};
    use std::borrow::Cow;

    let move_pane = MovePane::new();
    // `[-b]`
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.left_above();

    // `[-d]`
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.detached();

    // `[-f]`
    #[cfg(feature = "tmux_3_2")]
    let move_pane = move_pane.full_size();

    // `[-h]`
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.horizontal();

    // `[-v]`
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.vertical();

    // `[-l size]`
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.size(&PaneSize::Size(1));

    // `[-p percentage]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_3_1")))]
    let move_pane = move_pane.percentage("2");

    // `[-s src-pane]`
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.src_pane("3");

    // `[-t dst-pane]`
    #[cfg(feature = "tmux_1_7")]
    let move_pane = move_pane.dst_pane("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "move-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "movep";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    v.push("-b");
    #[cfg(feature = "tmux_1_7")]
    v.push("-d");
    #[cfg(feature = "tmux_3_2")]
    v.push("-f");
    #[cfg(feature = "tmux_1_7")]
    v.push("-h");
    #[cfg(feature = "tmux_1_7")]
    v.push("-v");
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-l", "1"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_3_1")))]
    v.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-s", "3"]);
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-t", "4"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let move_pane = move_pane.build().to_vec();

    assert_eq!(move_pane, v);
}
