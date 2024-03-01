/// # Manual
///
/// tmux ^3.2:
/// ```text
/// choose-tree [-GNrswZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
#[macro_export]
macro_rules! customize_mode {
    // `[-N]` - start without the preview
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.without_preview()
        }) $($tail)*)
    }};
    // `[-Z]` - zoom the pane
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};
    // `[-F format]` - format
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    // `[-f filter]` - filter
    (@cmd ($cmd:expr) -f $filter:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.filter($filter)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.target_pane($target)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) $template:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
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
        $crate::CustomizeMode::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({ $crate::CustomizeMode::new() }) $($tail)*,)
    }};
}

#[test]
fn customize_mode_macro() {
    #[cfg(feature = "tmux_2_6")]
    use crate::TargetPane;
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Put a pane into tree mode, where a session, window or pane may be chosen interactively
    // from a list
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // choose-tree [-GNrswZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
    // ```
    #[cfg(feature = "tmux_2_6")]
    let target_pane = TargetPane::Raw("5").to_string();
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let target_window = TargetWindow::Raw("5").to_string();

    let choose_tree = choose_tree!();
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree!((choose_tree), -N);
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree!((choose_tree), -Z);
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), -F "1");
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), -f "2");
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), -t & target_pane);
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), "6");

    let cmd = "choose-tree";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_7")]
    s.push("-N");
    #[cfg(feature = "tmux_2_7")]
    s.push("-Z");
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_2_6")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let choose_tree = choose_tree.build().to_vec();

    assert_eq!(choose_tree, s);
}
