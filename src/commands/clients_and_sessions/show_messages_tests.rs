#[test]
fn show_messages() {
    use crate::ShowMessages;
    use std::borrow::Cow;

    // Show client messages or server information
    //
    // # Manual
    //
    // tmux ^2.2:
    // ```text
    // show-messages [-JT] [-t target-client]
    // (alias: showmsgs)
    // ```
    //
    // tmux ^1.9:
    // ```text
    // show-messages [-IJT] [-t target-client]
    // (alias: showmsgs)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // show-messages [-t target-client]
    // (alias: showmsgs)
    // ```
    let show_messages = ShowMessages::new();
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    let show_messages = show_messages.server();
    #[cfg(feature = "tmux_1_9")]
    let show_messages = show_messages.jobs();
    #[cfg(feature = "tmux_1_9")]
    let show_messages = show_messages.terminals();
    #[cfg(feature = "tmux_1_2")]
    let show_messages = show_messages.target_client("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-messages";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showmsgs";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    s.push("-I");
    #[cfg(feature = "tmux_1_9")]
    s.push("-J");
    #[cfg(feature = "tmux_1_9")]
    s.push("-T");
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let show_messages = show_messages.build().to_vec();

    assert_eq!(show_messages, s);
}
