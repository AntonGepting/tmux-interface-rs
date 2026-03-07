// auto-generated file
//

// Create new window
//
// # Manual
//
// tmux >=3.2:
// ```text
// new-window [-abdkPS] [-c start-directory] [-e environment] [-F format] [-n window-name]
// [-t target-window] [shell-command]
// (alias: neww)
// ```
//
// tmux >=3.0:
// ```text
// new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name] [-t
// target-window] [shell-command]
// (alias: neww)
// ```
//
// tmux >=1.7:
// ```text
// new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
// [shell-command]
// (alias: neww)
// ```
//
// tmux >=1.5:
// ```text
// new-window [-adkP] [-n window-name] [-t target-window] [shell-command]
// (alias: neww)
// ```
//
// tmux >=0.8:
// ```text
// new-window [-d] [-n window-name] [-t target-window] [command]
// (alias: neww)
// ```
#[test]
fn new_window() {
    use crate::NewWindow;
    use std::borrow::Cow;

    let new_window = NewWindow::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_5")]
    let new_window = new_window.after();

    // `[-b]`
    #[cfg(feature = "tmux_3_2")]
    let new_window = new_window.before();

    // `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let new_window = new_window.detached();

    // `[-k]`
    #[cfg(feature = "tmux_1_5")]
    let new_window = new_window.kill();

    // `[-P]`
    #[cfg(feature = "tmux_1_5")]
    let new_window = new_window.print();

    // `[-S]`
    #[cfg(feature = "tmux_3_2")]
    let new_window = new_window.select();

    // `[-c start-directory]`
    #[cfg(feature = "tmux_1_7")]
    let new_window = new_window.start_directory("1");

    // `[-e environment]`
    #[cfg(feature = "tmux_3_0")]
    let new_window = new_window.environment("2");

    // `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    let new_window = new_window.format("3");

    // `[-n window-name]`
    #[cfg(feature = "tmux_0_8")]
    let new_window = new_window.window_name("4");

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let new_window = new_window.target_window("5");

    // `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    let new_window = new_window.shell_command("6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "new-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "neww";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-a");
    #[cfg(feature = "tmux_3_2")]
    v.push("-b");
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_1_5")]
    v.push("-k");
    #[cfg(feature = "tmux_1_5")]
    v.push("-P");
    #[cfg(feature = "tmux_3_2")]
    v.push("-S");
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_0")]
    v.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-F", "3"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-n", "4"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("6");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let new_window = new_window.build().to_vec();

    assert_eq!(new_window, v);
}
