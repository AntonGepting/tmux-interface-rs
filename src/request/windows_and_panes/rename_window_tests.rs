#[test]
fn rename_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^0.8:
        // ```text
        // tmux rename-window [-t target-window] new-name
        // (alias: renamew)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["rename-window", "-t", "1", "2"]"#
        );
        Err(Error::Hook)
    }));
    tmux.rename_window(Some(&TargetWindow::Raw("1")), "2")
        .unwrap_err();
}
