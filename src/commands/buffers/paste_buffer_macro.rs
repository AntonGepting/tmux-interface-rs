/// Structure for inserting the contents of a paste buffer into the specified pane
///
/// # Manual
///
/// tmux ^1.7:
/// ```text
/// paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
///
/// tmux ^1.3:
/// ```text
/// paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-window]
/// (alias: pasteb)
/// ```
///
/// tmux ^1.0:
/// ```text
/// paste-buffer [-dr] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// paste-buffer [-d] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
#[macro_export]
macro_rules! paste_buffer {
    // `[-d]` - delete the paste buffer
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.delete()
        }) $($tail)*)
    }};
    // `[-p]` - paste bracket control codes are inserted around the buffer
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.bracket_codes()
        }) $($tail)*)
    }};
    // `[-r]` - do no replacement (equivalent to a separator of LF)
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.no_replacement()
        }) $($tail)*)
    }};
    // `[-b buffer-name]` - specify the buffer mode
    (@cmd ($cmd:expr) -b $buffer_name:expr, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.buffer_name($buffer_name)
        }) $($tail)*)
    }};
    // `[-s separator]` - specify a separator
    (@cmd ($cmd:expr) -s $separator:expr, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.separator($separator)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - specify the target pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -b $buffer_index:expr, $($tail:tt)*) => {{
        $crate::paste_buffer!(@cmd ({
            $cmd.buffer_index($buffer_index)
        }) $($tail)*)
    }};
    // `[-t target-window]` - specify the target window
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
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
    use crate::TargetPane;
    use std::borrow::Cow;

    // Structure for inserting the contents of a paste buffer into the specified pane
    //
    // # Manual
    //
    // tmux ^1.7:
    // ```text
    // paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
    // (alias: pasteb)
    // ```
    //
    // tmux ^1.3:
    // ```text
    // paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-window]
    // (alias: pasteb)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // paste-buffer [-dr] [-b buffer-index] [-t target-window]
    // (alias: pasteb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // paste-buffer [-d] [-b buffer-index] [-t target-window]
    // (alias: pasteb)
    // ```
    let target_pane = TargetPane::Raw("3").to_string();

    let paste_buffer = paste_buffer!();
    #[cfg(feature = "tmux_0_8")]
    let paste_buffer = paste_buffer!((paste_buffer), -d);
    #[cfg(feature = "tmux_1_7")]
    let paste_buffer = paste_buffer!((paste_buffer), -p);
    #[cfg(feature = "tmux_1_0")]
    let paste_buffer = paste_buffer!((paste_buffer), -r);
    #[cfg(feature = "tmux_1_7")]
    let paste_buffer = paste_buffer!((paste_buffer), -b "1");
    #[cfg(feature = "tmux_1_3")]
    let paste_buffer = paste_buffer!((paste_buffer), -s "2");
    #[cfg(feature = "tmux_1_7")]
    let paste_buffer = paste_buffer!((paste_buffer), -t & target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    let paste_buffer = paste_buffer!((paste_buffer), -t & target_pane);

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
    #[cfg(feature = "tmux_1_0")]
    s.push("-r");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_1_3")]
    s.extend_from_slice(&["-s", "2"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let paste_buffer = paste_buffer.build().to_vec();

    assert_eq!(paste_buffer, s);
}
