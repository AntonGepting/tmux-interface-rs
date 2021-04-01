#[test]
fn confirm_before() {
    use crate::ConfirmBefore;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^1.5:
    // ```text
    // tmux confirm-before [-p prompt] [-t target-client] command
    // (alias: confirm)
    // ```
    //
    // tmux ^0.9:
    // ```text
    // tmux confirm-before [-t target-client] command
    // (alias: confirm)
    // ```
    let mut confirm_before = ConfirmBefore::new();
    #[cfg(feature = "tmux_1_5")]
    confirm_before.prompt("1");
    #[cfg(feature = "tmux_0_9")]
    confirm_before.target_client("2");
    #[cfg(feature = "tmux_0_9")]
    confirm_before.command("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "confirm-before";
    #[cfg(feature = "cmd_alias")]
    let cmd = "confirm";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-p", "1"]);
    #[cfg(feature = "tmux_0_9")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_0_9")]
    s.push("3");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(confirm_before.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(confirm_before.0.bin_args, None);
    assert_eq!(confirm_before.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(confirm_before.0.cmd_args, Some(s));
}
