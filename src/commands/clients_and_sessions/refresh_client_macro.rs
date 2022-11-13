/// Structure for refreshing the current client
///
/// # Manual
///
/// tmux 3.3:
/// ```text
/// refresh-client [-cDLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags]
/// [-l [target-pane]] [-t target-client] [adjustment] (alias: refresh)
/// ```
///
/// tmux 3.2:
/// ```text
/// refresh-client [-cDlLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux 3.0:
/// ```text
/// refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux 2.9a:
/// ```text
/// refresh-client [-cDlLRSU] [-C width,height] [-F flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux 2.4:
/// ```text
/// refresh-client [-C width,height] [-S] [-t target-client]
/// (alias: refresh)
/// ```
///
/// tmux 1.6:
/// ```text
/// refresh-client [-S] [-t target-client]
/// (alias: refresh)
/// ```
///
/// tmux 0.8:
/// ```text
/// refresh-client [-t target-client]
/// (alias: refresh)
/// ```
#[macro_export]
macro_rules! refresh_client {
    // `[-c]` - return to tracking the cursor automatically
    (@cmd ($cmd:expr) -c, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.tracking_cursor()
        }) $($tail)*)
    }};
    // `[-D]` - move the visible part of a window down by `adjustment` rows
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.down()
        }) $($tail)*)
    }};
    // `[-l]` - request the clipboard from the client using the xterm(1) escape sequence
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.request_clipboard()
        }) $($tail)*)
    }};
    // `[-l [target-pane]]` - request the clipboard from the client using the xterm(1) escape sequence
    (@cmd ($cmd:expr) -l $target_pane:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.request_clipboard($target_pane)
        }) $($tail)*)
    }};
    // `[-L]` - move the visible part of a window left by `adjustment` columns
    (@cmd ($cmd:expr) -L, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.left()
        }) $($tail)*)
    }};
    // `[-R]` - move the visible part of a window right by `adjustment` columns
    (@cmd ($cmd:expr) -R, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.right()
        }) $($tail)*)
    }};
    // `[-S]` - only update the client's status line
    (@cmd ($cmd:expr) -S, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.status_line()
        }) $($tail)*)
    }};
    // `[-U]` - move the visible part of a window up by `adjustment` rows
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.up()
        }) $($tail)*)
    }};
    // TODO: accept target_pane
    // `[-A pane:state]` - allows a control mode client to trigger actions on a pane
    (@cmd ($cmd:expr) -A $pane:expr;$state:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.allow_actions($expr)
        }) $($tail)*)
    }};
    // TODO: refactor, accept target_pane, target_window and masks * ...
    // [-B name:what:format]
    (@cmd ($cmd:expr) -B $name:expr;$what:expr;$format:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.subscribe($name)
        }) $($tail)*)
    }};
    // `[-C X,Y]` - set the width and height of a control client
    // `[-C XxY]` - set the width and height of a control client
    (@cmd ($cmd:expr) -C $x:expr,$y:expr , $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.size(($x, $y))
        }) $($tail)*)
    }};
    // XXX: refactor vec?
    // `[-F flags]` - set a comma-separated list of flags
    (@cmd ($cmd:expr) -F $flags:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.flags($flags)
        }) $($tail)*)
    }};
    // XXX: refactor vec?
    // `[-f flags]` - sets a comma-separated list of client flags
    (@cmd ($cmd:expr) -f $flags:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.flags($flags)
        }) $($tail)*)
    }};
    // `[-t target-client]` - specify the client
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::refresh_client!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};
    // `[adjustment]` - moves the visible part up/down left/right by adjustment rows/columns
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
    let refresh_client = refresh_client!();
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client!(-c);
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client!(-D);
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client!(-l);
    #[cfg(all(feature = "tmux_3_3", not(feature = "tmux_3_2a")))]
    let refresh_client = refresh_client!(-l "1");
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client!(-L);
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client!(-R);
    #[cfg(feature = "tmux_1_6")]
    let refresh_client = refresh_client!(-S);
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client!(-U);
    //#[cfg(feature = "tmux_3_2")]
    //let refresh_client = refresh_client!(-A "0"; State::On);
    //#[cfg(feature = "tmux_3_2")]
    //let refresh_client = refresh_client!(-B "0"; None; None);
    //#[cfg(feature = "tmux_2_4")]
    //let refresh_client = refresh_client!((1, 2));
    #[cfg(feature = "tmux_2_9a")]
    let flags = ClientFlags {
        active_pane: Some(true),
        ..Default::default()
    };
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client!(-F flags);
    #[cfg(feature = "tmux_0_8")]
    let refresh_client = refresh_client!(-t "4");
    #[cfg(feature = "tmux_2_9a")]
    let refresh_client = refresh_client!(5);

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
