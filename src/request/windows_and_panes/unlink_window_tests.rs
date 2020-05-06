#[test]
fn unlink_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.0:
        // ```text
        // tmux unlink-window [-k] [-t target-window]
        // (alias: unlinkw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux unlink-window [-t target-window]
        // (alias: unlinkw)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["unlink-window", "-k", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.unlink_window(Some(true), Some(&TargetWindow::Raw("1")))
        .unwrap_err();
}
