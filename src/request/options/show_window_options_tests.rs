#[test]
fn show_window_options() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux show-window-options [-gv] [-t target-window] [option]
        // (alias: showw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-window-options", "-g", "-v", "-t", "1", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.show_window_options(
        Some(true),
        Some(true),
        Some(&TargetWindow::Raw("1")),
        Some("2"),
    )
    .unwrap_err();
}
