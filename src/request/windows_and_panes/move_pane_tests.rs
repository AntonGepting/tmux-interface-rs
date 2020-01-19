#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn move_pane() {
    use crate::{Error, MovePane, PaneSize, TmuxInterface};

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
        size: Some(PaneSize::Size(1)),
        src_pane: Some("2"),
        dst_pane: Some("3"),
    };
    tmux.move_pane(Some(&move_pane)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn move_pane() {
    use crate::{Error, MovePane, PaneSize, TmuxInterface};

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
        size: Some(PaneSize::Size(1)),
        src_pane: Some("2"),
        dst_pane: Some("3"),
    };
    tmux.move_pane(Some(&move_pane)).unwrap_err();
}
