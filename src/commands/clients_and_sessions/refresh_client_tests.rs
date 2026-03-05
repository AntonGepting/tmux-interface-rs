// auto-generated file
//

#[cfg(feature = "tmux_3_2")]
use crate::{AllowActions, Subscribe};

// Refresh current client
//
// # Manual
//
// tmux >=3.3:
// ```text
// refresh-client [-cDLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags]
// [-l [target-pane]] [-t target-client] [adjustment] (alias: refresh)
// ```
//
// tmux >=3.2:
// ```text
// refresh-client [-cDlLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags] [-t target-client] [adjustment]
// (alias: refresh)
// ```
//
// tmux >=3.0a:
// ```text
// refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client] [adjustment]
// (alias: refresh)
// ```
//
// tmux >=2.9:
// ```text
// refresh-client [-cDlLRSU] [-C width,height] [-F flags] [-t target-client] [adjustment]
// (alias: refresh)
// ```
//
// tmux >=2.4:
// ```text
// refresh-client [-C width,height] [-S] [-t target-client]
// (alias: refresh)
// ```
//
// tmux >=1.6:
// ```text
// refresh-client [-S] [-t target-client]
// (alias: refresh)
// ```
//
// tmux >=0.8:
// ```text
// refresh-client [-t target-client]
// (alias: refresh)
// ```
#[test]
fn refresh_client() {
    #[cfg(feature = "tmux_2_9a")]
    use crate::ClientFlags;
    use crate::RefreshClient;
    #[cfg(feature = "tmux_3_2")]
    use crate::State;
    use std::borrow::Cow;

    let refresh_client = RefreshClient::new();
    // `[-c]`
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client.tracking_cursor();

    // `[-D]`
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client.down();

    // `[-l]`
    #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_3")))]
    let refresh_client = refresh_client.request_clipboard();

    // `[-L]`
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client.left();

    // `[-R]`
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client.right();

    // `[-S]`
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_4")))]
    let refresh_client = refresh_client.status_line();

    // `[-U]`
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client.up();

    // `[-A allow-actions]`
    #[cfg(feature = "tmux_3_2")]
    let refresh_client = refresh_client.allow_actions(AllowActions {
        pane: "1".into(),
        state: State::On,
    });

    // `[-B subscribe]`
    #[cfg(feature = "tmux_3_2")]
    let refresh_client = refresh_client.subscribe(Subscribe {
        name: "2".into(),
        what: Some(22),
        format: Some(23),
    });

    // `[-C size]`
    // `[-C size]`
    // `[-C size]`
    #[cfg(feature = "tmux_2_4")]
    let refresh_client = refresh_client.size((3, 4));

    // `[-F flags]`
    #[cfg(feature = "tmux_3_0a")]
    let flags = ClientFlags {
        active_pane: Some(true),
        ..Default::default()
    };
    // `[-f flags]`
    #[cfg(feature = "tmux_3_0a")]
    let refresh_client = refresh_client.flags(flags);

    #[cfg(feature = "tmux_3_3")]
    let refresh_client = refresh_client.request_clipboard(Some("7"));
    // `[-l target-pane]`
    #[cfg(feature = "tmux_3_3")]
    let refresh_client = refresh_client.target_pane("8");

    // `[-r osc10-11-responses]`
    #[cfg(feature = "tmux_3_5")]
    let refresh_client = refresh_client.osc10_11_responses("9");

    // `[-t target-client]`
    #[cfg(feature = "tmux_0_8")]
    let refresh_client = refresh_client.target_client("10");

    // `[adjustment]`
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client.adjustment(11);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "refresh-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "refresh";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    v.push("-c");
    #[cfg(feature = "tmux_2_9")]
    v.push("-D");
    #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_3")))]
    v.push("-l");
    #[cfg(feature = "tmux_2_9")]
    v.push("-L");
    #[cfg(feature = "tmux_2_9")]
    v.push("-R");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_4")))]
    v.push("-S");
    #[cfg(feature = "tmux_2_9")]
    v.push("-U");
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-A", "1:on"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-B", "%2"]);
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_0a")))]
    v.extend_from_slice(&["-C", "3,4"]);
    #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_3")))]
    v.extend_from_slice(&["-C", "3x4"]);
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-C", "3x4"]);
    #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_2")))]
    v.extend_from_slice(&["-F", "active-pane"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-f", "active-pane"]);
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-l", "8"]);
    #[cfg(feature = "tmux_3_5")]
    v.extend_from_slice(&["-r", "9"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "10"]);
    #[cfg(feature = "tmux_2_9")]
    v.push("11");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let refresh_client = refresh_client.build().to_vec();

    assert_eq!(refresh_client, v);
}
