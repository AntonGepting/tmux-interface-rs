/// Stucture for putting a pane into buffer mode
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// choose-buffer [-NZr] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^3.1:
/// ```text
/// choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.7:
/// ```text
/// choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.6:
/// ```text
/// choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^1.7:
/// ```text
/// choose-buffer [-F format] [-t target-pane] [template]
/// ```
///
/// tmux ^1.3:
/// ```text
/// choose-buffer [-t target-pane] [template]
/// ```
#[macro_export]
macro_rules! choose_buffer {
    // `[-N]` - start without the preview
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            $cmd.no_preview()
        }) $($tail)*)
    }};
    // `[-Z]` - zoom the pane
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};
    // `[-r]` - reverses the sort order
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            $cmd.reverse_sort_order()
        }) $($tail)*)
    }};
    // `[-F]` - specify the format for each item in the list
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    // `[-f filter]` - specify an initial filter
    (@cmd ($cmd:expr) -f $filter:expr, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            $cmd.filter($filter)
        }) $($tail)*)
    }};
    // `[-K key-format]` -
    (@cmd ($cmd:expr) -K $key_format:expr, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            $cmd.key_format($key_format)
        }) $($tail)*)
    }};
    // `[-O sort-order]` - specifies the initial sort field
    (@cmd ($cmd:expr) -O $sort_order:expr, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            $cmd.sort_order($sort_order)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - specify the target pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[template]` - specify the template
    (@cmd ($cmd:expr) $template:expr, $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({
            $cmd.template($template)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ChooseBuffer::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::choose_buffer!(@cmd ({ $crate::ChooseBuffer::new() }) $($tail)*,)
    }};
}

#[test]
fn choose_buffer_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Stucture for putting a pane into buffer mode
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // choose-buffer [-NZr] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^3.1:
    // ```text
    // choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.7:
    // ```text
    // choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.6:
    // ```text
    // choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^1.7:
    // ```text
    // choose-buffer [-F format] [-t target-pane] [template]
    // ```
    //
    // tmux ^1.3:
    // ```text
    // choose-buffer [-t target-pane] [template]
    // ```
    let target_pane = TargetPane::Raw("5").to_string();

    let choose_buffer = choose_buffer!();
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer!((choose_buffer), -N);
    #[cfg(feature = "tmux_2_7")]
    let choose_buffer = choose_buffer!((choose_buffer), -Z);
    #[cfg(feature = "tmux_3_1")]
    let choose_buffer = choose_buffer!((choose_buffer), -r);
    #[cfg(feature = "tmux_1_7")]
    let choose_buffer = choose_buffer!((choose_buffer), -F "1");
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer!((choose_buffer), -f "2");
    #[cfg(feature = "tmux_3_2")]
    let choose_buffer = choose_buffer!((choose_buffer), -K "3");
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer!((choose_buffer), -O "4");
    #[cfg(feature = "tmux_1_3")]
    let choose_buffer = choose_buffer!((choose_buffer), -t & target_pane);
    #[cfg(feature = "tmux_1_3")]
    let choose_buffer = choose_buffer!((choose_buffer), "6");

    let cmd = "choose-buffer";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_6")]
    s.push("-N");
    #[cfg(feature = "tmux_2_7")]
    s.push("-Z");
    #[cfg(feature = "tmux_3_1")]
    s.push("-r");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-K", "3"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-O", "4"]);
    #[cfg(feature = "tmux_1_3")]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_1_3")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let choose_buffer = choose_buffer.build().to_vec();

    assert_eq!(choose_buffer, s);
}
