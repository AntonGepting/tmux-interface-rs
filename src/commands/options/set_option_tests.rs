// auto-generated file
//

// Set a pane/window/session/server option
//
// # Manual
//
// tmux >=3.2:
// ```text
// set-option [-aFgopqsuUw] [-t target-pane] option value
// (alias: set)
// ```
//
// tmux >=3.0a:
// ```text
// set-option [-aFgopqsuw] [-t target-pane] option value
// (alias: set)
// ```
//
// tmux >=2.6:
// ```text
// set-option [-aFgoqsuw] [-t target-session | target-window] option value
// (alias: set)
// ```
//
// tmux >=1.8:
// ```text
// set-option [-agoqsuw] [-t target-session | target-window] option value
// (alias: set)
// ```
//
// tmux >=1.7:
// ```text
// set-option [-agqsuw] [-t target-session | target-window] option value
// (alias: set)
// ```
//
// tmux >=1.5:
// ```text
// set-option [-agsuw] [-t target-session | target-window] option value
// (alias: set)
// ```
//
// tmux >=0.8:
// ```text
// set-option [-gu] [-t target-session] option value
// (alias: set)
// ```
#[test]
fn set_option() {
    use crate::SetOption;
    use std::borrow::Cow;

    let set_option = SetOption::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_5")]
    let set_option = set_option.append();

    // `[-F]`
    #[cfg(feature = "tmux_2_6")]
    let set_option = set_option.format();

    // `[-g]`
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option.global();

    // `[-o]`
    #[cfg(feature = "tmux_1_8")]
    let set_option = set_option.not_overwrite();

    // `[-p]`
    #[cfg(feature = "tmux_3_0a")]
    let set_option = set_option.pane();

    // `[-q]`
    #[cfg(feature = "tmux_1_7")]
    let set_option = set_option.quiet();

    // `[-s]`
    #[cfg(feature = "tmux_1_5")]
    let set_option = set_option.server();

    // `[-u]`
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option.unset();

    // `[-U]`
    #[cfg(feature = "tmux_3_2")]
    let set_option = set_option.unset_on_all();

    // `[-w]`
    #[cfg(feature = "tmux_1_5")]
    let set_option = set_option.window();

    // `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let set_option = set_option.target_session("1");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_0a")))]
    let set_option = set_option.target_window("2");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_3_0a")]
    let set_option = set_option.target_pane("3");

    // `[option]`
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option.option("4");

    // `[value]`
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option.value("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "set";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-a");
    #[cfg(feature = "tmux_2_6")]
    v.push("-F");
    #[cfg(feature = "tmux_0_8")]
    v.push("-g");
    #[cfg(feature = "tmux_1_8")]
    v.push("-o");
    #[cfg(feature = "tmux_3_0a")]
    v.push("-p");
    #[cfg(feature = "tmux_1_7")]
    v.push("-q");
    #[cfg(feature = "tmux_1_5")]
    v.push("-s");
    #[cfg(feature = "tmux_0_8")]
    v.push("-u");
    #[cfg(feature = "tmux_3_2")]
    v.push("-U");
    #[cfg(feature = "tmux_1_5")]
    v.push("-w");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_0a")))]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_3_0a")]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("4");
    #[cfg(feature = "tmux_0_8")]
    v.push("5");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let set_option = set_option.build().to_vec();

    assert_eq!(set_option, v);
}
