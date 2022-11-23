/// Structure
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^3.0:
/// ```text
/// send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^2.4:
/// ```text
/// send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^2.1:
/// ```text
/// send-keys [-lMR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^1.7:
/// ```text
/// send-keys [-lR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^1.6:
/// ```text
/// send-keys [-R] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^0.8:
/// ```text
/// send-keys [-t target-window] key ...
/// (alias: send)
/// ```
#[macro_export]
macro_rules! send_keys {
    // `[-F]` - expand formats in arguments where appropriate
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.expand_formats()
        }) $($tail)*)
    }};
    // `[-H]` - expect each key to be a hexadecimal number for an ASCII character
    (@cmd ($cmd:expr) -H, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.hex()
        }) $($tail)*)
    }};
    // `[-l]` - disable key name lookup and processes the keys as literal UTF-8 characters
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.disable_lookup()
        }) $($tail)*)
    }};
    // `[-M]` - pass through a mouse event
    (@cmd ($cmd:expr) -M, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.mouse_event()
        }) $($tail)*)
    }};
    // `[-R]` - cause the terminal state to be reset
    (@cmd ($cmd:expr) -R, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.copy_mode()
        }) $($tail)*)
    }};
    // `[-X]` - send a command into copy mode
    (@cmd ($cmd:expr) -X, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.reset()
        }) $($tail)*)
    }};
    // `[-N repeat-count]` - specify a repeat count
    (@cmd ($cmd:expr) -N $repeat_count:expr, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.repeat_count($repeat_count)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - specify the target pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[-t target-window]` - specify the target window
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    // `key`
    (@cmd ($cmd:expr) $key:expr, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.key($key)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SendKeys::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({ $crate::SendKeys::new() }) $($tail)*,)
    }};
}

#[test]
fn send_keys_macro() {
    use crate::TargetPaneExt;
    use std::borrow::Cow;

    // Structure
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^3.0:
    // ```text
    // send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // send-keys [-lMR] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // send-keys [-lR] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^1.6:
    // ```text
    // send-keys [-R] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // send-keys [-t target-window] key ...
    // (alias: send)
    // ```
    let target_pane = TargetPaneExt::raw("2").to_string();

    let send_keys = send_keys!();
    #[cfg(feature = "tmux_3_1")]
    let send_keys = send_keys!((send_keys), -F);
    #[cfg(feature = "tmux_3_0")]
    let send_keys = send_keys!((send_keys), -H);
    #[cfg(feature = "tmux_1_7")]
    let send_keys = send_keys!((send_keys), -l);
    #[cfg(feature = "tmux_2_1")]
    let send_keys = send_keys!((send_keys), -M);
    #[cfg(feature = "tmux_1_6")]
    let send_keys = send_keys!((send_keys), -R);
    #[cfg(feature = "tmux_2_4")]
    let send_keys = send_keys!((send_keys), -X);
    #[cfg(feature = "tmux_2_4")]
    let send_keys = send_keys!((send_keys), -N 1);
    #[cfg(feature = "tmux_1_6")]
    let send_keys = send_keys!((send_keys), -t & target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    let send_keys = send_keys!((send_keys), -t & target_pane);
    #[cfg(feature = "tmux_0_8")]
    let send_keys = send_keys!((send_keys), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "send-keys";
    #[cfg(feature = "cmd_alias")]
    let cmd = "send";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_1")]
    s.push("-F");
    #[cfg(feature = "tmux_3_0")]
    s.push("-H");
    #[cfg(feature = "tmux_1_7")]
    s.push("-l");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_1_6")]
    s.push("-R");
    #[cfg(feature = "tmux_2_4")]
    s.push("-X");
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-N", "1"]);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    s.extend_from_slice(&["-t", "2"]);
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let send_keys = send_keys.build().to_vec();

    assert_eq!(send_keys, s);
}
