#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn rotate_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux rotate-window [-DUZ] [-t target-window]
        // (alias: rotatew)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["rotate-window", "-D", "-U", "-Z", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.rotate_window(
        Some(true),
        Some(true),
        Some(true),
        Some(&TargetWindow::Raw("1")),
    )
    .unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn rotate_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux rotate-window [-DU] [-t target-window]
        // (alias: rotatew)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["rotate-window", "-D", "-U", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.rotate_window(Some(true), Some(true), Some(&TargetWindow::Raw("1")))
        .unwrap_err();
}
