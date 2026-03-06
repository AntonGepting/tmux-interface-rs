// auto-generated file
//

// Show options
//
// # Manual
//
// tmux >=3.0a:
// ```text
// show-options [-AgHpqsvw] [-t target-pane] [option]
// (alias: show)
// ```
//
// tmux >=1.8:
// ```text
// show-options [-gqsvw] [-t target-session | target-window] [option]
// (alias: show)
// ```
//
// tmux >=1.5:
// ```text
// show-options [-gsw] [-t target-session | target-window] [option]
// (alias: show)
// ```
//
// tmux >=0.8:
// ```text
// show-options [-t target-session] option value
// (alias: show)
// ```
#[test]
fn show_options() {
    use crate::ShowOptions;
    use std::borrow::Cow;

    let show_options = ShowOptions::new();
    // `[-A]`
    #[cfg(feature = "tmux_3_0a")]
    let show_options = show_options.include_inherited();

    // `[-g]`
    #[cfg(feature = "tmux_1_5")]
    let show_options = show_options.global();

    // `[-H]`
    #[cfg(feature = "tmux_3_0a")]
    let show_options = show_options.hooks();

    // `[-p]`
    #[cfg(feature = "tmux_3_0a")]
    let show_options = show_options.pane();

    // `[-q]`
    #[cfg(feature = "tmux_1_8")]
    let show_options = show_options.quiet();

    // `[-s]`
    #[cfg(feature = "tmux_1_5")]
    let show_options = show_options.server();

    // `[-v]`
    #[cfg(feature = "tmux_1_8")]
    let show_options = show_options.value();

    // `[-w]`
    #[cfg(feature = "tmux_1_5")]
    let show_options = show_options.window();

    // `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_0a")))]
    let show_options = show_options.target_session("1");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_3_0a")]
    let show_options = show_options.target_pane("2");

    // `[option]`
    #[cfg(feature = "tmux_0_8")]
    let show_options = show_options.option("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "show";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_0a")]
    v.push("-A");
    #[cfg(feature = "tmux_1_5")]
    v.push("-g");
    #[cfg(feature = "tmux_3_0a")]
    v.push("-H");
    #[cfg(feature = "tmux_3_0a")]
    v.push("-p");
    #[cfg(feature = "tmux_1_8")]
    v.push("-q");
    #[cfg(feature = "tmux_1_5")]
    v.push("-s");
    #[cfg(feature = "tmux_1_8")]
    v.push("-v");
    #[cfg(feature = "tmux_1_5")]
    v.push("-w");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_0a")))]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_3_0a")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let show_options = show_options.build().to_vec();

    assert_eq!(show_options, v);
}
