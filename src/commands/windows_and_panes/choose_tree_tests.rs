// auto-generated file
//

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
#[test]
fn choose_tree() {
    use crate::ChooseTree;
    use std::borrow::Cow;

    let choose_tree = ChooseTree::new();
    // `[-G]`
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree.all();

    // `[-N]`
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.without_preview();

    // `[-r]`
    #[cfg(feature = "tmux_3_1")]
    let choose_tree = choose_tree.reverse_sort_order();

    // `[-s]`
    #[cfg(feature = "tmux_1_7")]
    let choose_tree = choose_tree.collapsed_sessions();

    // `[-u]`
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree.expanded_sessions();

    // `[-w]`
    #[cfg(feature = "tmux_1_7")]
    let choose_tree = choose_tree.collapsed_windows();

    // `[-y]`
    #[cfg(feature = "tmux_3_6")]
    let choose_tree = choose_tree.disable_confirmation();

    // `[-Z]`
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree.zoom();

    // `[-F format]`
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.format("1");

    // `[-f filter]`
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.filter("2");

    // `[-K key-format]`
    #[cfg(feature = "tmux_3_2")]
    let choose_tree = choose_tree.key_format("3");

    // `[-O sort-order]`
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.sort_order("4");

    // `[-b session-template]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree.session_template("5");

    // `[-c window-template]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree.window_template("6");

    // `[-S format]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree.format("7");

    // `[-W format]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree.format("8");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let choose_tree = choose_tree.target_window("9");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.target_pane("10");

    // `[template]`
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.template("11");

    let cmd = "choose-tree";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_7")]
    v.push("-G");
    #[cfg(feature = "tmux_2_6")]
    v.push("-N");
    #[cfg(feature = "tmux_3_1")]
    v.push("-r");
    #[cfg(feature = "tmux_1_7")]
    v.push("-s");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_6")))]
    v.push("-u");
    #[cfg(feature = "tmux_1_7")]
    v.push("-w");
    #[cfg(feature = "tmux_3_6")]
    v.push("-y");
    #[cfg(feature = "tmux_2_7")]
    v.push("-Z");
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-K", "3"]);
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-O", "4"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    v.extend_from_slice(&["-b", "5"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    v.extend_from_slice(&["-c", "6"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    v.extend_from_slice(&["-S", "7"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    v.extend_from_slice(&["-W", "8"]);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    v.extend_from_slice(&["-t", "9"]);
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-t", "10"]);
    #[cfg(feature = "tmux_2_6")]
    v.push("11");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let choose_tree = choose_tree.build().to_vec();

    assert_eq!(choose_tree, v);
}
