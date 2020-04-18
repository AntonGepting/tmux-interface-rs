#[test]
fn choose_tree() {
    use crate::{ChooseTree, ChooseTreeBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
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
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("choose-tree");
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
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-O", "3"]);
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-t", "4"]);
        #[cfg(feature = "tmux_1_0")]
        s.push("5");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let choose_tree = ChooseTree {
        #[cfg(feature = "tmux_2_7")]
        all: Some(true),
        #[cfg(feature = "tmux_2_7")]
        without_preview: Some(true),
        #[cfg(feature = "tmux_3_1")]
        reverse_sort_order: Some(true),
        #[cfg(feature = "tmux_1_7")]
        collapsed_sessions: Some(true),
        #[cfg(feature = "tmux_1_8")]
        collapsed_windows: Some(true),
        #[cfg(feature = "tmux_2_7")]
        zoom: Some(true),
        #[cfg(feature = "tmux_2_6")]
        format: Some("1"),
        #[cfg(feature = "tmux_2_6")]
        filter: Some("2"),
        #[cfg(feature = "tmux_2_6")]
        sort_order: Some("3"),
        #[cfg(feature = "tmux_2_6")]
        target_pane: Some(&TargetPane::Raw("4")),
        #[cfg(feature = "tmux_1_0")]
        template: Some("5"),
    };
    tmux.choose_tree(Some(&choose_tree)).unwrap_err();

    let mut builder = ChooseTreeBuilder::new();
    #[cfg(feature = "tmux_2_7")]
    builder.all();
    #[cfg(feature = "tmux_2_7")]
    builder.without_preview();
    #[cfg(feature = "tmux_3_1")]
    builder.reverse_sort_order();
    #[cfg(feature = "tmux_1_7")]
    builder.collapsed_sessions();
    #[cfg(feature = "tmux_1_8")]
    builder.collapsed_windows();
    #[cfg(feature = "tmux_2_7")]
    builder.zoom();
    #[cfg(feature = "tmux_2_6")]
    builder.format("1");
    #[cfg(feature = "tmux_2_6")]
    builder.filter("2");
    #[cfg(feature = "tmux_2_6")]
    builder.sort_order("3");
    #[cfg(feature = "tmux_2_6")]
    builder.target_pane(&TargetPane::Raw("4"));
    #[cfg(feature = "tmux_1_0")]
    builder.template("5");
    let choose_tree = builder.build();
    tmux.choose_tree(Some(&choose_tree)).unwrap_err();
}
