#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn display_message() {
    use crate::{DisplayMessage, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
        //  (alias: display)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["display-message", "-a", "-I", "-p", "-v", "-c", "1", "-t", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let display_message = DisplayMessage {
        list_format_vars: Some(true),
        forward_stdin: Some(true),
        print: Some(true),
        verbose: Some(true),
        target_client: Some("1"),
        target_pane: Some("2"),
        message: Some("3"),
    };
    tmux.display_message(Some(&display_message)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn display_message() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux display-message [-p] [-c target-client] [-t target-pane] [message]
        //  (alias: display)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["display-message", "-p", "-c", "1", "-t", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.display_message(Some(true), Some("1"), Some("2"), Some("3"))
        .unwrap_err();
}
