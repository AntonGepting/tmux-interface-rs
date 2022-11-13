/// Show client messages or server information
///
/// # Manual
///
/// tmux ^2.2:
/// ```text
/// show-messages [-JT] [-t target-client]
/// (alias: showmsgs)
/// ```
///
/// tmux ^1.9:
/// ```text
/// show-messages [-IJT] [-t target-client]
/// (alias: showmsgs)
/// ```
///
/// tmux ^1.2:
/// ```text
/// show-messages [-t target-client]
/// (alias: showmsgs)
/// ```
#[macro_export]
macro_rules! show_messages {
    (@cmd ($cmd:expr) -I, $($tail:tt)*) => {{
        $crate::show_messages!(@cmd ({
            $cmd.server()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -J, $($tail:tt)*) => {{
        $crate::show_messages!(@cmd ({
            $cmd.jobs()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -T, $($tail:tt)*) => {{
        $crate::show_messages!(@cmd ({
            $cmd.terminals()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::show_messages!(@cmd ({
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
        $crate::ShowMessages::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::show_messages!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::show_messages!(@cmd ({ $crate::ShowMessages::new() }) $($tail)*,)
    }};
}

#[test]
fn show_messages_macro() {
    use crate::ShowMessages;
    use std::borrow::Cow;

    // Show client messages or server information
    //
    // # Manual
    //
    // tmux ^2.2:
    // ```text
    // show-messages [-JT] [-t target-client]
    // (alias: showmsgs)
    // ```
    //
    // tmux ^1.9:
    // ```text
    // show-messages [-IJT] [-t target-client]
    // (alias: showmsgs)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // show-messages [-t target-client]
    // (alias: showmsgs)
    // ```
    let show_messages = show_messages!();
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    let show_messages = show_messages!((show_messages), -I);
    #[cfg(feature = "tmux_1_9")]
    let show_messages = show_messages!((show_messages), -J);
    #[cfg(feature = "tmux_1_9")]
    let show_messages = show_messages!((show_messages), -T);
    #[cfg(feature = "tmux_1_2")]
    let show_messages = show_messages!((show_messages), -t "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-messages";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showmsgs";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_2")))]
    s.push("-I");
    #[cfg(feature = "tmux_1_9")]
    s.push("-J");
    #[cfg(feature = "tmux_1_9")]
    s.push("-T");
    #[cfg(feature = "tmux_1_2")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let show_messages = show_messages.build().to_vec();

    assert_eq!(show_messages, s);
}
