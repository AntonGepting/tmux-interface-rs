#[test]
fn pipe_pane() {
    use crate::{Error, PipePane, PipePaneBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.7:
        // ```text
        // tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
        // (alias: pipep)
        // ```
        //
        // tmux ^1.2:
        // ```text
        // tmux pipe-pane [-o] [-t target-pane] [shell-command]
        // (alias: pipep)
        // ```
        //
        // tmux ^1.1:
        // ```text
        // tmux pipe-pane [-o] [-t target-pane] [command]
        // (alias: pipep)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["pipe-pane", "-I", "-O", "-o", "-t", "1", "2"]"#
        );
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("pipe-pane");
        #[cfg(feature = "tmux_2_7")]
        s.push("-I");
        #[cfg(feature = "tmux_2_7")]
        s.push("-O");
        #[cfg(feature = "tmux_1_1")]
        s.push("-o");
        #[cfg(feature = "tmux_1_1")]
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(feature = "tmux_1_2")]
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let pipe_pane = PipePane {
        #[cfg(feature = "tmux_2_7")]
        stdout: Some(true),
        #[cfg(feature = "tmux_2_7")]
        stdin: Some(true),
        #[cfg(feature = "tmux_1_1")]
        open: Some(true),
        #[cfg(feature = "tmux_1_1")]
        target_pane: Some(&TargetPane::Raw("1")),
        #[cfg(feature = "tmux_1_2")]
        shell_command: Some("2"),
    };
    tmux.pipe_pane(Some(&pipe_pane)).unwrap_err();

    let mut builder = PipePaneBuilder::new();
    #[cfg(feature = "tmux_2_7")]
    builder.stdout();
    #[cfg(feature = "tmux_2_7")]
    builder.stdin();
    #[cfg(feature = "tmux_1_1")]
    builder.open();
    #[cfg(feature = "tmux_1_1")]
    builder.target_pane(&TargetPane::Raw("1"));
    #[cfg(feature = "tmux_1_2")]
    builder.shell_command("2");
    let pipe_pane = builder.build();
    tmux.pipe_pane(Some(&pipe_pane)).unwrap_err();
}
