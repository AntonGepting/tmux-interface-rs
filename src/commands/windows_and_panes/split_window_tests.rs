// auto-generated file
//

// Create a new pane by splitting target-pane
//
// # Manual
//
// tmux >=3.2:
// ```text
// split-window [-bdfhIvPZ] [-c start-directory] [-e environment] [-l size] [-t target-pane]
// [shell-command] [-F format]
// (alias: splitw)
// ```
//
// tmux >=3.1:
// ```text
// split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
// [shell-command] [-F format]
// (alias: splitw)
// ```
//
// tmux >=3.0:
// ```text
// split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size | -p percentage]
// [-t target-pane] [shell-command] [-F format]
// (alias: splitw)
// ```
//
// tmux >=2.4:
// ```text
// split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
// [shell-command] [-F format]
// (alias: splitw)
// ```
//
// tmux >=2.0:
// ```text
// split-window [-bdhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
// [shell-command] [-F format]
// (alias: splitw)
// ```
//
// tmux >=1.7:
// ```text
// split-window [-dhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
// [shell-command] [-F format]
// (alias: splitw)
// ```
//
// tmux >=1.5:
// ```text
// split-window [-dhvP] [-l size | -p percentage] [-t target-pane] [shell-command]
// (alias: splitw)
// ```
//
// tmux >=0.8:
// ```text
// split-window [-d] [-l size | -p percentage] [-t target-window] [command]
// (alias: splitw)
// ```
#[test]
fn split_window() {
    use crate::{PaneSize, SplitWindow};
    use std::borrow::Cow;

    let split_window = SplitWindow::new();
    // `[-b]`
    #[cfg(feature = "tmux_2_0")]
    let split_window = split_window.before();

    // `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let split_window = split_window.detached();

    // `[-f]`
    #[cfg(feature = "tmux_2_4")]
    let split_window = split_window.full_size();

    // `[-h]`
    #[cfg(feature = "tmux_1_5")]
    let split_window = split_window.horizontal();

    // `[-I]`
    #[cfg(feature = "tmux_3_0")]
    let split_window = split_window.stdin_forward();

    // `[-v]`
    #[cfg(feature = "tmux_1_5")]
    let split_window = split_window.vertical();

    // `[-P]`
    #[cfg(feature = "tmux_1_5")]
    let split_window = split_window.print();

    // `[-Z]`
    #[cfg(feature = "tmux_3_2")]
    let split_window = split_window.zoom();

    // `[-c start-directory]`
    #[cfg(feature = "tmux_1_7")]
    let split_window = split_window.start_directory("1");

    // `[-e environment]`
    #[cfg(feature = "tmux_3_1")]
    let split_window = split_window.environment("2");

    // `[-l size]`
    #[cfg(feature = "tmux_0_8")]
    let split_window = split_window.size(&PaneSize::Size(3));

    // `[-p precentage]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
    let split_window = split_window.precentage("4");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let split_window = split_window.target_window("5");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let split_window = split_window.target_pane("6");

    // `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    let split_window = split_window.format("7");

    // `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    let split_window = split_window.shell_command("8");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "split-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "splitw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_0")]
    v.push("-b");
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_2_4")]
    v.push("-f");
    #[cfg(feature = "tmux_1_5")]
    v.push("-h");
    #[cfg(feature = "tmux_3_0")]
    v.push("-I");
    #[cfg(feature = "tmux_1_5")]
    v.push("-v");
    #[cfg(feature = "tmux_1_5")]
    v.push("-P");
    #[cfg(feature = "tmux_3_2")]
    v.push("-Z");
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_1")]
    v.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-l", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
    v.extend_from_slice(&["-p", "4"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "6"]);
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-F", "7"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("8");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let split_window = split_window.build().to_vec();

    assert_eq!(split_window, v);
}
