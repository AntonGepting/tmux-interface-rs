// auto-generated file
//

// Ask for confirmation before executing command
//
// # Manual
//
// tmux >=3.4:
// ```text
// confirm-before [-by] [-c confirm-key] [-p prompt] [-t target-client] command
// (alias: confirm)
// ```
//
// tmux >=3.3:
// ```text
// confirm-before [-b] [-p prompt] [-t target-client] command
// (alias: confirm)
// ```
//
// tmux >=1.5:
// ```text
// confirm-before [-p prompt] [-t target-client] command
// (alias: confirm)
// ```
#[test]
fn confirm_before() {
    use crate::ConfirmBefore;
    use std::borrow::Cow;

    let confirm_before = ConfirmBefore::new();
    // `[-b]`
    #[cfg(feature = "tmux_3_3")]
    let confirm_before = confirm_before.background();

    // `[-y]`
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before.change_default();

    // `[-c confirm-key]`
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before.confirm_key("1");

    // `[-p prompt]`
    #[cfg(feature = "tmux_1_5")]
    let confirm_before = confirm_before.prompt("2");

    // `[-t target-client]`
    #[cfg(feature = "tmux_1_5")]
    let confirm_before = confirm_before.target_client("3");

    // `[command]`
    #[cfg(feature = "tmux_1_5")]
    let confirm_before = confirm_before.command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "confirm-before";
    #[cfg(feature = "cmd_alias")]
    let cmd = "confirm";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    v.push("-b");
    #[cfg(feature = "tmux_3_4")]
    v.push("-y");
    #[cfg(feature = "tmux_3_4")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("4");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let confirm_before = confirm_before.build().to_vec();

    assert_eq!(confirm_before, v);
}
