// auto-generated file
//

/// Save the contents of the specified paste buffer to path.
///
/// # Manual
///
/// tmux >=2.0:
/// ```text
/// save-buffer [-a] [-b buffer-name] path
/// (alias: saveb)
/// ```
///
/// tmux >=1.5:
/// ```text
/// save-buffer [-a] [-b buffer-index] path
/// (alias: saveb)
/// ```
///
/// tmux >=0.8:
/// ```text
/// save-buffer [-a] [-b buffer-index] [-t target-session] path
/// (alias: saveb)
/// ```
#[macro_export]
macro_rules! save_buffer {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::save_buffer!(@cmd ({
            $cmd.append()
        }) $($tail)*)
    }};

    // `[-b buffer-index]`
    (@cmd ($cmd:expr) -b $buffer_index:expr, $($tail:tt)*) => {{
        $crate::save_buffer!(@cmd ({
            $cmd.buffer_index($buffer_index)
        }) $($tail)*)
    }};

    // `[-b buffer-name]`
    (@cmd ($cmd:expr) -b $buffer_name:expr, $($tail:tt)*) => {{
        $crate::save_buffer!(@cmd ({
            $cmd.buffer_name($buffer_name)
        }) $($tail)*)
    }};

    // `[-t target-session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::save_buffer!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};

    // `[path]`
    (@cmd ($cmd:expr) $path:expr, $($tail:tt)*) => {{
        $crate::save_buffer!(@cmd ({
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
        $crate::SaveBuffer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::save_buffer!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::save_buffer!(@cmd ({ $crate::SaveBuffer::new() }) $($tail)*,)
    }};
}

#[test]
fn save_buffer_macro() {
    use std::borrow::Cow;

    // Save the contents of the specified paste buffer to path.
    //
    // # Manual
    //
    // tmux >=2.0:
    // ```text
    // save-buffer [-a] [-b buffer-name] path
    // (alias: saveb)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // save-buffer [-a] [-b buffer-index] path
    // (alias: saveb)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // save-buffer [-a] [-b buffer-index] [-t target-session] path
    // (alias: saveb)
    // ```

    let save_buffer = save_buffer!();
    #[cfg(feature = "tmux_0_8")]
    let save_buffer = save_buffer!((save_buffer), -a);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let save_buffer = save_buffer!((save_buffer), -b "1");
    #[cfg(feature = "tmux_2_0")]
    let save_buffer = save_buffer!((save_buffer), -b "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let save_buffer = save_buffer!((save_buffer), -t "3");
    #[cfg(feature = "tmux_0_8")]
    let save_buffer = save_buffer!((save_buffer), "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "save-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "saveb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-a");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let save_buffer = save_buffer.build().to_vec();

    assert_eq!(save_buffer, s);
}
