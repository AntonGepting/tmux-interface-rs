// TODO: size and percentage both test
#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn join_pane() {
    use crate::{Error, JoinPane, PaneSize, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
        // (alias: joinp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["join-pane", "-b", "-d", "-f", "-h", "-v", "-l", "1%", "-s", "2", "-t", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let join_pane = JoinPane {
        left_above: Some(true),
        detached: Some(true),
        full_size: Some(true),
        horizontal: Some(true),
        vertical: Some(true),
        size: Some(PaneSize::Percentage(1)),
        src_pane: Some("2"),
        dst_pane: Some("3"),
    };
    tmux.join_pane(Some(&join_pane)).unwrap_err();
}

// TODO: size and percentage both test
#[cfg(feature = "tmux_2_6")]
#[test]
fn join_pane() {
    use crate::{Error, JoinPane, PaneSize, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
        // (alias: joinp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["join-pane", "-b", "-d", "-h", "-v", "-p", "1", "-s", "2", "-t", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let join_pane = JoinPane {
        left_above: Some(true),
        detached: Some(true),
        horizontal: Some(true),
        vertical: Some(true),
        size: Some(PaneSize::Percentage(1)),
        src_pane: Some("2"),
        dst_pane: Some("3"),
    };
    tmux.join_pane(Some(&join_pane)).unwrap_err();
}
