// auto-generated file
//

/// Show server information
///
/// tmux >=0.8 && <=1.9:
/// ```text
/// server-info
/// (alias: info)
/// ```
#[macro_export]
macro_rules! server_info {
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ServerInfo::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::server_info!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::server_info!(@cmd ({ $crate::ServerInfo::new() }) $($tail)*,)
    }};
}

#[test]
fn server_info_macro() {
    use std::borrow::Cow;

    // Show server information
    //
    // tmux >=0.8 && <=1.9:
    // ```text
    // server-info
    // (alias: info)
    // ```

    let server_info = server_info!();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "server-info";
    #[cfg(feature = "cmd_alias")]
    let cmd = "info";

    let mut s = Vec::new();
    s.push(cmd);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let server_info = server_info.build().to_vec();

    assert_eq!(server_info, s);
}
