#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn send_keys() {
    use crate::{Error, SendKeys, TargetPaneEx, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
        // (alias: send)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["send-keys", "-F", "-H", "-l", "-M", "-R", "-X", "-N", "1", "-t", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let target_pane = TargetPaneEx::raw("2");
    let send_keys = SendKeys {
        expand_formats: Some(true),
        hex: Some(true),
        disable_lookup: Some(true),
        mouse_event: Some(true),
        copy_mode: Some(true),
        reset: Some(true),
        repeat_count: Some(1),
        target_pane: Some(&target_pane),
    };
    tmux.send_keys(Some(&send_keys), &vec!["3"]).unwrap_err();
    //tmux.send_keys(None, &vec!["3"]);
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn send_keys() {
    use crate::{Error, SendKeys, TargetPaneEx, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
        // (alias: send)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["send-keys", "-l", "-M", "-R", "-X", "-N", "1", "-t", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let target_pane = TargetPaneEx::raw("2");
    let send_keys = SendKeys {
        disable_lookup: Some(true),
        mouse_event: Some(true),
        copy_mode: Some(true),
        reset: Some(true),
        repeat_count: Some(1),
        target_pane: Some(&target_pane),
    };
    tmux.send_keys(Some(&send_keys), &vec!["3"]).unwrap_err();
}
