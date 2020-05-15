#[test]
fn display_message() {
    use crate::{DisplayMessage, DisplayMessageBuilder};
    use crate::{Error, TargetPane, TmuxInterface};
    use std::marker::PhantomData;

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
        //  (alias: display)
        // ```
        //
        // tmux ^2.9a:
        // ```text
        // tmux display-message [-apv] [-c target-client] [-t target-pane] [message]
        //  (alias: display)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux display-message [-p] [-c target-client] [-t target-pane] [message]
        //  (alias: display)
        // ```
        //
        // tmux ^1.2:
        // ```text
        // tmux display-message [-p] [-t target-client] [message]
        //  (alias: display)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux display-message [-t target-client] [message]
        //  (alias: display)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("display-message");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-a");
        #[cfg(feature = "tmux_3_0")]
        s.push("-I");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-p");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-v");
        #[cfg(feature = "tmux_1_0")]
        s.extend_from_slice(&["-c", "1"]);
        #[cfg(feature = "tmux_1_5")]
        s.extend_from_slice(&["-t", "2"]);
        s.push("3");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let display_message = DisplayMessage {
        #[cfg(feature = "tmux_2_9a")]
        list_format_vars: Some(true),
        #[cfg(feature = "tmux_3_0")]
        forward_stdin: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        print: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        verbose: Some(true),
        #[cfg(feature = "tmux_1_0")]
        target_client: Some("1"),
        #[cfg(feature = "tmux_1_5")]
        target_pane: Some(&TargetPane::Raw("2")),
        message: Some("3"),
        _phantom: PhantomData,
    };
    tmux.display_message(Some(&display_message)).unwrap_err();

    let mut builder = DisplayMessageBuilder::new();
    #[cfg(feature = "tmux_2_9a")]
    builder.list_format_vars();
    #[cfg(feature = "tmux_3_0")]
    builder.forward_stdin();
    #[cfg(feature = "tmux_2_9a")]
    builder.print();
    #[cfg(feature = "tmux_2_9a")]
    builder.verbose();
    #[cfg(feature = "tmux_1_0")]
    builder.target_client("1");
    #[cfg(feature = "tmux_1_5")]
    builder.target_pane(&TargetPane::Raw("2"));
    builder.message("3");
    let display_message = builder.build();
    tmux.display_message(Some(&display_message)).unwrap_err();
}
