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
    // tmux show-messages [-JT] [-t target-client]
    // (alias: showmsgs)
    // ```
    //
    // tmux ^1.9:
    // ```text
    // tmux show-messages [-IJT] [-t target-client]
    // (alias: showmsgs)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux show-messages [-t target-client]
    // (alias: showmsgs)
    // ```
    let mut show_messages = ShowMessages::new();
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    show_messages.server();
    #[cfg(feature = "tmux_1_9")]
    show_messages.jobs();
    #[cfg(feature = "tmux_1_9")]
    show_messages.terminals();
    #[cfg(feature = "tmux_1_2")]
    show_messages.target_client("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-messages";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showmsgs";

    let mut s = Vec::new();
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    s.push("-I");
    #[cfg(feature = "tmux_1_9")]
    s.push("-J");
    #[cfg(feature = "tmux_1_9")]
    s.push("-T");
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(show_messages.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(show_messages.0.bin_args, None);
    assert_eq!(show_messages.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(show_messages.0.cmd_args, Some(s));
}
