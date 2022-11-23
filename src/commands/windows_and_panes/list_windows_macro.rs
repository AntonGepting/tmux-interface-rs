// XXX: better return type
/// List windows on the server
///
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// list-windows [-a] [-F format] [-t target-session]
/// (alias: lsw)
/// ```
///
/// tmux ^1.5:
/// ```text
/// list-windows [-a] [-t target-session]
/// (alias: lsw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// list-windows [-t target-session]
/// (alias: lsw)
/// ```
#[macro_export]
macro_rules! list_windows {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::list_windows!(@cmd ({
            $cmd.all()
        }) $($tail)*)
    }};
    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::list_windows!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::list_windows!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ListWindows::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::list_windows!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::list_windows!(@cmd ({ $crate::ListWindows::new() }) $($tail)*,)
    }};
}

#[test]
fn list_windows_macro() {
    use crate::TargetSession;
    use std::borrow::Cow;

    // List windows on the server
    //
    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // list-windows [-a] [-F format] [-t target-session]
    // (alias: lsw)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // list-windows [-a] [-t target-session]
    // (alias: lsw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // list-windows [-t target-session]
    // (alias: lsw)
    // ```
    let target_session = TargetSession::Raw("2").to_string();

    let list_windows = list_windows!();
    #[cfg(feature = "tmux_1_5")]
    let list_windows = list_windows!((list_windows), -a);
    #[cfg(feature = "tmux_1_6")]
    let list_windows = list_windows!((list_windows), -F "1");
    #[cfg(feature = "tmux_0_8")]
    let list_windows = list_windows!((list_windows), -t & target_session);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-windows";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsw";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-a");
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_windows = list_windows.build().to_vec();

    assert_eq!(list_windows, s);
}
