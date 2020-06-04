#[test]
fn swap_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^0.8:
        // ```text
        // tmux swap-window [-d] [-s src-window] [-t dst-window]
        // (alias: swapw)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["swap-window", "-d", "-s", "1", "-t", "2"]"#
        );
        Err(Error::Hook)
    }));

    let src_window = TargetWindow::Raw("1").to_string();
    let dst_window = TargetWindow::Raw("2").to_string();

    tmux.swap_window(Some(true), Some(&src_window), Some(&dst_window))
        .unwrap_err();
}
