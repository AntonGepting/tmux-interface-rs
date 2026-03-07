// auto-generated file
//

// Link the window at src-window to the specified dst-window
//
// # Manual
//
// tmux >=3.2:
// ```text
// link-window [-abdk] [-s src-window] [-t dst-window]
// (alias: linkw)
// ```
//
// tmux >=2.1:
// ```text
// link-window [-adk] [-s src-window] [-t dst-window]
// (alias: linkw)
// ```
//
// tmux >=0.8:
// ```text
// link-window [-dk] [-s src-window] [-t dst-window]
// (alias: linkw)
// ```
#[test]
fn link_window() {
    use crate::LinkWindow;
    use std::borrow::Cow;

    let link_window = LinkWindow::new();
    // `[-a]`
    #[cfg(feature = "tmux_2_1")]
    let link_window = link_window.after();

    // `[-b]`
    #[cfg(feature = "tmux_3_2")]
    let link_window = link_window.before();

    // `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window.detached();

    // `[-k]`
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window.kill();

    // `[-s src-window]`
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window.src_window("1");

    // `[-t dst-window]`
    #[cfg(feature = "tmux_0_8")]
    let link_window = link_window.dst_window("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "link-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "linkw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    v.push("-a");
    #[cfg(feature = "tmux_3_2")]
    v.push("-b");
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_0_8")]
    v.push("-k");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "2"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let link_window = link_window.build().to_vec();

    assert_eq!(link_window, v);
}
