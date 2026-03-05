// auto-generated file
//

/// Send a key or keys to a window
///
/// # Manual
///
/// tmux >=3.4:
/// ```text
/// send-keys [-FHKlMRX] [-c target-client] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=3.1:
/// ```text
/// send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=3.0a:
/// ```text
/// send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=2.4:
/// ```text
/// send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=2.1:
/// ```text
/// send-keys [-lMR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=1.7:
/// ```text
/// send-keys [-lR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=1.6:
/// ```text
/// send-keys [-R] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux >=0.8:
/// ```text
/// send-keys [-t target-window] key ...
/// (alias: send)
/// ```
#[macro_export]
macro_rules! send_keys {
    // `[-F]`
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.expand_formats()
        }) $($tail)*)
    }};

    // `[-H]`
    (@cmd ($cmd:expr) -H, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.hex()
        }) $($tail)*)
    }};

    // `[-K]`
    (@cmd ($cmd:expr) -K, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.client()
        }) $($tail)*)
    }};

    // `[-l]`
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.disable_lookup()
        }) $($tail)*)
    }};

    // `[-M]`
    (@cmd ($cmd:expr) -M, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.mouse_event()
        }) $($tail)*)
    }};

    // `[-R]`
    (@cmd ($cmd:expr) -R, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.copy_mode()
        }) $($tail)*)
    }};

    // `[-X]`
    (@cmd ($cmd:expr) -X, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.reset()
        }) $($tail)*)
    }};

    // `[-N repeat-count]`
    (@cmd ($cmd:expr) -N $repeat_count:expr, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.repeat_count($repeat_count)
        }) $($tail)*)
    }};

    // `[-c target-client]`
    (@cmd ($cmd:expr) -c $target_client:expr, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    // `[-t target]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::send_keys!(@cmd ({
            #[cfg(feature = "tmux_2_4")]
            {
                $cmd.target_pane($target)
            }
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
            {
                $cmd.target_window($target)
            }
        }) $($tail)*)
    }};

    // `[key]`
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
    use std::borrow::Cow;

    // Send a key or keys to a window
    //
    // # Manual
    //
    // tmux >=3.4:
    // ```text
    // send-keys [-FHKlMRX] [-c target-client] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux >=3.1:
    // ```text
    // send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux >=3.0a:
    // ```text
    // send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux >=2.4:
    // ```text
    // send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux >=2.1:
    // ```text
    // send-keys [-lMR] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux >=1.7:
    // ```text
    // send-keys [-lR] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux >=1.6:
    // ```text
    // send-keys [-R] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // send-keys [-t target-window] key ...
    // (alias: send)
    // ```

    let send_keys = send_keys!();
    #[cfg(feature = "tmux_3_1")]
    let send_keys = send_keys!((send_keys), -F);
    #[cfg(feature = "tmux_3_0a")]
    let send_keys = send_keys!((send_keys), -H);
    #[cfg(feature = "tmux_3_4")]
    let send_keys = send_keys!((send_keys), -K);
    #[cfg(feature = "tmux_1_7")]
    let send_keys = send_keys!((send_keys), -l);
    #[cfg(feature = "tmux_2_1")]
    let send_keys = send_keys!((send_keys), -M);
    #[cfg(feature = "tmux_1_7")]
    let send_keys = send_keys!((send_keys), -R);
    #[cfg(feature = "tmux_2_4")]
    let send_keys = send_keys!((send_keys), -X);
    #[cfg(feature = "tmux_2_4")]
    let send_keys = send_keys!((send_keys), -N 1);
    #[cfg(feature = "tmux_3_4")]
    let send_keys = send_keys!((send_keys), -c "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    let send_keys = send_keys!((send_keys), -t "3");
    #[cfg(feature = "tmux_2_4")]
    let send_keys = send_keys!((send_keys), -t "4");
    #[cfg(feature = "tmux_0_8")]
    let send_keys = send_keys!((send_keys), "5");
    #[cfg(feature = "tmux_0_8")]
    let send_keys = send_keys!((send_keys), "6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "send-keys";
    #[cfg(feature = "cmd_alias")]
    let cmd = "send";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_1")]
    s.push("-F");
    #[cfg(feature = "tmux_3_0a")]
    s.push("-H");
    #[cfg(feature = "tmux_3_4")]
    s.push("-K");
    #[cfg(feature = "tmux_1_7")]
    s.push("-l");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_1_7")]
    s.push("-R");
    #[cfg(feature = "tmux_2_4")]
    s.push("-X");
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-N", "1"]);
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-c", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("5");
    #[cfg(feature = "tmux_0_8")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let send_keys = send_keys.build().to_vec();

    assert_eq!(send_keys, s);
}
