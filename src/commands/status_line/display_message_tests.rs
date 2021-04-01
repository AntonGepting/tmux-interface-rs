#[test]
fn display_message() {
    use crate::{DisplayMessage, TargetPane};
    use std::borrow::Cow;

    // Structure for displaying a message
    //
    // # Manual
    //
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
    let target_pane = TargetPane::Raw("2").to_string();
    let mut display_message = DisplayMessage::new();
    #[cfg(feature = "tmux_2_9a")]
    display_message.list_format_vars();
    #[cfg(feature = "tmux_3_0")]
    display_message.forward_stdin();
    #[cfg(feature = "tmux_2_9a")]
    display_message.print();
    #[cfg(feature = "tmux_2_9a")]
    display_message.verbose();
    #[cfg(feature = "tmux_1_0")]
    display_message.target_client("1");
    #[cfg(feature = "tmux_1_5")]
    display_message.target_pane(&target_pane);
    display_message.message("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-message";
    #[cfg(feature = "cmd_alias")]
    let cmd = "display";

    let mut s = Vec::new();
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
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(display_message.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(display_message.0.bin_args, None);
    assert_eq!(display_message.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(display_message.0.cmd_args, Some(s));
}
