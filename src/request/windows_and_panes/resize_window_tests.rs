#[test]
fn resize_window() {
    use crate::{Error, ResizeWindow, ResizeWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
        // (alias: resizew)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["resize-window", "-a", "-A", "-D", "-L", "-R", "-U", "-t", "1", "-x", "2", "-y", "3", "4"]"#
        );
        Err(Error::new("hook"))
    }));

    let resize_window = ResizeWindow {
        smallest: Some(true),
        largest: Some(true),
        down: Some(true),
        left: Some(true),
        right: Some(true),
        up: Some(true),
        target_window: Some(&TargetWindow::Raw("1")),
        width: Some(2),
        height: Some(3),
        adjustment: Some("4"),
    };
    tmux.resize_window(Some(&resize_window)).unwrap_err();

    let resize_window = ResizeWindowBuilder::new()
        .smallest()
        .largest()
        .down()
        .left()
        .right()
        .up()
        .target_window(&TargetWindow::Raw("1"))
        .width(2)
        .height(3)
        .adjustment("4")
        .build();
    tmux.resize_window(Some(&resize_window)).unwrap_err();
}
