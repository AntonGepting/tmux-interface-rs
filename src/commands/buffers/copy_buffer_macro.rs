// auto-generated file
//

#[macro_export]
macro_rules! copy_buffer {
    // `[-a src-index]`
    (@cmd ($cmd:expr) -a $src_index:expr, $($tail:tt)*) => {{
        $crate::copy_buffer!(@cmd ({
            $cmd.src_index($src_index)
        }) $($tail)*)
    }};

    // `[-b dst-index]`
    (@cmd ($cmd:expr) -b $dst_index:expr, $($tail:tt)*) => {{
        $crate::copy_buffer!(@cmd ({
            $cmd.dst_index($dst_index)
        }) $($tail)*)
    }};

    // `[-s src-session]`
    (@cmd ($cmd:expr) -s $src_session:expr, $($tail:tt)*) => {{
        $crate::copy_buffer!(@cmd ({
            $cmd.src_session($src_session)
        }) $($tail)*)
    }};

    // `[-t dst-session]`
    (@cmd ($cmd:expr) -t $dst_session:expr, $($tail:tt)*) => {{
        $crate::copy_buffer!(@cmd ({
            $cmd.dst_session($dst_session)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::CopyBuffer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::copy_buffer!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::copy_buffer!(@cmd ({ $crate::CopyBuffer::new() }) $($tail)*,)
    }};
}

#[test]
fn copy_buffer_macro() {
    use std::borrow::Cow;

    let copy_buffer = copy_buffer!();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_buffer = copy_buffer!((copy_buffer), -a "1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_buffer = copy_buffer!((copy_buffer), -b "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_buffer = copy_buffer!((copy_buffer), -s "3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_buffer = copy_buffer!((copy_buffer), -t "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "copy-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "copyb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-a", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-s", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "4"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let copy_buffer = copy_buffer.build().to_vec();

    assert_eq!(copy_buffer, s);
}
