#[test]
fn split_window() {
    use crate::{PaneSize, SplitWindow, TargetPane};
    use std::borrow::Cow;

    // Create a new pane by splitting target-pane
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^3.0:
    // ```text
    // tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size | -p percentage]
    // [-t target-pane] [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // tmux split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // tmux split-window [-bdhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux split-window [-dhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    // [shell-command] [-F format]
    // (alias: splitw)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux split-window [-dhvP] [-l size | -p percentage] [-t target-pane] [shell-command]
    // (alias: splitw)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux split-window [-dhv] [-l size | -p percentage] [-t target-pane] [shell-command]
    // (alias: splitw)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux split-window [-dhv] [-l size | -p percentage] [-t target-window] [command]
    // (alias: splitw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux split-window [-d] [-l size | -p percentage] [-t target-window] [command]
    // (alias: splitw)
    // ```
    let target_pane = TargetPane::Raw("4").to_string();

    let mut split_window = SplitWindow::new();
    #[cfg(feature = "tmux_2_4")]
    split_window.before();
    #[cfg(feature = "tmux_0_8")]
    split_window.detached();
    #[cfg(feature = "tmux_2_4")]
    split_window.full();
    #[cfg(feature = "tmux_1_0")]
    split_window.horizontal();
    #[cfg(feature = "tmux_3_0")]
    split_window.stdin_forward();
    #[cfg(feature = "tmux_1_0")]
    split_window.vertical();
    #[cfg(feature = "tmux_1_5")]
    split_window.print();
    #[cfg(feature = "tmux_1_7")]
    split_window.start_directory("1");
    #[cfg(feature = "tmux_3_1")]
    split_window.environment("2");
    #[cfg(feature = "tmux_0_8")]
    split_window.size(&PaneSize::Size(3));
    #[cfg(feature = "tmux_1_2")]
    split_window.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_2")]
    split_window.shell_command("5");
    #[cfg(feature = "tmux_1_7")]
    split_window.format("6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "split-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "splitw";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_4")]
    s.push("-b");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_2_4")]
    s.push("-f");
    #[cfg(feature = "tmux_1_0")]
    s.push("-h");
    #[cfg(feature = "tmux_3_0")]
    s.push("-I");
    #[cfg(feature = "tmux_1_0")]
    s.push("-v");
    #[cfg(feature = "tmux_1_5")]
    s.push("-P");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-e", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-l", "3"]);
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("5");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "6"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(split_window.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(split_window.0.bin_args, None);
    assert_eq!(split_window.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(split_window.0.cmd_args, Some(s));
}
