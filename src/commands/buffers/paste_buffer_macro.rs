// auto-generated file
//

/// Structure for inserting the contents of a paste buffer into the specified pane
///
/// # Manual
///
/// tmux >=1.9:
/// ```text
/// paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
///
/// tmux >=1.5:
/// ```text
/// paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
///
/// tmux >=0.8:
/// ```text
/// paste-buffer [-d] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
#[macro_export]
macro_rules! paste_buffer {
    // `[-d]`
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.delete()
        }) $($tail)*)
    }};

    // `[-p]`
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.bracket_codes()
        }) $($tail)*)
    }};

    // `[-r]`
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.no_replacement()
        }) $($tail)*)
    }};

    // `[-b buffer-index]`
    (@cmd ($cmd:expr) -b $buffer_index:expr, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.buffer_index($buffer_index)
        }) $($tail)*)
    }};

    // `[-b buffer-name]`
    (@cmd ($cmd:expr) -b $buffer_name:expr, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.buffer_name($buffer_name)
        }) $($tail)*)
    }};

    // `[-s separator]`
    (@cmd ($cmd:expr) -s $separator:expr, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.separator($separator)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};

    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::PasteBuffer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({ $crate::PasteBuffer::new() }) $($tail)*,)
    }};
}

#[test]
fn paste_buffer_macro() {
    use std::borrow::Cow;

    // Structure for inserting the contents of a paste buffer into the specified pane
    //
    // # Manual
    //
    // tmux >=1.9:
    // ```text
    // paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
    // (alias: pasteb)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-pane]
    // (alias: pasteb)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // paste-buffer [-d] [-b buffer-index] [-t target-window]
    // (alias: pasteb)
    // ```

    let paste_buffer = paste_buffer!();
    #[cfg(feature = "tmux_0_8")]
    let paste_buffer = paste_buffer!((paste_buffer), -d);
    #[cfg(feature = "tmux_1_7")]
    let paste_buffer = paste_buffer!((paste_buffer), -p);
    #[cfg(feature = "tmux_1_5")]
    let paste_buffer = paste_buffer!((paste_buffer), -r);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let paste_buffer = paste_buffer!((paste_buffer), -b "1");
    #[cfg(feature = "tmux_2_0")]
    let paste_buffer = paste_buffer!((paste_buffer), -b "2");
    #[cfg(feature = "tmux_1_5")]
    let paste_buffer = paste_buffer!((paste_buffer), -s "3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let paste_buffer = paste_buffer!((paste_buffer), -t "4");
    #[cfg(feature = "tmux_1_5")]
    let paste_buffer = paste_buffer!((paste_buffer), -t "5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "paste-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "pasteb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_7")]
    s.push("-p");
    #[cfg(feature = "tmux_1_5")]
    s.push("-r");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-s", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "5"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let paste_buffer = paste_buffer.build().to_vec();

    assert_eq!(paste_buffer, s);
}
