// auto-generated file
//

/// Put a pane into tree mode, where a session, window or pane may be chosen interactively
/// from a list
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// choose-tree [-GnrswyZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=3.2:
/// ```text
/// choose-tree [-GNrswZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=3.1:
/// ```text
/// choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=2.7:
/// ```text
/// choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=2.6:
/// ```text
/// choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=1.8:
/// ```text
/// choose-tree [-suw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
///
/// tmux >=1.7:
/// ```text
/// choose-tree [-sw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
#[macro_export]
macro_rules! choose_tree {
    // `[-G]`
    (@cmd ($cmd:expr) -G, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.all()
        }) $($tail)*)
    }};

    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.without_preview()
        }) $($tail)*)
    }};

    // `[-r]`
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.reverse_sort_order()
        }) $($tail)*)
    }};

    // `[-s]`
    (@cmd ($cmd:expr) -s, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.collapsed_sessions()
        }) $($tail)*)
    }};

    // `[-u]`
    (@cmd ($cmd:expr) -u, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.expanded_sessions()
        }) $($tail)*)
    }};

    // `[-w]`
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.collapsed_windows()
        }) $($tail)*)
    }};

    // `[-y]`
    (@cmd ($cmd:expr) -y, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.disable_confirmation()
        }) $($tail)*)
    }};

    // `[-Z]`
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};

    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-f filter]`
    (@cmd ($cmd:expr) -f $filter:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.filter($filter)
        }) $($tail)*)
    }};

    // `[-K key-format]`
    (@cmd ($cmd:expr) -K $key_format:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.key_format($key_format)
        }) $($tail)*)
    }};

    // `[-O sort-order]`
    (@cmd ($cmd:expr) -O $sort_order:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.sort_order($sort_order)
        }) $($tail)*)
    }};

    // `[-b session-template]`
    (@cmd ($cmd:expr) -b $session_template:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.session_template($session_template)
        }) $($tail)*)
    }};

    // `[-c window-template]`
    (@cmd ($cmd:expr) -c $window_template:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.window_template($window_template)
        }) $($tail)*)
    }};

    // `[-S format]`
    (@cmd ($cmd:expr) -S $format:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-W format]`
    (@cmd ($cmd:expr) -W $format:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::choose_tree!(@cmd ({
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_2_6")]
            {
                $cmd.target_pane($target)
            }
        }) $($tail)*)
    }};

    // `[template]`
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
    use std::borrow::Cow;

    // Put a pane into tree mode, where a session, window or pane may be chosen interactively
    // from a list
    //
    // # Manual
    //
    // tmux >=3.6:
    // ```text
    // choose-tree [-GnrswyZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=3.2:
    // ```text
    // choose-tree [-GNrswZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=3.1:
    // ```text
    // choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=2.7:
    // ```text
    // choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=2.6:
    // ```text
    // choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=1.8:
    // ```text
    // choose-tree [-suw] [-b session-template] [-c window-template] [-S format] [-W format]
    // [-t target-window]
    // ```
    //
    // tmux >=1.7:
    // ```text
    // choose-tree [-sw] [-b session-template] [-c window-template] [-S format] [-W format]
    // [-t target-window]
    // ```

    let choose_tree = choose_tree!();
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree!((choose_tree), -G);
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), -N);
    #[cfg(feature = "tmux_3_1")]
    let choose_tree = choose_tree!((choose_tree), -r);
    #[cfg(feature = "tmux_1_7")]
    let choose_tree = choose_tree!((choose_tree), -s);
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree!((choose_tree), -u);
    #[cfg(feature = "tmux_1_7")]
    let choose_tree = choose_tree!((choose_tree), -w);
    #[cfg(feature = "tmux_3_6")]
    let choose_tree = choose_tree!((choose_tree), -y);
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
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree!((choose_tree), -b "5");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree!((choose_tree), -c "6");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree!((choose_tree), -S "7");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree!((choose_tree), -W "8");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree!((choose_tree), -t "9");
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), -t "10");
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree!((choose_tree), "11");

    let cmd = "choose-tree";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_7")]
    s.push("-G");
    #[cfg(feature = "tmux_2_6")]
    s.push("-N");
    #[cfg(feature = "tmux_3_1")]
    s.push("-r");
    #[cfg(feature = "tmux_1_7")]
    s.push("-s");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_6")))]
    s.push("-u");
    #[cfg(feature = "tmux_1_7")]
    s.push("-w");
    #[cfg(feature = "tmux_3_6")]
    s.push("-y");
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
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-b", "5"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-c", "6"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-S", "7"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-W", "8"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-t", "9"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-t", "10"]);
    #[cfg(feature = "tmux_2_6")]
    s.push("11");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let choose_tree = choose_tree.build().to_vec();

    assert_eq!(choose_tree, s);
}
