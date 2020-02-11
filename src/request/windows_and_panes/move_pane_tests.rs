#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn move_pane() {
    use crate::{Error, MovePane, MovePaneBuilder, PaneSize, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
        // (alias: movep)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["move-pane", "-b", "-d", "-h", "-v", "-l", "1", "-s", "2", "-t", "3"]"#
        );
        Err(Error::new("hook"))
    }));

    let move_pane = MovePane {
        left_above: Some(true),
        detached: Some(true),
        horizontal: Some(true),
        vertical: Some(true),
        size: Some(&PaneSize::Size(1)),
        src_pane: Some(&TargetPane::Raw("2")),
        dst_pane: Some(&TargetPane::Raw("3")),
    };
    tmux.move_pane(Some(&move_pane)).unwrap_err();

    let move_pane = MovePaneBuilder::new()
        .left_above()
        .detached()
        .horizontal()
        .vertical()
        .size(&PaneSize::Size(1))
        .src_pane(&TargetPane::Raw("2"))
        .dst_pane(&TargetPane::Raw("3"))
        .build();
    tmux.move_pane(Some(&move_pane)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn move_pane() {
    use crate::{Error, MovePane, MovePaneBuilder, PaneSize, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
        // (alias: movep)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["move-pane", "-b", "-d", "-h", "-v", "-l", "1", "-s", "2", "-t", "3"]"#
        );
        Err(Error::new("hook"))
    }));

    let move_pane = MovePane {
        left_above: Some(true),
        detached: Some(true),
        horizontal: Some(true),
        vertical: Some(true),
        size: Some(&PaneSize::Size(1)),
        src_pane: Some(&TargetPane::Raw("2")),
        dst_pane: Some(&TargetPane::Raw("3")),
    };
    tmux.move_pane(Some(&move_pane)).unwrap_err();

    let move_pane = MovePaneBuilder::new()
        .left_above()
        .detached()
        .horizontal()
        .vertical()
        .size(&PaneSize::Size(1))
        .src_pane(&TargetPane::Raw("2"))
        .dst_pane(&TargetPane::Raw("3"))
        .build();
    tmux.move_pane(Some(&move_pane)).unwrap_err();
}
