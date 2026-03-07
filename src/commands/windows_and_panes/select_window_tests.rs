// auto-generated file
//

// Select the window at target-window.
//
// # Manual
//
// tmux >=1.8:
// ```text
// select-window [-lnpT] [-t target-window]
// (alias: selectw)
// ```
//
// tmux >=1.5:
// ```text
// select-window [-lnp] [-t target-window]
// (alias: selectw)
// ```
//
// tmux >=0.8:
// ```text
// select-window [-t target-window]
// (alias: selectw)
// ```
#[test]
fn select_window() {
    use crate::SelectWindow;
    use std::borrow::Cow;

    let select_window = SelectWindow::new();
    // `[-l]`
    #[cfg(feature = "tmux_1_5")]
    let select_window = select_window.last();

    // `[-n]`
    #[cfg(feature = "tmux_1_5")]
    let select_window = select_window.next();

    // `[-p]`
    #[cfg(feature = "tmux_1_5")]
    let select_window = select_window.previous();

    // `[-T]`
    #[cfg(feature = "tmux_1_8")]
    let select_window = select_window.switch();

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let select_window = select_window.target_window("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-l");
    #[cfg(feature = "tmux_1_5")]
    v.push("-n");
    #[cfg(feature = "tmux_1_5")]
    v.push("-p");
    #[cfg(feature = "tmux_1_8")]
    v.push("-T");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let select_window = select_window.build().to_vec();

    assert_eq!(select_window, v);
}
