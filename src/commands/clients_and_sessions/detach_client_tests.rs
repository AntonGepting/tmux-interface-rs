#[test]
fn detach_client() {
    use crate::{DetachClient, TargetSession};
    use std::borrow::Cow;

    // Structure for detaching the current client
    //
    // # Manual
    //
    // tmux ^2.4:
    // ```text
    // tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
    // (alias: detach)
    // ```
    //
    // tmux ^2.2:
    // ```text
    // tmux detach-client [-aP] [-s target-session] [-t target-client]
    // (alias: detach)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux detach-client [-P] [-s target-session] [-t target-client]
    // (alias: detach)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux detach-client [-t target-client]
    // (alias: detach)

    let target_session = TargetSession::Raw("2").to_string();

    let detach_client = DetachClient::new();
    #[cfg(feature = "tmux_2_2")]
    let detach_client = detach_client.all();
    #[cfg(feature = "tmux_1_5")]
    let detach_client = detach_client.parent_sighup();
    #[cfg(feature = "tmux_2_4")]
    let detach_client = detach_client.shell_command("1");
    #[cfg(feature = "tmux_1_5")]
    let detach_client = detach_client.target_session(&target_session);
    #[cfg(feature = "tmux_0_8")]
    let detach_client = detach_client.target_client("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "detach-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "detach";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_2")]
    s.push("-a");
    #[cfg(feature = "tmux_1_5")]
    s.push("-P");
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-E", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-s", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let detach_client = detach_client.build().to_vec();

    assert_eq!(detach_client, s);
}
