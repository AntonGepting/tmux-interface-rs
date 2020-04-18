#[test]
fn show_window_options() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // (removed)
        // ```
        //
        // tmux ^1.8:
        // ```text
        // tmux show-window-options [-gv] [-t target-window] [option]
        // (alias: showw)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux show-window-options [-g] [-t target-window] [option]
        // (alias: showw)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux show-window-options [-g] [-t target-window]
        // (alias: showw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux show-window-options [-t target-window] option value
        // (alias: showw)
        // ```
        // (alias: showw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-window-options", "-g", "-v", "-t", "1", "2"]"#
        );
        Err(Error::Hook)
    }));
    tmux.show_window_options(
        Some(true),
        Some(true),
        Some(&TargetWindow::Raw("1")),
        Some("2"),
    )
    .unwrap_err();
}
