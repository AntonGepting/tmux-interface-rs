// auto-generated file
//

/// Load the contents of the specified paste buffer from path.
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// load-buffer [-w] [-b buffer-name] [-t target-client] path
/// (alias: loadb)
/// ```
///
/// tmux >=2.0:
/// ```text
/// load-buffer [-b buffer-name] path
/// (alias: loadb)
/// ```
///
/// tmux >=1.5:
/// ```text
/// load-buffer [-b buffer-index] path
/// (alias: loadb)
/// ```
///
/// tmux >=0.8:
/// ```text
/// load-buffer [-b buffer-index] [-t target-session] path
/// (alias: loadb)
/// ```
#[macro_export]
macro_rules! load_buffer {
    // `[-w]`
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::load_buffer!(@cmd ({
            $cmd.send_to_clipboard()
        }) $($tail)*)
    }};

    // `[-b buffer]`
    (@cmd ($cmd:expr) -b $buffer:expr, $($tail:tt)*) => {{
        $crate::load_buffer!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
            {
                $cmd.buffer_index($buffer)
            }
            #[cfg(feature = "tmux_2_0")]
            {
                $cmd.buffer_name($buffer)
            }
        }) $($tail)*)
    }};

    // `[-t target-client]`
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::load_buffer!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::load_buffer!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};

    // `[path]`
    (@cmd ($cmd:expr) $path:expr, $($tail:tt)*) => {{
        $crate::load_buffer!(@cmd ({
            $cmd.path($path)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::LoadBuffer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::load_buffer!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::load_buffer!(@cmd ({ $crate::LoadBuffer::new() }) $($tail)*,)
    }};
}

#[test]
fn load_buffer_macro() {
    use std::borrow::Cow;

    // Load the contents of the specified paste buffer from path.
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // load-buffer [-w] [-b buffer-name] [-t target-client] path
    // (alias: loadb)
    // ```
    //
    // tmux >=2.0:
    // ```text
    // load-buffer [-b buffer-name] path
    // (alias: loadb)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // load-buffer [-b buffer-index] path
    // (alias: loadb)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // load-buffer [-b buffer-index] [-t target-session] path
    // (alias: loadb)
    // ```

    let load_buffer = load_buffer!();
    #[cfg(feature = "tmux_3_2")]
    let load_buffer = load_buffer!((load_buffer), -w);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let load_buffer = load_buffer!((load_buffer), -b "1");
    #[cfg(feature = "tmux_2_0")]
    let load_buffer = load_buffer!((load_buffer), -b "2");
    #[cfg(feature = "tmux_3_2")]
    let load_buffer = load_buffer!((load_buffer), -t "3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let load_buffer = load_buffer!((load_buffer), -t "4");
    #[cfg(feature = "tmux_0_8")]
    let load_buffer = load_buffer!((load_buffer), "5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "load-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "loadb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-w");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "2"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("5");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let load_buffer = load_buffer.build().to_vec();

    assert_eq!(load_buffer, s);
}
