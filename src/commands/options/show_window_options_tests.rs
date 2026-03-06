// auto-generated file
//

// List the window options for `target-window`
//
// # Manual
//
// tmux >=1.8 && <=3.0:
// ```text
// show-window-options [-gv] [-t target-window] [option]
// (alias: showw)
// ```
//
// tmux >=1.5:
// ```text
// show-window-options [-g] [-t target-window] [option]
// (alias: showw)
// ```
//
// tmux >=0.8:
// ```text
// show-window-options [-t target-window] option value
// (alias: showw)
// ```
#[test]
fn show_window_options() {
    use crate::ShowWindowOptions;
    use std::borrow::Cow;

    let show_window_options = ShowWindowOptions::new();
    // `[-g]`
    #[cfg(feature = "tmux_1_5")]
    let show_window_options = show_window_options.global();

    // `[-v]`
    #[cfg(feature = "tmux_1_8")]
    let show_window_options = show_window_options.only_value();

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let show_window_options = show_window_options.target_window("1");

    // `[option]`
    #[cfg(feature = "tmux_0_8")]
    let show_window_options = show_window_options.option("2");

    // `[value]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let show_window_options = show_window_options.value("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-window-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-g");
    #[cfg(feature = "tmux_1_8")]
    v.push("-v");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let show_window_options = show_window_options.build().to_vec();

    assert_eq!(show_window_options, v);
}
