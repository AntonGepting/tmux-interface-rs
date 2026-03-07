// auto-generated file
//

// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
//
// # Manual
//
// tmux >=3.2:
// ```text
// move-window [-abrdk] [-s src-window] [-t dst-window]
// (alias: movew)
// ```
//
// tmux >=2.1:
// ```text
// move-window [-ardk] [-s src-window] [-t dst-window]
// (alias: movew)
// ```
//
// tmux >=1.7:
// ```text
// move-window [-rdk] [-s src-window] [-t dst-window]
// (alias: movew)
// ```
//
// tmux >=1.5:
// ```text
// move-window [-dk] [-s src-window] [-t dst-window]
// (alias: movew)
// ```
//
// tmux >=0.8:
// ```text
// move-window [-d] [-s src-window] [-t dst-window]
// (alias: movew)
// ```
#[test]
fn move_window() {
    use crate::MoveWindow;
    use std::borrow::Cow;

    let move_window = MoveWindow::new();
    // `[-a]`
    #[cfg(feature = "tmux_2_1")]
    let move_window = move_window.after();

    // `[-b]`
    #[cfg(feature = "tmux_3_2")]
    let move_window = move_window.before();

    // `[-r]`
    #[cfg(feature = "tmux_1_7")]
    let move_window = move_window.renumber();

    // `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let move_window = move_window.detached();

    // `[-k]`
    #[cfg(feature = "tmux_1_5")]
    let move_window = move_window.kill();

    // `[-s src-window]`
    #[cfg(feature = "tmux_0_8")]
    let move_window = move_window.src_window("1");

    // `[-t dst-window]`
    #[cfg(feature = "tmux_0_8")]
    let move_window = move_window.dst_window("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "move-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "movew";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    v.push("-a");
    #[cfg(feature = "tmux_3_2")]
    v.push("-b");
    #[cfg(feature = "tmux_1_7")]
    v.push("-r");
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_1_5")]
    v.push("-k");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-s", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "2"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let move_window = move_window.build().to_vec();

    assert_eq!(move_window, v);
}
