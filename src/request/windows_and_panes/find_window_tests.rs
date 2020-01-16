#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn find_window() {
    use crate::{Error, FindWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux find-window [-rCNTZ] [-t target-pane] match-string
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["find-window", "-r", "-C", "-N", "-T", "-Z", "-t", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let find_window = FindWindow {
        regex: Some(true),
        only_visible: Some(true),
        only_name: Some(true),
        only_title: Some(true),
        zoom: Some(true),
        target_pane: Some("2"),
    };
    tmux.find_window(Some(&find_window), "3").unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn find_window() {
    use crate::{Error, FindWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux find-window [-CNT] [-t target-pane] match-string
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["find-window", "-C", "-N", "-T", "-t", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let find_window = FindWindow {
        only_visible: Some(true),
        only_name: Some(true),
        only_title: Some(true),
        target_pane: Some("2"),
    };
    tmux.find_window(Some(&find_window), "3").unwrap_err();
}
