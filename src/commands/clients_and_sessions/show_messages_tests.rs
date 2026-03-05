// auto-generated file
//

// Show client messages or server information
//
// # Manual
//
// tmux >=2.2:
// ```text
// show-messages [-JT] [-t target-client]
// (alias: showmsgs)
// ```
//
// tmux >=1.9:
// ```text
// show-messages [-IJT] [-t target-client]
// (alias: showmsgs)
// ```
//
// tmux >=1.5:
// ```text
// show-messages [-t target-client]
// (alias: showmsgs)
// ```
#[test]
fn show_messages() {
    use crate::ShowMessages;
    use std::borrow::Cow;

    let show_messages = ShowMessages::new();
    // `[-I]`
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    let show_messages = show_messages.server();

    // `[-J]`
    #[cfg(feature = "tmux_1_9")]
    let show_messages = show_messages.jobs();

    // `[-T]`
    #[cfg(feature = "tmux_1_9")]
    let show_messages = show_messages.terminals();

    // `[-t target-client]`
    #[cfg(feature = "tmux_1_5")]
    let show_messages = show_messages.target_client("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-messages";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showmsgs";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    v.push("-I");
    #[cfg(feature = "tmux_1_9")]
    v.push("-J");
    #[cfg(feature = "tmux_1_9")]
    v.push("-T");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let show_messages = show_messages.build().to_vec();

    assert_eq!(show_messages, v);
}
