// auto-generated file
//

// Switch the current session for client `target-client` to `target-session`
//
// # Manual
//
// tmux >=3.1:
// ```text
// switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
// (alias: switchc)
// ```
//
// tmux >=2.1:
// ```text
// switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
// (alias: switchc)
// ```
//
// tmux >=1.6:
// ```text
// switch-client [-lnpr] [-c target-client] [-t target-session]
// (alias: switchc)
// ```
//
// tmux >=1.4:
// ```text
// switch-client [-lnp] [-c target-client] [-t target-session]
// (alias: switchc)
// ```
//
// tmux >=1.0:
// ```text
// switch-client [-c target-client] [-t target-session]
// (alias: switchc)
// ```
//
// tmux >=0.8:
// ```text
// switch-client [-c target-client -t target-session]
// (alias: switchc)
// ```
#[test]
fn switch_client() {
    use crate::SwitchClient;
    use std::borrow::Cow;

    let switch_client = SwitchClient::new();
    // `[-E]`
    #[cfg(feature = "tmux_2_1")]
    let switch_client = switch_client.not_update_env();

    // `[-l]`
    #[cfg(feature = "tmux_1_5")]
    let switch_client = switch_client.last_session();

    // `[-n]`
    #[cfg(feature = "tmux_1_5")]
    let switch_client = switch_client.next_session();

    // `[-p]`
    #[cfg(feature = "tmux_1_5")]
    let switch_client = switch_client.previous_session();

    // `[-r]`
    #[cfg(feature = "tmux_1_6")]
    let switch_client = switch_client.read_only();

    // `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    let switch_client = switch_client.keep_zoomed();

    // `[-c target-client]`
    #[cfg(feature = "tmux_0_8")]
    let switch_client = switch_client.target_client("1");

    // `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    let switch_client = switch_client.target_session("2");

    // `[-T key-table]`
    #[cfg(feature = "tmux_2_1")]
    let switch_client = switch_client.key_table("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "switch-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "switchc";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_1")]
    v.push("-E");
    #[cfg(feature = "tmux_1_5")]
    v.push("-l");
    #[cfg(feature = "tmux_1_5")]
    v.push("-n");
    #[cfg(feature = "tmux_1_5")]
    v.push("-p");
    #[cfg(feature = "tmux_1_6")]
    v.push("-r");
    #[cfg(feature = "tmux_3_1")]
    v.push("-Z");
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_1")]
    v.extend_from_slice(&["-T", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let switch_client = switch_client.build().to_vec();

    assert_eq!(switch_client, v);
}
