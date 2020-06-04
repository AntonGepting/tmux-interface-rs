#[test]
fn last_pane() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux last-pane [-deZ] [-t target-window]
        // (alias: lastp)
        // ```
        //
        // tmux ^2.0:
        // ```text
        // tmux last-pane [-de] [-t target-window]
        // (alias: lastp)
        // ```
        //
        // tmux ^1.4:
        // ```text
        // tmux last-pane [-t target-window]
        // (alias: lastp)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["last-pane", "-d", "-e", "-Z", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    let target_window = TargetWindow::Raw("1").to_string();
    tmux.last_pane(Some(true), Some(true), Some(true), Some(&target_window))
        .unwrap_err();
}
