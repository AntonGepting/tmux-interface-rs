#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn swap_pane() {
    use crate::{Error, SwapPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
        // (alias: swapp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["swap-pane", "-d", "-D", "-U", "-Z", "-s", "1", "-t", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    let swap_pane = SwapPane {
        detached: Some(true),
        previous: Some(true),
        next: Some(true),
        keep_zoomed: Some(true),
        src_pane: Some("1"),
        dst_pane: Some("2"),
    };
    tmux.swap_pane(Some(&swap_pane)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn swap_pane() {
    use crate::{Error, SwapPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
        // (alias: swapp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["swap-pane", "-d", "-D", "-U", "-s", "1", "-t", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    let swap_pane = SwapPane {
        detached: Some(true),
        previous: Some(true),
        next: Some(true),
        src_pane: Some("1"),
        dst_pane: Some("2"),
    };
    tmux.swap_pane(Some(&swap_pane)).unwrap_err();
}
