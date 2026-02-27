// auto-generated file
//

/// Delete the buffer named buffer-name, or the most recently added automatically named buffer
/// if not specified.
///
/// # Manual
///
/// tmux >=2.0:
/// ```text
/// delete-buffer [-b buffer-name]
/// (alias: deleteb)
/// ```
///
/// tmux >=1.5 && <2.0:
/// ```text
/// delete-buffer [-b buffer-index]
/// (alias: deleteb)
/// ```
///
/// tmux >=0.8:
/// ```text
/// delete-buffer [-b buffer-index] [-t target-session]
/// (alias: deleteb)
/// ```
#[macro_export]
macro_rules! delete_buffer {

    // `[-b buffer-index]`
    (@cmd ($cmd:expr) -b $buffer_index:expr, $($tail:tt)*) => {{
        $crate::delete_buffer!(@cmd ({
            $cmd.buffer_index($buffer_index)
        }) $($tail)*)
    }};

    // `[-b buffer-name]`
    (@cmd ($cmd:expr) -b $buffer_name:expr, $($tail:tt)*) => {{
        $crate::delete_buffer!(@cmd ({
            $cmd.buffer_name($buffer_name)
        }) $($tail)*)
    }};

    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::delete_buffer!(@cmd ({
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
        $crate::DeleteBuffer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::delete_buffer!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::delete_buffer!(@cmd ({ $crate::DeleteBuffer::new() }) $($tail)*,)
    }};
}

#[test]
fn delete_buffer_macro() {
    use std::borrow::Cow;

    // Delete the buffer named buffer-name, or the most recently added automatically named buffer
    // if not specified.
    //
    // # Manual
    //
    // tmux >=2.0:
    // ```text
    // delete-buffer [-b buffer-name]
    // (alias: deleteb)
    // ```
    //
    // tmux >=1.5 && <2.0:
    // ```text
    // delete-buffer [-b buffer-index]
    // (alias: deleteb)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // delete-buffer [-b buffer-index] [-t target-session]
    // (alias: deleteb)
    // ```

    let delete_buffer = delete_buffer!();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let delete_buffer = delete_buffer!((delete_buffer), -b "1");
    #[cfg(feature = "tmux_2_0")]
    let delete_buffer = delete_buffer!((delete_buffer), -b "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let delete_buffer = delete_buffer!((delete_buffer), -t "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "delete-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "deleteb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let delete_buffer = delete_buffer.build().to_vec();

    assert_eq!(delete_buffer, s);
}
