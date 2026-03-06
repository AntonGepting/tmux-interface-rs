// auto-generated file
//

// Display a message
//
// # Manual
//
// tmux >=3.6:
// ```text
// display-message [-aCIlNpv] [-c target-client] [-d delay] [-t target-pane] [message]
//  (alias: display)
// ```
//
// tmux >=3.4:
// ```text
// display-message [-aIlNpv] [-c target-client] [-d delay] [-t target-pane] [message]
//  (alias: display)
// ```
//
// tmux >=3.2:
// ```text
// display-message [-aINpv] [-c target-client] [-d delay] [-t target-pane] [message]
//  (alias: display)
// ```
//
// tmux >=3.0:
// ```text
// display-message [-aIpv] [-c target-client] [-t target-pane] [message]
//  (alias: display)
// ```
//
// tmux >=2.9:
// ```text
// display-message [-apv] [-c target-client] [-t target-pane] [message]
//  (alias: display)
// ```
//
// tmux >=1.5:
// ```text
// display-message [-p] [-c target-client] [-t target-pane] [message]
//  (alias: display)
// ```
#[test]
fn display_message() {
    use crate::DisplayMessage;
    use std::borrow::Cow;

    let display_message = DisplayMessage::new();
    // `[-a]`
    #[cfg(feature = "tmux_2_9")]
    let display_message = display_message.list_format_vars();

    // `[-C]`
    #[cfg(feature = "tmux_3_6")]
    let display_message = display_message.keep_updated();

    // `[-I]`
    #[cfg(feature = "tmux_3_0")]
    let display_message = display_message.forward_stdin();

    // `[-l]`
    #[cfg(feature = "tmux_3_4")]
    let display_message = display_message.disable_format();

    // `[-N]`
    #[cfg(feature = "tmux_3_2")]
    let display_message = display_message.ignore_keys();

    // `[-p]`
    #[cfg(feature = "tmux_1_5")]
    let display_message = display_message.print();

    // `[-v]`
    #[cfg(feature = "tmux_2_9")]
    let display_message = display_message.verbose();

    // `[-c target-client]`
    #[cfg(feature = "tmux_1_5")]
    let display_message = display_message.target_client("1");

    // `[-d delay]`
    #[cfg(feature = "tmux_3_2")]
    let display_message = display_message.delay(2);

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let display_message = display_message.target_pane("3");

    // `[message]`
    #[cfg(feature = "tmux_1_5")]
    let display_message = display_message.message("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-message";
    #[cfg(feature = "cmd_alias")]
    let cmd = "display";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    v.push("-a");
    #[cfg(feature = "tmux_3_6")]
    v.push("-C");
    #[cfg(feature = "tmux_3_0")]
    v.push("-I");
    #[cfg(feature = "tmux_3_4")]
    v.push("-l");
    #[cfg(feature = "tmux_3_2")]
    v.push("-N");
    #[cfg(feature = "tmux_1_5")]
    v.push("-p");
    #[cfg(feature = "tmux_2_9")]
    v.push("-v");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-d", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("4");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let display_message = display_message.build().to_vec();

    assert_eq!(display_message, v);
}
