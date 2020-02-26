#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn choose_tree() {
    use crate::{ChooseTree, ChooseTreeBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-tree", "-G", "-N", "-r", "-s", "-w", "-Z", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::Hook)
    }));

    let choose_tree = ChooseTree {
        all: Some(true),
        without_preview: Some(true),
        reverse_sort_order: Some(true),
        collapsed_sessions: Some(true),
        collapsed_windows: Some(true),
        zoom: Some(true),
        format: Some("1"),
        filter: Some("2"),
        sort_order: Some("3"),
        target_pane: Some(&TargetPane::Raw("4")),
        template: Some("5"),
    };
    tmux.choose_tree(Some(&choose_tree)).unwrap_err();

    let choose_tree = ChooseTreeBuilder::new()
        .all()
        .without_preview()
        .reverse_sort_order()
        .collapsed_sessions()
        .collapsed_windows()
        .zoom()
        .format("1")
        .filter("2")
        .sort_order("3")
        .target_pane(&TargetPane::Raw("4"))
        .template("5")
        .build();
    tmux.choose_tree(Some(&choose_tree)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn choose_tree() {
    use crate::{ChooseTree, ChooseTreeBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-tree", "-N", "-s", "-w", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::Hook)
    }));

    let choose_tree = ChooseTree {
        without_preview: Some(true),
        collapsed_sessions: Some(true),
        collapsed_windows: Some(true),
        format: Some("1"),
        filter: Some("2"),
        sort_order: Some("3"),
        target_pane: Some(&TargetPane::Raw("4")),
        template: Some("5"),
    };
    tmux.choose_tree(Some(&choose_tree)).unwrap_err();

    let choose_tree = ChooseTreeBuilder::new()
        .without_preview()
        .collapsed_sessions()
        .collapsed_windows()
        .format("1")
        .filter("2")
        .sort_order("3")
        .target_pane(&TargetPane::Raw("4"))
        .template("5")
        .build();
    tmux.choose_tree(Some(&choose_tree)).unwrap_err();
}
