#[test]
fn choose_tree() {
    use crate::{ChooseTree, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-tree", "-G", "-N", "-r", "-s", "-w", "-Z", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::new("hook"))
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
        target_pane: Some("4"),
        template: Some("5"),
    };
    tmux.choose_tree(Some(&choose_tree)).unwrap_err();
}
