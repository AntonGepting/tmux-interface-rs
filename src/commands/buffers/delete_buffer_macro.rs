/// Delete the buffer named buffer-name, or the most recently added automatically named buffer
/// if not specified.
///
/// # Manual
///
/// tmux ^2.0:
/// ```text
/// delete-buffer [-b buffer-name]
/// (alias: deleteb)
/// ```
///
/// tmux ^1.5:
/// ```text
/// delete-buffer [-b buffer-index]
/// (alias: deleteb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// delete-buffer [-b buffer-index] [-t target-session]
/// (alias: deleteb)
/// ```
#[macro_export]
macro_rules! delete_buffer {
    (@cmd ($cmd:expr) -b $buffer:expr, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            #[cfg(feature = "tmux_2_0")]
            {
                $cmd.buffer_name($buffer)
            }
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
            {
                $cmd.buffer_index($buffer)
            }
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
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
    use crate::TargetPane;
    use std::borrow::Cow;

    // Delete the buffer named buffer-name, or the most recently added automatically named buffer
    // if not specified.
    //
    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // delete-buffer [-b buffer-name]
    // (alias: deleteb)
    // ```
    //
    // tmux ^1.5 v2.0:
    // ```text
    // delete-buffer [-b buffer-index]
    // (alias: deleteb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // delete-buffer [-b buffer-index] [-t target-session]
    // (alias: deleteb)
    // ```
    let buffer_name = TargetPane::Raw("1").to_string();

    let delete_buffer = delete_buffer!();
    #[cfg(feature = "tmux_2_0")]
    let delete_buffer = delete_buffer!((delete_buffer), -b buffer_name);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    let delete_buffer = delete_buffer!((delete_buffer), -b buffer_name);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let delete_buffer = delete_buffer!((delete_buffer), -t buffer_name);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "delete-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "deleteb";

    let mut s = Vec::new();
    s.push(cmd);

    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let delete_buffer = delete_buffer.build().to_vec();

    assert_eq!(delete_buffer, s);
}
