/// List all clients attached to the server
///
/// # Manual
///
/// tmux ^3.4:
/// ```text
/// list-clients [-F format] [-f filter] [-t target-session]
/// (alias: lsc)
/// ```
///
/// tmux ^1.6:
/// ```text
/// list-clients [-F format] [-t target-session]
/// (alias: lsc)
/// ```
///
/// ```
/// tmux ^1.5:
/// ```text
/// list-clients [-t target-session]
/// (alias: lsc)
/// ```
///
/// tmux ^0.8:
/// ```text
/// list-clients
/// (alias: lsc)
/// ```
#[macro_export]
macro_rules! list_clients {
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::list_clients!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -f $filter:expr, $($tail:tt)*) => {{
        $crate::list_clients!(@cmd ({
            $cmd.filter($filter)
        }) $($tail)*)
    }};
    // `[-s target-session]` - specify the session, all clients currently attached
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::list_clients!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter");
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ListClients::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::list_clients!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::list_clients!(@cmd ({ $crate::ListClients::new() }) $($tail)*,)
    }};

}

#[test]
fn list_clients_macro() {
    use crate::TargetSession;
    use std::borrow::Cow;

    // List all clients attached to the server
    //
    // # Manual
    //
    // tmux ^3.4:
    // ```text
    // list-clients [-F format] [-f filter] [-t target-session]
    // (alias: lsc)
    // ```
    //
    // tmux ^1.6:
    // ```text
    // list-clients [-F format] [-t target-session]
    // (alias: lsc)
    // ```
    //
    // ```
    // tmux ^1.5:
    // ```text
    // list-clients [-t target-session]
    // (alias: lsc)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // list-clients
    // (alias: lsc)
    // ```
    let target_session = TargetSession::Raw("3").to_string();

    let list_clients = list_clients!();
    #[cfg(feature = "tmux_1_6")]
    let list_clients = list_clients!((list_clients), -F "1");
    #[cfg(feature = "tmux_3_4")]
    let list_clients = list_clients!((list_clients), -f "2");
    #[cfg(feature = "tmux_1_5")]
    let list_clients = list_clients!((list_clients), -t & target_session);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-clients";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsc";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_clients = list_clients.build().to_vec();

    assert_eq!(list_clients, s);
}
