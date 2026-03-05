// auto-generated file
//

/// Lock `target-client`
///
/// # Manual
///
/// tmux >=1.5:
/// ```text
/// lock-client [-t target-client]
/// (alias: lockc)
/// ```
#[macro_export]
macro_rules! lock_client {
    // `[-t target-client]`
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::lock_client!(@cmd ({
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
        $crate::LockClient::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::lock_client!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::lock_client!(@cmd ({ $crate::LockClient::new() }) $($tail)*,)
    }};
}

#[test]
fn lock_client_macro() {
    use std::borrow::Cow;

    // Lock `target-client`
    //
    // # Manual
    //
    // tmux >=1.5:
    // ```text
    // lock-client [-t target-client]
    // (alias: lockc)
    // ```

    let lock_client = lock_client!();
    #[cfg(feature = "tmux_1_5")]
    let lock_client = lock_client!((lock_client), -t "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "lock-client";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lockc";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let lock_client = lock_client.build().to_vec();

    assert_eq!(lock_client, s);
}
