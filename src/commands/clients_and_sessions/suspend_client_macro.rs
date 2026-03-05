// auto-generated file
//

/// Suspend a client by sending SIGTSTP (tty stop)
///
/// # Manual
///
/// tmux >=1.5:
/// ```text
/// suspend-client [-t target-client]
/// (alias: suspendc)
/// ```
///
/// tmux >=0.8:
/// ```text
/// suspend-client [-c target-client]
/// (alias: suspendc)
/// ```
#[macro_export]
macro_rules! suspend_client {
    // `[-c target-client]`
    (@cmd ($cmd:expr) -c $target_client:expr, $($tail:tt)*) => {{
        $crate::suspend_client!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    // `[-t target-client]`
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::suspend_client!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SuspendClient::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::suspend_client!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::suspend_client!(@cmd ({ $crate::SuspendClient::new() }) $($tail)*,)
    }};
}

#[test]
fn suspend_client_macro() {
    use std::borrow::Cow;

    // Suspend a client by sending SIGTSTP (tty stop)
    //
    // # Manual
    //
    // tmux >=1.5:
    // ```text
    // suspend-client [-t target-client]
    // (alias: suspendc)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // suspend-client [-c target-client]
    // (alias: suspendc)
    // ```

    let suspend_client = suspend_client!();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let suspend_client = suspend_client!((suspend_client), -c "1");
    #[cfg(feature = "tmux_1_5")]
    let suspend_client = suspend_client!((suspend_client), -t "2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "suspend-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "suspendc";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let suspend_client = suspend_client.build().to_vec();

    assert_eq!(suspend_client, s);
}
