#[test]
fn resize_pane() {
    use crate::{Error, ResizePane, ResizePaneBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
        // (alias: resizep)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["resize-pane", "-D", "-L", "-M", "-R", "-U", "-Z", "-t", "1", "-x", "2", "-y", "3", "4"]"#
        );
        Err(Error::Hook)
    }));

    let resize_pane = ResizePane {
        down: Some(true),
        left: Some(true),
        mouse: Some(true),
        right: Some(true),
        up: Some(true),
        zoom: Some(true),
        target_pane: Some(&TargetPane::Raw("1")),
        width: Some(2),
        height: Some(3),
        adjustment: Some("4"),
    };
    tmux.resize_pane(Some(&resize_pane)).unwrap_err();

    let resize_pane = ResizePaneBuilder::new()
        .down()
        .left()
        .mouse()
        .right()
        .up()
        .zoom()
        .target_pane(&TargetPane::Raw("1"))
        .width(2)
        .height(3)
        .adjustment("4")
        .build();
    tmux.resize_pane(Some(&resize_pane)).unwrap_err();
}
