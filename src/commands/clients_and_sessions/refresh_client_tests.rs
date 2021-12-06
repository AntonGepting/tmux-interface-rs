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
    // tmux 3.2:
    // ```text
    // tmux refresh-client [-cDlLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags] [-t target-client] [adjustment]
    // (alias: refresh)
    // ```
    //
    // tmux 3.0:
    // ```text
    // tmux refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client] [adjustment]
    // (alias: refresh)
    // ```
    //
    // tmux 2.9a:
    // ```text
    // tmux refresh-client [-cDlLRSU] [-C width,height] [-F flags] [-t target-client] [adjustment]
    // (alias: refresh)
    // ```
    //
    // tmux 2.4:
    // ```text
    // tmux refresh-client [-C width,height] [-S] [-t target-client]
    // (alias: refresh)
    // ```
    //
    // tmux 1.6:
    // ```text
    // tmux refresh-client [-S] [-t target-client]
    // (alias: refresh)
    // ```
    //
    // tmux 0.8:
    // ```text
    // tmux refresh-client [-t target-client]
    // (alias: refresh)
    // ```
    let mut refresh_client = RefreshClient::new();
    #[cfg(feature = "tmux_2_9a")]
    refresh_client.tracking_cursor();
    #[cfg(feature = "tmux_2_9a")]
    refresh_client.down();
    #[cfg(feature = "tmux_2_9a")]
    refresh_client.request_clipboard();
    #[cfg(feature = "tmux_2_9a")]
    refresh_client.left();
    #[cfg(feature = "tmux_2_9a")]
    refresh_client.right();
    #[cfg(feature = "tmux_1_6")]
    refresh_client.status_line();
    #[cfg(feature = "tmux_2_9a")]
    refresh_client.up();
    #[cfg(feature = "tmux_3_2")]
    refresh_client.allow_actions(0, State::On);
    #[cfg(feature = "tmux_3_2")]
    refresh_client.subscribe(0, None, None);
    #[cfg(feature = "tmux_2_4")]
    refresh_client.size((1, 2));
    #[cfg(feature = "tmux_2_9a")]
    {
        let flags = ClientFlags {
            active_pane: Some(true),
            ..Default::default()
        };
        refresh_client.flags(flags);
    }
    #[cfg(feature = "tmux_0_8")]
    refresh_client.target_client("4");
    #[cfg(feature = "tmux_2_9a")]
    refresh_client.adjustment(5);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "refresh-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "refresh";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_9a")]
    s.push("-c");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-D");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-l");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-L");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-R");
    #[cfg(feature = "tmux_1_6")]
    s.push("-S");
    #[cfg(feature = "tmux_2_9a")]
    s.push("-U");
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-A", "%0:on"]);
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
    let s = s.into_iter().map(|a| a.into()).collect();

    //assert_eq!(refresh_client.0.bin, Cow::Borrowed("tmux"));
    //assert_eq!(refresh_client.0.bin_args, None);
    assert_eq!(refresh_client.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(refresh_client.0.args, Some(s));
}
