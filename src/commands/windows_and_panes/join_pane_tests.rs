// TODO: size and percentage both test
#[test]
fn join_pane() {
    use crate::{JoinPane, PaneSize, TargetPane};
    use std::borrow::Cow;

    // Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
    // and move `src-pane` into the space
    //
    // # Manual
    //
    // tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // tmux ^3.1:
    // ```text
    // tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux join-pane [-dhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    // (alias: joinp)
    // ```
    let src_pane = TargetPane::Raw("2").to_string();
    let dst_pane = TargetPane::Raw("3").to_string();

    let mut join_pane = JoinPane::new();
    #[cfg(feature = "tmux_2_6")]
    join_pane.left_above();
    #[cfg(feature = "tmux_1_2")]
    join_pane.detached();
    #[cfg(feature = "tmux_2_6")]
    join_pane.full_size();
    #[cfg(feature = "tmux_1_2")]
    join_pane.horizontal();
    #[cfg(feature = "tmux_1_2")]
    join_pane.vertical();
    #[cfg(feature = "tmux_1_2")]
    join_pane.size(&PaneSize::Percentage(1));
    #[cfg(feature = "tmux_1_2")]
    join_pane.src_pane(&src_pane);
    #[cfg(feature = "tmux_1_2")]
    join_pane.dst_pane(&dst_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "join-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "joinp";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_6")]
    s.push("-b");
    #[cfg(feature = "tmux_1_2")]
    s.push("-d");
    #[cfg(feature = "tmux_2_6")]
    s.push("-f");
    #[cfg(feature = "tmux_1_2")]
    s.push("-h");
    #[cfg(feature = "tmux_1_2")]
    s.push("-v");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_1")))]
    s.extend_from_slice(&["-l", "1%"]);
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-p", "1"]);
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-s", "2"]);
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-t", "3"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(join_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(join_pane.0.bin_args, None);
    assert_eq!(join_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(join_pane.0.cmd_args, Some(s));
}
