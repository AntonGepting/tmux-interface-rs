#[test]
fn kill_pane() {
    use crate::{Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.1:
        // ```text
        // tmux kill-pane [-a] [-t target-pane]
        // (alias: killp)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux kill-pane [-t target-pane]
        // (alias: killp)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux kill-pane [-p pane-index] [-t target-window]
        // (alias: killp)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["kill-pane", "-a", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    let target_pane = TargetPane::Raw("1").to_string();
    tmux.kill_pane(Some(true), Some(&target_pane)).unwrap_err();
}
