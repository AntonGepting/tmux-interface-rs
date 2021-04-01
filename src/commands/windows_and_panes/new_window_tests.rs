#[test]
fn new_window() {
    use crate::{NewWindow, TargetWindow};
    use std::borrow::Cow;

    // Structure for creating new window, using `tmux new-window` command
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name] [-t
    // target-window] [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
    // [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux new-window [-adkP] [-n window-name] [-t target-window] [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.3:
    // ```text
    // tmux new-window [-adk] [-n window-name] [-t target-window] [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux new-window [-dk] [-n window-name] [-t target-window] [shell-command]
    // (alias: neww)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux new-window [-dk] [-n window-name] [-t target-window] [command]
    // (alias: neww)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux new-window [-d] [-n window-name] [-t target-window] [command]
    // (alias: neww)
    // ```
    let target_window = TargetWindow::Raw("5").to_string();

    let mut new_window = NewWindow::new();
    #[cfg(feature = "tmux_1_3")]
    new_window.add();
    #[cfg(feature = "tmux_0_8")]
    new_window.detached();
    #[cfg(feature = "tmux_1_0")]
    new_window.kill();
    #[cfg(feature = "tmux_1_5")]
    new_window.print();
    #[cfg(feature = "tmux_1_7")]
    new_window.start_directory("1");
    #[cfg(feature = "tmux_3_0")]
    new_window.environment("2");
    #[cfg(feature = "tmux_1_7")]
    new_window.format("3");
    #[cfg(feature = "tmux_0_8")]
    new_window.window_name("4");
    #[cfg(feature = "tmux_0_8")]
    new_window.target_window(&target_window);
    #[cfg(feature = "tmux_1_2")]
    new_window.shell_command("6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "new-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "neww";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_3")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_0")]
    s.push("-k");
    #[cfg(feature = "tmux_1_5")]
    s.push("-P");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-n", "4"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("6");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(new_window.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(new_window.0.bin_args, None);
    assert_eq!(new_window.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(new_window.0.cmd_args, Some(s));
}
