// auto-generated file
//

/// Refresh current client
///
/// # Manual
///
/// tmux >=3.3:
/// ```text
/// refresh-client [-cDLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags]
/// [-l [target-pane]] [-t target-client] [adjustment] (alias: refresh)
/// ```
///
/// tmux >=3.2:
/// ```text
/// refresh-client [-cDlLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux >=3.0a:
/// ```text
/// refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux >=2.9:
/// ```text
/// refresh-client [-cDlLRSU] [-C width,height] [-F flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux >=2.4:
/// ```text
/// refresh-client [-C width,height] [-S] [-t target-client]
/// (alias: refresh)
/// ```
///
/// tmux >=1.6:
/// ```text
/// refresh-client [-S] [-t target-client]
/// (alias: refresh)
/// ```
///
/// tmux >=0.8:
/// ```text
/// refresh-client [-t target-client]
/// (alias: refresh)
/// ```
#[macro_export]
macro_rules! refresh_client {
    // `[-c]`
    (@cmd ($cmd:expr) -c, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.tracking_cursor()
        }) $($tail)*)
    }};

    // `[-D]`
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.down()
        }) $($tail)*)
    }};

    // `[-l]`
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.request_clipboard()
        }) $($tail)*)
    }};

    // `[-L]`
    (@cmd ($cmd:expr) -L, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.left()
        }) $($tail)*)
    }};

    // `[-R]`
    (@cmd ($cmd:expr) -R, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.right()
        }) $($tail)*)
    }};

    // `[-S]`
    (@cmd ($cmd:expr) -S, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.status_line()
        }) $($tail)*)
    }};

    // `[-U]`
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.up()
        }) $($tail)*)
    }};

    // `[-A allow-actions]`
    (@cmd ($cmd:expr) -A $allow_actions:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.allow_actions($allow_actions)
        }) $($tail)*)
    }};

    // `[-B subscribe]`
    (@cmd ($cmd:expr) -B $subscribe:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.subscribe($subscribe)
        }) $($tail)*)
    }};

    // `[-C size]`
    (@cmd ($cmd:expr) -C $size:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.size($size)
        }) $($tail)*)
    }};

    // `[-C size]`
    (@cmd ($cmd:expr) -C $size:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.size($size)
        }) $($tail)*)
    }};

    // `[-C size]`
    (@cmd ($cmd:expr) -C $size:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.size($size)
        }) $($tail)*)
    }};

    // `[-F flags]`
    (@cmd ($cmd:expr) -F $flags:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.flags($flags)
        }) $($tail)*)
    }};

    // `[-f flags]`
    (@cmd ($cmd:expr) -f $flags:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.flags($flags)
        }) $($tail)*)
    }};

    // `[-l target-pane]`
    (@cmd ($cmd:expr) -l $target_pane:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.request_clipboard($target_pane)
        }) $($tail)*)
    }};

    // `[-r osc10-11-responses]`
    (@cmd ($cmd:expr) -r $osc10_11_responses:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.osc10_11_responses($osc10_11_responses)
        }) $($tail)*)
    }};

    // `[-t target-client]`
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    // `[adjustment]`
    (@cmd ($cmd:expr) $adjustment:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.adjustment($adjustment)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::RefreshClient::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({ $crate::RefreshClient::new() }) $($tail)*,)
    }};
}

#[test]
fn refresh_client_macro() {
    #[cfg(feature = "tmux_2_9a")]
    use crate::ClientFlags;
    #[cfg(feature = "tmux_3_2")]
    use crate::{AllowActions, State, Subscribe};
    use std::borrow::Cow;

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

    let refresh_client = refresh_client!();
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client!((refresh_client), -c);
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client!((refresh_client), -D);
    #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_3")))]
    let refresh_client = refresh_client!((refresh_client), -l);
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client!((refresh_client), -L);
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client!((refresh_client), -R);
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_4")))]
    let refresh_client = refresh_client!((refresh_client), -S);
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client!((refresh_client), -U);
    #[cfg(feature = "tmux_3_2")]
    let refresh_client = refresh_client!((refresh_client), -A AllowActions {
        pane: "1".into(),
        state: State::On
    });
    #[cfg(feature = "tmux_3_2")]
    let refresh_client = refresh_client!((refresh_client), -B Subscribe {
        name: "2".into(),
        what: Some(22),
        format: Some(23),
    });
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_0a")))]
    let refresh_client = refresh_client!((refresh_client), -C(3, 4));
    #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_3")))]
    let refresh_client = refresh_client!((refresh_client), -C(3, 4));
    #[cfg(feature = "tmux_3_3")]
    let refresh_client = refresh_client!((refresh_client), -C(3, 4));
    #[cfg(feature = "tmux_3_0a")]
    let flags = ClientFlags {
        active_pane: Some(true),
        ..Default::default()
    };
    #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_2")))]
    let refresh_client = refresh_client!((refresh_client), -F flags);
    #[cfg(feature = "tmux_3_2")]
    let refresh_client = refresh_client!((refresh_client), -f flags);
    #[cfg(feature = "tmux_3_3")]
    let refresh_client = refresh_client!((refresh_client), -l Some("7"));
    #[cfg(feature = "tmux_3_5")]
    let refresh_client = refresh_client!((refresh_client), -r "9");
    #[cfg(feature = "tmux_0_8")]
    let refresh_client = refresh_client!((refresh_client), -t "10");
    #[cfg(feature = "tmux_2_9")]
    let refresh_client = refresh_client!((refresh_client), 11);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "refresh-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "refresh";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    s.push("-c");
    #[cfg(feature = "tmux_2_9")]
    s.push("-D");
    #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_3")))]
    s.push("-l");
    #[cfg(feature = "tmux_2_9")]
    s.push("-L");
    #[cfg(feature = "tmux_2_9")]
    s.push("-R");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_4")))]
    s.push("-S");
    #[cfg(feature = "tmux_2_9")]
    s.push("-U");
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-A", "1:on"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-B", "%2:22:23"]);
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_0a")))]
    s.extend_from_slice(&["-C", "3,4"]);
    #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_3")))]
    s.extend_from_slice(&["-C", "3x4"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-C", "3x4"]);
    #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_2")))]
    s.extend_from_slice(&["-F", "active-pane"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-f", "active-pane"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-l", "7"]);
    #[cfg(feature = "tmux_3_5")]
    s.extend_from_slice(&["-r", "9"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "10"]);
    #[cfg(feature = "tmux_2_9")]
    s.push("11");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let refresh_client = refresh_client.build().to_vec();

    assert_eq!(refresh_client, s);
}
