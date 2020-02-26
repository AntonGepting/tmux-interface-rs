#[test]
fn swap_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux swap-window [-d] [-s src-window] [-t dst-window]
        // (alias: swapw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["swap-window", "-d", "-s", "1", "-t", "2"]"#
        );
        Err(Error::Hook)
    }));
    tmux.swap_window(
        Some(true),
        Some(&TargetWindow::Raw("1")),
        Some(&TargetWindow::Raw("2")),
    )
    .unwrap_err();
}
