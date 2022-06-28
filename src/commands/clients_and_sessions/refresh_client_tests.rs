#[test]
fn refresh_client() {
    #[cfg(feature = "tmux_2_9a")]
    use crate::ClientFlags;
    use crate::RefreshClient;
    #[cfg(feature = "tmux_3_2")]
    use crate::State;
    use std::borrow::Cow;

    // Structure for refreshing the current client
    //
    // # Manual
    //
    // tmux 3.3:
    // ```text
    // refresh-client [-cDLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags]
    // [-l [target-pane]] [-t target-client] [adjustment] (alias: refresh)
    // ```
    //
    // tmux 3.2:
    // ```text
    // refresh-client [-cDlLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags] [-t target-client] [adjustment]
    // (alias: refresh)
    // ```
    //
    // tmux 3.0:
    // ```text
    // refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client] [adjustment]
    // (alias: refresh)
    // ```
    //
    // tmux 2.9a:
    // ```text
    // refresh-client [-cDlLRSU] [-C width,height] [-F flags] [-t target-client] [adjustment]
    // (alias: refresh)
    // ```
    //
    // tmux 2.4:
    // ```text
    // refresh-client [-C width,height] [-S] [-t target-client]
    // (alias: refresh)
    // ```
    //
    // tmux 1.6:
    // ```text
    // refresh-client [-S] [-t target-client]
    // (alias: refresh)
    // ```
    //
    // tmux 0.8:
    // ```text
    // refresh-client [-t target-client]
    // (alias: refresh)
    // ```
    let refresh_client = RefreshClient::new();
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client.tracking_cursor();
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client.down();
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client.request_clipboard();
    #[cfg(all(feature = "tmux_3_3", not(feature = "tmux_3_2a")))]
    let refresh_client = refresh_client.request_clipboard("1");
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client.left();
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client.right();
    #[cfg(feature = "tmux_1_6")]
    let refresh_client = refresh_client.status_line();
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client.up();
    #[cfg(feature = "tmux_3_2")]
    let refresh_client = refresh_client.allow_actions("0", State::On);
    #[cfg(feature = "tmux_3_2")]
    let refresh_client = refresh_client.subscribe("0", None, None);
    #[cfg(feature = "tmux_2_4")]
    let refresh_client = refresh_client.size((1, 2));
    #[cfg(feature = "tmux_2_9a")]
    let flags = ClientFlags {
        active_pane: Some(true),
        ..Default::default()
    };
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client.flags(flags);
    #[cfg(feature = "tmux_0_8")]
    let refresh_client = refresh_client.target_client("4");
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client.adjustment(5);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "refresh-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "refresh";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_9a")]
    s.push("-c");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-D");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-l");
    #[cfg(all(feature = "tmux_3_3", not(feature = "tmux_3_2a")))]
    s.extend_from_slice(["-l", "1"]);
    #[cfg(feature = "tmux_2_9a")]
    s.push("-L");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-R");
    #[cfg(feature = "tmux_1_6")]
    s.push("-S");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-U");
    // TODO: %0 test
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-A", "0:on"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-B", "%0"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-C", "1x2"]);
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_0")))]
    s.extend_from_slice(&["-C", "1,2"]);
    #[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_2")))]
    s.extend_from_slice(&["-F", "active-pane"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-f", "active-pane"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_2_9a")]
    s.push("5");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let refresh_client = refresh_client.build().to_vec();

    assert_eq!(refresh_client, s);
}
