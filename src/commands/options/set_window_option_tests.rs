// auto-generated file
//

// Set a window option
//
// # Manual
//
// tmux >=2.6 && <=3.0:
// ```text
// set-window-option [-aFgoqu] [-t target-window] option value
// (alias: setw)
// ```
//
// tmux >=1.9:
// ```text
// set-window-option [-agoqu] [-t target-window] option value
// (alias: setw)
// ```
//
// tmux >=1.7:
// ```text
// set-window-option [-agqu] [-t target-window] option value
// (alias: setw)
// ```
//
// tmux >=1.0:
// ```text
// set-window-option [-agu] [-t target-window] option value
// (alias: setw)
// ```
//
// tmux >=0.8:
// ```text
// set-window-option [-gu] [-t target-window] option value
// (alias: setw)
// ```
#[test]
fn set_window_option() {
    use crate::SetWindowOption;
    use std::borrow::Cow;

    let set_window_option = SetWindowOption::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_5")]
    let set_window_option = set_window_option.append();

    // `[-F]`
    #[cfg(feature = "tmux_2_6")]
    let set_window_option = set_window_option.format();

    // `[-g]`
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option.global();

    // `[-o]`
    #[cfg(feature = "tmux_1_9")]
    let set_window_option = set_window_option.not_overwrite();

    // `[-q]`
    #[cfg(feature = "tmux_1_7")]
    let set_window_option = set_window_option.quiet();

    // `[-u]`
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option.unset();

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option.target_window("1");

    // `[option]`
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option.option("2");

    // `[value]`
    #[cfg(feature = "tmux_0_8")]
    let set_window_option = set_window_option.value("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-window-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-a");
    #[cfg(feature = "tmux_2_6")]
    v.push("-F");
    #[cfg(feature = "tmux_0_8")]
    v.push("-g");
    #[cfg(feature = "tmux_1_9")]
    v.push("-o");
    #[cfg(feature = "tmux_1_7")]
    v.push("-q");
    #[cfg(feature = "tmux_0_8")]
    v.push("-u");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("2");
    #[cfg(feature = "tmux_0_8")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let set_window_option = set_window_option.build().to_vec();

    assert_eq!(set_window_option, v);
}
