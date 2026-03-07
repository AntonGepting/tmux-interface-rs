// auto-generated file
//

use crate::PaneSize;

// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
// and move `src-pane` into the space
//
// # Manual
//
// tmux >=3.1:
// ```text
// join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
// (alias: joinp)
// ```
//
// tmux >=1.7:
// ```text
// join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
// (alias: joinp)
// ```
//
// tmux >=1.5:
// ```text
// join-pane [-dhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
// (alias: joinp)
// ```
#[test]
fn join_pane() {
    use crate::{JoinPane, PaneSize};
    use std::borrow::Cow;

    let join_pane = JoinPane::new();
    // `[-b]`
    #[cfg(feature = "tmux_1_7")]
    let join_pane = join_pane.left_above();

    // `[-d]`
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane.detached();

    // `[-f]`
    #[cfg(feature = "tmux_3_1")]
    let join_pane = join_pane.full_size();

    // `[-h]`
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane.horizontal();

    // `[-v]`
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane.vertical();

    // `[-l size]`
    // `[-p percentage]`
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane.size(&PaneSize::Size(1));

    // `[-s src-pane]`
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane.src_pane("3");

    // `[-t dst-pane]`
    #[cfg(feature = "tmux_1_5")]
    let join_pane = join_pane.dst_pane("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "join-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "joinp";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    v.push("-b");
    #[cfg(feature = "tmux_1_5")]
    v.push("-d");
    #[cfg(feature = "tmux_3_1")]
    v.push("-f");
    #[cfg(feature = "tmux_1_5")]
    v.push("-h");
    #[cfg(feature = "tmux_1_5")]
    v.push("-v");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-l", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-s", "3"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "4"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let join_pane = join_pane.build().to_vec();

    assert_eq!(join_pane, v);
}
