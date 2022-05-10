#[test]
fn choose_tree() {
    use crate::ChooseTree;
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
    // tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^3.1:
    // ```text
    // tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.7:
    // ```text
    // tmux choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.6:
    // ```text
    // tmux choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux choose-tree [-suw] [-b session-template] [-c window-template] [-S format] [-W format]
    // [-t target-window]
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux choose-tree [-sw] [-b session-template] [-c window-template] [-S format] [-W format]
    // [-t target-window]
    // ```
    #[cfg(feature = "tmux_2_6")]
    let target_pane = TargetPane::Raw("5").to_string();
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let target_window = TargetWindow::Raw("5").to_string();

    let mut choose_tree = ChooseTree::new();
    #[cfg(feature = "tmux_2_7")]
    choose_tree.all();
    #[cfg(feature = "tmux_2_7")]
    choose_tree.without_preview();
    #[cfg(feature = "tmux_3_1")]
    choose_tree.reverse_sort_order();
    #[cfg(feature = "tmux_1_7")]
    choose_tree.collapsed_sessions();
    #[cfg(feature = "tmux_1_8")]
    choose_tree.collapsed_windows();
    #[cfg(feature = "tmux_2_7")]
    choose_tree.zoom();
    #[cfg(feature = "tmux_2_6")]
    choose_tree.format("1");
    #[cfg(feature = "tmux_2_6")]
    choose_tree.filter("2");
    #[cfg(feature = "tmux_3_2")]
    choose_tree.key_format("3");
    #[cfg(feature = "tmux_2_6")]
    choose_tree.sort_order("4");
    #[cfg(feature = "tmux_2_6")]
    choose_tree.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    choose_tree.target_window(&target_window);
    #[cfg(feature = "tmux_2_6")]
    choose_tree.template("6");

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
