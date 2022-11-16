/// # Manual
///
/// tmux ^2.4:
/// ```text
/// detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
///
/// tmux ^2.2:
/// ```text
/// detach-client [-aP] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
///
/// tmux ^1.5:
/// ```text
/// detach-client [-P] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
///
/// tmux ^0.8:
/// ```text
/// detach-client [-t target-client]
/// (alias: detach)
/// ```
#[macro_export]
macro_rules! detach_client {
    // `[-a]` - kill all but the client client given with `-t`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::detach_client!(@cmd ({
            $cmd.all()
        }) $($tail)*)
    }};
    // `[-P]` - send SIGHUP to the parent process of the client, typically causing it to exit
    (@cmd ($cmd:expr) -P, $($tail:tt)*) => {{
        $crate::detach_client!(@cmd ({
            $cmd.parent_sighup()
        }) $($tail)*)
    }};
    // `[-E shell-command]` - run shell-command to replace the client
    (@cmd ($cmd:expr) -E $shell_command:expr, $($tail:tt)*) => {{
        $crate::detach_client!(@cmd ({
            $cmd.shell_command($shell_command)
        }) $($tail)*)
    }};
    // `[-s target-session]` - specify the session, all clients currently attached
    (@cmd ($cmd:expr) -s $target_session:expr, $($tail:tt)*) => {{
        $crate::detach_client!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    // `[-t target-client]` - specify the client
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::detach_client!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter");
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::DetachClient::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::detach_client!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::detach_client!(@cmd ({ $crate::DetachClient::new() }) $($tail)*,)
    }};
}

#[test]
fn detach_client_macro() {
    use crate::detach_client;
    use crate::TargetSession;
    use std::borrow::Cow;

    // Structure for detaching the current client
    //
    // # Manual
    //
    // tmux ^2.4:
    // ```text
    // detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
    // (alias: detach)
    // ```
    //
    // tmux ^2.2:
    // ```text
    // detach-client [-aP] [-s target-session] [-t target-client]
    // (alias: detach)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // detach-client [-P] [-s target-session] [-t target-client]
    // (alias: detach)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // detach-client [-t target-client]
    // (alias: detach)

    let target_session = TargetSession::Raw("2").to_string();

    let detach_client = detach_client!();
    #[cfg(feature = "tmux_2_2")]
    let detach_client = detach_client!((detach_client), -a);
    #[cfg(feature = "tmux_1_5")]
    let detach_client = detach_client!((detach_client), -P);
    #[cfg(feature = "tmux_2_4")]
    let detach_client = detach_client!((detach_client), -E "1");
    #[cfg(feature = "tmux_1_5")]
    let detach_client = detach_client!((detach_client), -s & target_session);
    #[cfg(feature = "tmux_0_8")]
    let detach_client = detach_client!((detach_client), -t "3");

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
