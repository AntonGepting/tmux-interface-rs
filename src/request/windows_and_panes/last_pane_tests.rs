#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn last_pane() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux last-pane [-deZ] [-t target-window]
        // (alias: lastp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["last-pane", "-d", "-e", "-Z", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.last_pane(
        Some(true),
        Some(true),
        Some(true),
        Some(&TargetWindow::Raw("1")),
    )
    .unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn last_pane() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux last-pane [-de] [-t target-window]
        // (alias: lastp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["last-pane", "-d", "-e", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.last_pane(Some(true), Some(true), Some(&TargetWindow::Raw("1")))
        .unwrap_err();
}
