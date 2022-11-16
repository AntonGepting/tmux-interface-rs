/// Set the contents of the specified buffer to data.
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// set-buffer [-aw] [-b buffer-name] [-t target-client] [-n new-buffer-name] data
/// (alias: setb)
/// ```
///
/// tmux ^2.0:
/// ```text
/// set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
/// (alias: setb)
/// ```
///
/// tmux ^1.5:
/// ```text
/// set-buffer [-b buffer-index] data
/// (alias: setb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// set-buffer [-b buffer-index] [-t target-session] data
/// (alias: setb)
/// ```
#[macro_export]
macro_rules! set_buffer {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ({
            $cmd.append()
        }) $($tail)*)
    }};
    // `[-w]`
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ({
            $cmd.send_to_clipboard()
        }) $($tail)*)
    }};
    // `[-b buffer-name]`
    (@cmd ($cmd:expr) -b $buffer_name:expr, $($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ({
            $cmd.buffer_name($buffer_name)
        }) $($tail)*)
    }};
    // `[-t target-client]`
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};
    // `[-n new-buffer-name]`
    (@cmd ($cmd:expr) -n $new_buffer_name:expr, $($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ({
            $cmd.new_buffer_name($new_buffer_name)
        }) $($tail)*)
    }};
    // `[-b buffer-index]`
    (@cmd ($cmd:expr) -b $buffer_index:expr, $($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ({
            $cmd.buffer_index($buffer_index)
        }) $($tail)*)
    }};
    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    // `data`
    (@cmd ($cmd:expr) $data:expr, $($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ({
            $cmd.data($data)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SetBuffer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::set_buffer!(@cmd ({ $crate::SetBuffer::new() }) $($tail)*,)
    }};
}

#[test]
fn set_buffer_macro() {
    use std::borrow::Cow;

    // Set the contents of the specified buffer to data.
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // set-buffer [-aw] [-b buffer-name] [-t target-client] [-n new-buffer-name] data
    // (alias: setb)
    // ```
    //
    //
    // tmux ^2.0:
    // ```text
    // set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
    // (alias: setb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // set-buffer [-b buffer-index] data
    // (alias: setb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // set-buffer [-b buffer-index] [-t target-session] data
    // (alias: setb)
    // ```
    let set_buffer = set_buffer!();
    #[cfg(feature = "tmux_2_0")]
    let set_buffer = set_buffer!((set_buffer), -a);
    #[cfg(feature = "tmux_3_2")]
    let set_buffer = set_buffer!((set_buffer), -w);
    #[cfg(feature = "tmux_2_0")]
    let set_buffer = set_buffer!((set_buffer), -b "1");
    #[cfg(feature = "tmux_3_2")]
    let set_buffer = set_buffer!((set_buffer), -t "2");
    #[cfg(feature = "tmux_2_0")]
    let set_buffer = set_buffer!((set_buffer), -n "3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let set_buffer = set_buffer!((set_buffer), -b "4");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let set_buffer = set_buffer!((set_buffer), -t "5");
    #[cfg(feature = "tmux_0_8")]
    let set_buffer = set_buffer!((set_buffer), "6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_0")]
    s.push("-a");
    #[cfg(feature = "tmux_3_2")]
    s.push("-w");
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-n", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "4"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let set_buffer = set_buffer.build().to_vec();

    assert_eq!(set_buffer, s);
}
