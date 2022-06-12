#[test]
fn switch_client() {
    use crate::{SwitchClient, TargetSession};
    use std::borrow::Cow;

    // Structure to switch the current session for client `target-client` to `target-session`
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
    // (alias: switchc)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
    // (alias: switchc)
    // ```
    //
    // tmux ^1.6:
    // ```text
    // tmux switch-client [-lnpr] [-c target-client] [-t target-session]
    // (alias: switchc)
    // ```
    //
    // tmux ^1.4:
    // ```text
    // tmux switch-client [-lnp] [-c target-client] [-t target-session]
    // (alias: switchc)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux switch-client [-c target-client] [-t target-session]
    // (alias: switchc)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux switch-client [-c target-client -t target-session]
    // (alias: switchc)
    // ```
    let target_session = TargetSession::Raw("2").to_string();
    let switch_client = SwitchClient::new();
    #[cfg(feature = "tmux_2_1")]
    let switch_client = switch_client.not_update_env();
    #[cfg(feature = "tmux_1_4")]
    let switch_client = switch_client.last_session();
    #[cfg(feature = "tmux_1_4")]
    let switch_client = switch_client.next_session();
    #[cfg(feature = "tmux_1_4")]
    let switch_client = switch_client.previous_session();
    #[cfg(feature = "tmux_1_6")]
    let switch_client = switch_client.read_only();
    #[cfg(feature = "tmux_3_1")]
    let switch_client = switch_client.keep_zoomed();
    #[cfg(feature = "tmux_1_0")]
    let switch_client = switch_client.target_client("1");
    #[cfg(feature = "tmux_1_0")]
    let switch_client = switch_client.target_session(&target_session);
    #[cfg(feature = "tmux_2_1")]
    let switch_client = switch_client.key_table("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "switch-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "switchc";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    s.push("-E");
    #[cfg(feature = "tmux_1_4")]
    s.push("-l");
    #[cfg(feature = "tmux_1_4")]
    s.push("-n");
    #[cfg(feature = "tmux_1_4")]
    s.push("-p");
    #[cfg(feature = "tmux_1_6")]
    s.push("-r");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-T", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let switch_client = switch_client.build().to_vec();

    assert_eq!(switch_client, s);
}
