// auto-generated file
//

// Detach the current client
//
// # Manual
//
// tmux >=2.4:
// ```text
// detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
// (alias: detach)
// ```
//
// tmux >=1.7:
// ```text
// detach-client [-aP] [-s target-session] [-t target-client]
// (alias: detach)
// ```
//
// tmux >=1.5:
// ```text
// detach-client [-P] [-s target-session] [-t target-client]
// (alias: detach)
// ```
//
// tmux >=0.8:
// ```text
// detach-client [-t target-client]
// (alias: detach)
// ```
#[test]
fn detach_client() {
    use crate::DetachClient;
    use std::borrow::Cow;

    let detach_client = DetachClient::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_7")]
    let detach_client = detach_client.all();

    // `[-P]`
    #[cfg(feature = "tmux_1_5")]
    let detach_client = detach_client.parent_sighup();

    // `[-E shell-command]`
    #[cfg(feature = "tmux_2_4")]
    let detach_client = detach_client.shell_command("1");

    // `[-s target-session]`
    #[cfg(feature = "tmux_1_5")]
    let detach_client = detach_client.target_session("2");

    // `[-t target-client]`
    #[cfg(feature = "tmux_0_8")]
    let detach_client = detach_client.target_client("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "detach-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "detach";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    v.push("-a");
    #[cfg(feature = "tmux_1_5")]
    v.push("-P");
    #[cfg(feature = "tmux_2_4")]
    v.extend_from_slice(&["-E", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-s", "2"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let detach_client = detach_client.build().to_vec();

    assert_eq!(detach_client, v);
}
