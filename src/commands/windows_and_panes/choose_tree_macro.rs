/// Put a pane into tree mode, where a session, window or pane may be chosen interactively
/// from a list
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// choose-tree [-GNrswZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^3.1:
/// ```text
/// choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.7:
/// ```text
/// choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.6:
/// ```text
/// choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^1.8:
/// ```text
/// choose-tree [-suw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
///
/// tmux ^1.7:
/// ```text
/// choose-tree [-sw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
#[macro_export]
macro_rules! choose_tree {
    // `[-G]` - include all sessions in any session groups in the tree rather than only the first
    (@cmd ($cmd:expr) -G, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.all()
        }) $($tail)*)
    }};
    // `[-N]` - start without the preview
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.without_preview()
        }) $($tail)*)
    }};
    // `[-r]` - reverses the sort order
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.reverse_sort_order()
        }) $($tail)*)
    }};
    // `[-s]` - start with collapsed sessions
    (@cmd ($cmd:expr) -s, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.collapsed_sessions()
        }) $($tail)*)
    }};
    // `[-w]` - start with collapsed windows
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.collapsed_windows()
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
    // `[-K key-format]` - format for each shortcut key
    (@cmd ($cmd:expr) -K $key_format:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.key_format($key_format)
        }) $($tail)*)
    }};
    // `[-O sort-order]` - specifies the initial sort field
    (@cmd ($cmd:expr) -O $sort_order:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.sort_order($sort_order)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    // `[-t target-window]` - target-window
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            #[cfg(feature = "tmux_2_6")]
            {
                $cmd.target_pane($target)
            }
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
            {
                $cmd.target_window($target)
            }
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
        $crate::ChooseTree::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({ $crate::ChooseTree::new() }) $($tail)*,)
    }};
}

#[test]
fn choose_tree_macro() {
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
    //
    // tmux ^3.1:
    // ```text
    // choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.7:
    // ```text
    // choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.6:
    // ```text
    // choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^1.8:
    // ```text
    // choose-tree [-suw] [-b session-template] [-c window-template] [-S format] [-W format]
    // [-t target-window]
    // ```
    //
    // tmux ^1.7:
    // ```text
    // choose-tree [-sw] [-b session-template] [-c window-template] [-S format] [-W format]
    // [-t target-window]
    // ```
    #[cfg(feature = "tmux_2_6")]
    let target_pane = TargetPane::Raw("5").to_string();
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let target_window = TargetWindow::Raw("5").to_string();

    let choose_tree = choose_tree!();
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree!((choose_tree), -G);
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree!((choose_tree), -N);
    #[cfg(feature = "tmux_3_1")]
    let choose_tree = choose_tree!((choose_tree), -r);
    #[cfg(feature = "tmux_1_7")]
    let choose_tree = choose_tree!((choose_tree), -s);
    #[cfg(feature = "tmux_1_8")]
    let choose_tree = choose_tree!((choose_tree), -w);
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree!((choose_tree), -Z);
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), -F "1");
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), -f "2");
    #[cfg(feature = "tmux_3_2")]
    let choose_tree = choose_tree!((choose_tree), -K "3");
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), -O "4");
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), -t & target_pane);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree!((choose_tree), -t & target_window);
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), "6");

    let cmd = "choose-tree";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_7")]
    s.push("-G");
    #[cfg(feature = "tmux_2_7")]
    s.push("-N");
    #[cfg(feature = "tmux_3_1")]
    s.push("-r");
    #[cfg(feature = "tmux_1_7")]
    s.push("-s");
    #[cfg(feature = "tmux_1_8")]
    s.push("-w");
    #[cfg(feature = "tmux_2_7")]
    s.push("-Z");
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-K", "3"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-O", "4"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_2_6")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let choose_tree = choose_tree.build().to_vec();

    assert_eq!(choose_tree, s);
}
