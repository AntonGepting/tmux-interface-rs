// auto-generated file
//

/// Display a message
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// display-message [-aCIlNpv] [-c target-client] [-d delay] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=3.4:
/// ```text
/// display-message [-aIlNpv] [-c target-client] [-d delay] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=3.2:
/// ```text
/// display-message [-aINpv] [-c target-client] [-d delay] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=3.0:
/// ```text
/// display-message [-aIpv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=2.9:
/// ```text
/// display-message [-apv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
///
/// tmux >=1.5:
/// ```text
/// display-message [-p] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
#[macro_export]
macro_rules! display_message {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.list_format_vars()
        }) $($tail)*)
    }};

    // `[-C]`
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.keep_updated()
        }) $($tail)*)
    }};

    // `[-I]`
    (@cmd ($cmd:expr) -I, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.forward_stdin()
        }) $($tail)*)
    }};

    // `[-l]`
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.disable_format()
        }) $($tail)*)
    }};

    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.ignore_keys()
        }) $($tail)*)
    }};

    // `[-p]`
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.print()
        }) $($tail)*)
    }};

    // `[-v]`
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.verbose()
        }) $($tail)*)
    }};

    // `[-c target-client]`
    (@cmd ($cmd:expr) -c $target_client:expr, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    // `[-d delay]`
    (@cmd ($cmd:expr) -d $delay:expr, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.delay($delay)
        }) $($tail)*)
    }};

    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};

    // `[message]`
    (@cmd ($cmd:expr) $message:expr, $($tail:tt)*) => {{
        $crate::display_message!(@cmd ({
            $cmd.message($message)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::DisplayMessage::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::display_message!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::display_message!(@cmd ({ $crate::DisplayMessage::new() }) $($tail)*,)
    }};
}

#[test]
fn display_message_macro() {
    use std::borrow::Cow;

    // Display a message
    //
    // # Manual
    //
    // tmux >=3.6:
    // ```text
    // display-message [-aCIlNpv] [-c target-client] [-d delay] [-t target-pane] [message]
    //  (alias: display)
    // ```
    //
    // tmux >=3.4:
    // ```text
    // display-message [-aIlNpv] [-c target-client] [-d delay] [-t target-pane] [message]
    //  (alias: display)
    // ```
    //
    // tmux >=3.2:
    // ```text
    // display-message [-aINpv] [-c target-client] [-d delay] [-t target-pane] [message]
    //  (alias: display)
    // ```
    //
    // tmux >=3.0:
    // ```text
    // display-message [-aIpv] [-c target-client] [-t target-pane] [message]
    //  (alias: display)
    // ```
    //
    // tmux >=2.9:
    // ```text
    // display-message [-apv] [-c target-client] [-t target-pane] [message]
    //  (alias: display)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // display-message [-p] [-c target-client] [-t target-pane] [message]
    //  (alias: display)
    // ```

    let display_message = display_message!();
    #[cfg(feature = "tmux_2_9")]
    let display_message = display_message!((display_message), -a);
    #[cfg(feature = "tmux_3_6")]
    let display_message = display_message!((display_message), -C);
    #[cfg(feature = "tmux_3_0")]
    let display_message = display_message!((display_message), -I);
    #[cfg(feature = "tmux_3_4")]
    let display_message = display_message!((display_message), -l);
    #[cfg(feature = "tmux_3_2")]
    let display_message = display_message!((display_message), -N);
    #[cfg(feature = "tmux_1_5")]
    let display_message = display_message!((display_message), -p);
    #[cfg(feature = "tmux_2_9")]
    let display_message = display_message!((display_message), -v);
    #[cfg(feature = "tmux_1_5")]
    let display_message = display_message!((display_message), -c "1");
    #[cfg(feature = "tmux_3_2")]
    let display_message = display_message!((display_message), -d 2);
    #[cfg(feature = "tmux_1_5")]
    let display_message = display_message!((display_message), -t "3");
    #[cfg(feature = "tmux_1_5")]
    let display_message = display_message!((display_message), "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-message";
    #[cfg(feature = "cmd_alias")]
    let cmd = "display";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    s.push("-a");
    #[cfg(feature = "tmux_3_6")]
    s.push("-C");
    #[cfg(feature = "tmux_3_0")]
    s.push("-I");
    #[cfg(feature = "tmux_3_4")]
    s.push("-l");
    #[cfg(feature = "tmux_3_2")]
    s.push("-N");
    #[cfg(feature = "tmux_1_5")]
    s.push("-p");
    #[cfg(feature = "tmux_2_9")]
    s.push("-v");
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-d", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_1_5")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let display_message = display_message.build().to_vec();

    assert_eq!(display_message, s);
}
