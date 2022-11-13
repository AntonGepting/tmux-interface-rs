/// Rename the session to `new-name`
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// rename-session [-t target-session] new-name
/// (alias: rename)
/// ```
#[macro_export]
macro_rules! rename_session {
    // `[-t target-session]` - specify the client
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::rename_session!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    // `[new-name]` - moves the visible part up/down left/right by adjustment rows/columns
    (@cmd ($cmd:expr) $new_name:expr, $($tail:tt)*) => {{
        $crate::rename_session!(@cmd ({
            $cmd.new_name($new_name)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::RenameSession::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::rename_session!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::rename_session!(@cmd ({ $crate::RenameSession::new() }) $($tail)*,)
    }};
}

#[test]
fn rename_session_macro() {
    use crate::{RenameSession, TargetSession};
    use std::borrow::Cow;

    // Rename the session to `new-name`
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // rename-session [-t target-session] new-name
    // (alias: rename)
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let rename_session = rename_session!();
    #[cfg(feature = "tmux_0_8")]
    let rename_session = rename_session!((rename_session), -t & target_session);
    #[cfg(feature = "tmux_0_8")]
    let rename_session = rename_session!((rename_session), "2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rename-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "rename";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let rename_session = rename_session.build().to_vec();

    assert_eq!(rename_session, s);
}
