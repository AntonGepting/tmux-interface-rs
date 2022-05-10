#[test]
fn display_message() {
    use crate::{DisplayMessage, TargetPane};
    use std::borrow::Cow;

    // Structure for displaying a message
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux display-message [-aINpv] [-c target-client] [-d delay] [-t target-pane] [message]
    //  (alias: display)
    // ```
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
    let target_pane = TargetPane::Raw("3").to_string();
    let mut display_message = DisplayMessage::new();
    #[cfg(feature = "tmux_2_9a")]
    display_message.list_format_vars();
    #[cfg(feature = "tmux_3_0")]
    display_message.forward_stdin();
    #[cfg(feature = "tmux_3_2")]
    display_message.ignore_keys();
    #[cfg(feature = "tmux_2_9a")]
    display_message.print();
    #[cfg(feature = "tmux_2_9a")]
    display_message.verbose();
    #[cfg(feature = "tmux_1_0")]
    display_message.target_client("1");
    #[cfg(feature = "tmux_3_2")]
    display_message.delay(2);
    #[cfg(feature = "tmux_1_5")]
    display_message.target_pane(&target_pane);
    display_message.message("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-message";
    #[cfg(feature = "cmd_alias")]
    let cmd = "display";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_9a")]
    s.push("-a");
    #[cfg(feature = "tmux_3_0")]
    s.push("-I");
    #[cfg(feature = "tmux_3_2")]
    s.push("-N");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-p");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-v");
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-d", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "3"]);
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let display_message = display_message.build().to_vec();

    assert_eq!(display_message, s);
}
