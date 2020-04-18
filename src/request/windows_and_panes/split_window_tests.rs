#[test]
fn split_window() {
    use crate::{Error, PaneSize, SplitWindow, SplitWindowBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
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
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("split-window");
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
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let split_window = SplitWindow {
        #[cfg(feature = "tmux_2_4")]
        before: Some(true),
        #[cfg(feature = "tmux_0_8")]
        detached: Some(true),
        #[cfg(feature = "tmux_2_4")]
        full: Some(true),
        #[cfg(feature = "tmux_1_0")]
        horizontal: Some(true),
        #[cfg(feature = "tmux_3_0")]
        stdin_forward: Some(true),
        #[cfg(feature = "tmux_1_0")]
        vertical: Some(true),
        #[cfg(feature = "tmux_1_5")]
        print: Some(true),
        #[cfg(feature = "tmux_1_7")]
        cwd: Some("1"),
        #[cfg(feature = "tmux_3_1")]
        environment: Some("2"),
        #[cfg(feature = "tmux_0_8")]
        size: Some(&PaneSize::Size(3)),
        #[cfg(feature = "tmux_1_2")]
        target_pane: Some(&TargetPane::Raw("4")),
        #[cfg(feature = "tmux_1_2")]
        shell_command: Some("5"),
        #[cfg(feature = "tmux_1_7")]
        format: Some("6"),
    };
    tmux.split_window(Some(&split_window)).unwrap_err();

    let mut builder = SplitWindowBuilder::new();
    #[cfg(feature = "tmux_2_4")]
    builder.before();
    #[cfg(feature = "tmux_0_8")]
    builder.detached();
    #[cfg(feature = "tmux_2_4")]
    builder.full();
    #[cfg(feature = "tmux_1_0")]
    builder.horizontal();
    #[cfg(feature = "tmux_3_0")]
    builder.stdin_forward();
    #[cfg(feature = "tmux_1_0")]
    builder.vertical();
    #[cfg(feature = "tmux_1_5")]
    builder.print();
    #[cfg(feature = "tmux_1_7")]
    builder.cwd("1");
    #[cfg(feature = "tmux_3_1")]
    builder.environment("2");
    #[cfg(feature = "tmux_0_8")]
    builder.size(&PaneSize::Size(3));
    #[cfg(feature = "tmux_1_2")]
    builder.target_pane(&TargetPane::Raw("4"));
    #[cfg(feature = "tmux_1_2")]
    builder.shell_command("5");
    #[cfg(feature = "tmux_1_7")]
    builder.format("6");
    let split_window = builder.build();
    tmux.split_window(Some(&split_window)).unwrap_err();
}
