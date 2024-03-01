#[test]
fn confirm_before() {
    use crate::ConfirmBefore;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.4:
    // ```text
    // confirm-before [-by] [-c confirm-key] [-p prompt] [-t target-client] command
    // (alias: confirm)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // confirm-before [-p prompt] [-t target-client] command
    // (alias: confirm)
    // ```
    //
    // tmux ^0.9:
    // ```text
    // confirm-before [-t target-client] command
    // (alias: confirm)
    // ```
    let confirm_before = ConfirmBefore::new();
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before.background();
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before.change_default();
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before.confirm_key("1");
    #[cfg(feature = "tmux_1_5")]
    let confirm_before = confirm_before.prompt("2");
    #[cfg(feature = "tmux_0_9")]
    let confirm_before = confirm_before.target_client("3");
    #[cfg(feature = "tmux_0_9")]
    let confirm_before = confirm_before.command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "confirm-before";
    #[cfg(feature = "cmd_alias")]
    let cmd = "confirm";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_4")]
    s.push("-b");
    #[cfg(feature = "tmux_3_4")]
    s.push("-y");
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_0_9")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_9")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let confirm_before = confirm_before.build().to_vec();

    assert_eq!(confirm_before, s);
}
