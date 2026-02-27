// auto-generated file
//

/// Display the contents of the specified buffer.
///
/// # Manual
///
/// tmux >=2.0:
/// ```text
/// show-buffer [-b buffer-name]
/// (alias: showb)
/// ```
///
/// tmux >=1.5:
/// ```text
/// show-buffer [-b buffer-index]
/// (alias: showb)
/// ```
///
/// tmux >=0.8:
/// ```text
/// show-buffer [-b buffer-index] [-t target-session]
/// (alias: showb)
/// ```
#[macro_export]
macro_rules! show_buffer {
    // `[-b buffer]`
    (@cmd ($cmd:expr) -b $buffer:expr, $($tail:tt)*) => {{
        $crate::show_buffer!(@cmd ({
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

    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::show_buffer!(@cmd ({
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
        $crate::ShowBuffer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::show_buffer!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::show_buffer!(@cmd ({ $crate::ShowBuffer::new() }) $($tail)*,)
    }};
}

#[test]
fn show_buffer_macro() {
    use std::borrow::Cow;

    // Display the contents of the specified buffer.
    //
    // # Manual
    //
    // tmux >=2.0:
    // ```text
    // show-buffer [-b buffer-name]
    // (alias: showb)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // show-buffer [-b buffer-index]
    // (alias: showb)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // show-buffer [-b buffer-index] [-t target-session]
    // (alias: showb)
    // ```

    let show_buffer = show_buffer!();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let show_buffer = show_buffer!((show_buffer), -b "1");
    #[cfg(feature = "tmux_2_0")]
    let show_buffer = show_buffer!((show_buffer), -b "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let show_buffer = show_buffer!((show_buffer), -t "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let show_buffer = show_buffer.build().to_vec();

    assert_eq!(show_buffer, s);
}
