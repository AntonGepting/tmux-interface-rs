#[test]
fn select_layout() {
    use crate::{Error, SelectLayot, SelectLayotBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux select-layout [-Enop] [-t target-pane] [layout-name]
        // (alias: selectl)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["select-layout", "-E", "-n", "-o", "-p", "-t", "1", "2"]"#
        );
        Err(Error::Hook)
    }));

    let select_layout = SelectLayot {
        spread: Some(true),
        next_layout: Some(true),
        last_layout: Some(true),
        previous_layout: Some(true),
        target_pane: Some(&TargetPane::Raw("1")),
        layout_name: Some("2"),
    };
    tmux.select_layout(Some(&select_layout)).unwrap_err();

    let select_layout = SelectLayotBuilder::new()
        .spread()
        .next_layout()
        .last_layout()
        .previous_layout()
        .target_pane(&TargetPane::Raw("1"))
        .layout_name("2")
        .build();
    tmux.select_layout(Some(&select_layout)).unwrap_err();
}
