#[test]
fn kill_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux kill-window [-a] [-t target-window]
        // (alias: killw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["kill-window", "-a", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.kill_window(Some(true), Some(&TargetWindow::Raw("1")))
        .unwrap_err();
}
