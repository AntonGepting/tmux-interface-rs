/// Structure to switch the current session for client `target-client` to `target-session`
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
/// (alias: switchc)
/// ```
///
/// tmux ^2.1:
/// ```text
/// switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
/// (alias: switchc)
/// ```
///
/// tmux ^1.6:
/// ```text
/// switch-client [-lnpr] [-c target-client] [-t target-session]
/// (alias: switchc)
/// ```
///
/// tmux ^1.4:
/// ```text
/// switch-client [-lnp] [-c target-client] [-t target-session]
/// (alias: switchc)
/// ```
///
/// tmux ^1.0:
/// ```text
/// switch-client [-c target-client] [-t target-session]
/// (alias: switchc)
/// ```
///
/// tmux ^0.8:
/// ```text
/// switch-client [-c target-client -t target-session]
/// (alias: switchc)
/// ```
#[macro_export]
macro_rules! switch_client {
    // `[-E]` - update-environment option will not be applied
    (@cmd ($cmd:expr) -E, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.not_update_env()
        }) $($tail)*)
    }};
    // `[-l]` - move to the last session
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.last_session()
        }) $($tail)*)
    }};
    // `[-n]` - move to the next session
    (@cmd ($cmd:expr) -n, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.next_session()
        }) $($tail)*)
    }};
    // `[-p]` - move to the previous session
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.previous_session()
        }) $($tail)*)
    }};
    // `[-r]` - toggle whether a client is read-only
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.read_only()
        }) $($tail)*)
    }};
    // `[-Z]` - keep the window zoomed if it was zoomed
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.keep_zoomed()
        }) $($tail)*)
    }};
    // `[-c target-client]` - specify the target-client
    (@cmd ($cmd:expr) -c $target_client:expr, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};
    // `[-t target-session]` - specify the target session
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    // `[-T key-table]` - set the client's key table
    (@cmd ($cmd:expr) -T $key_table:expr, $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({
            $cmd.key_table($key_table)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SwitchClient::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::switch_client!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::switch_client!(@cmd ({ $crate::SwitchClient::new() }) $($tail)*,)
    }};
}

#[test]
fn switch_client_macro() {
    use crate::TargetSession;
    use std::borrow::Cow;

    // Structure to switch the current session for client `target-client` to `target-session`
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
    // (alias: switchc)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
    // (alias: switchc)
    // ```
    //
    // tmux ^1.6:
    // ```text
    // switch-client [-lnpr] [-c target-client] [-t target-session]
    // (alias: switchc)
    // ```
    //
    // tmux ^1.4:
    // ```text
    // switch-client [-lnp] [-c target-client] [-t target-session]
    // (alias: switchc)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // switch-client [-c target-client] [-t target-session]
    // (alias: switchc)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // switch-client [-c target-client -t target-session]
    // (alias: switchc)
    // ```
    let target_session = TargetSession::Raw("2").to_string();
    let switch_client = switch_client!();
    #[cfg(feature = "tmux_2_1")]
    let switch_client = switch_client!((switch_client), -E);
    #[cfg(feature = "tmux_1_4")]
    let switch_client = switch_client!((switch_client), -l);
    #[cfg(feature = "tmux_1_4")]
    let switch_client = switch_client!((switch_client), -n);
    #[cfg(feature = "tmux_1_4")]
    let switch_client = switch_client!((switch_client), -p);
    #[cfg(feature = "tmux_1_6")]
    let switch_client = switch_client!((switch_client), -r);
    #[cfg(feature = "tmux_3_1")]
    let switch_client = switch_client!((switch_client), -Z);
    #[cfg(feature = "tmux_1_0")]
    let switch_client = switch_client!((switch_client), -c "1");
    #[cfg(feature = "tmux_1_0")]
    let switch_client = switch_client!((switch_client), -t & target_session);
    #[cfg(feature = "tmux_2_1")]
    let switch_client = switch_client!((switch_client), -T "3");

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
