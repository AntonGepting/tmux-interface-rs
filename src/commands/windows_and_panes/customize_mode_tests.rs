#[test]
fn customize_mode() {
    use crate::CustomizeMode;
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

    let choose_tree = ChooseTree::new();
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree.without_preview();
    #[cfg(feature = "tmux_2_7")]
    let choose_tree = choose_tree.zoom();
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.format("1");
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.filter("2");
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.target_pane(&target_pane);
    #[cfg(feature = "tmux_2_6")]
    let choose_tree = choose_tree.template("6");

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
