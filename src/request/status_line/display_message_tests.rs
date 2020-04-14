#[test]
fn display_message() {
    use crate::{DisplayMessage, DisplayMessageBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
        //  (alias: display)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["display-message", "-a", "-I", "-p", "-v", "-c", "1", "-t", "2", "3"]"#
        );
        Err(Error::Hook)
    }));

    let display_message = DisplayMessage {
        list_format_vars: Some(true),
        forward_stdin: Some(true),
        print: Some(true),
        verbose: Some(true),
        target_client: Some("1"),
        target_pane: Some(&TargetPane::Raw("2")),
        message: Some("3"),
    };
    tmux.display_message(Some(&display_message)).unwrap_err();

    let display_message = DisplayMessageBuilder::new()
        .list_format_vars()
        .forward_stdin()
        .print()
        .verbose()
        .target_client("1")
        .target_pane(&TargetPane::Raw("2"))
        .message("3")
        .build();
    tmux.display_message(Some(&display_message)).unwrap_err();
}
