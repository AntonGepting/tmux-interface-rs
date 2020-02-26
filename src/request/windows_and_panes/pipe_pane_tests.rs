#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn pipe_pane() {
    use crate::{Error, PipePane, PipePaneBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
        // (alias: pipep)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["pipe-pane", "-I", "-O", "-o", "-t", "1", "2"]"#
        );
        Err(Error::Hook)
    }));

    let pipe_pane = PipePane {
        stdout: Some(true),
        stdin: Some(true),
        open: Some(true),
        target_pane: Some(&TargetPane::Raw("1")),
        shell_command: Some("2"),
    };
    tmux.pipe_pane(Some(&pipe_pane)).unwrap_err();

    let pipe_pane = PipePaneBuilder::new()
        .stdout()
        .stdin()
        .open()
        .target_pane(&TargetPane::Raw("1"))
        .shell_command("2")
        .build();
    tmux.pipe_pane(Some(&pipe_pane)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn pipe_pane() {
    use crate::{Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux pipe-pane [-o] [-t target-pane] [shell-command]
        // (alias: pipep)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["pipe-pane", "-o", "-t", "1", "2"]"#
        );
        Err(Error::Hook)
    }));
    tmux.pipe_pane(Some(true), Some(&TargetPane::Raw("1")), Some("2"))
        .unwrap_err();
}
