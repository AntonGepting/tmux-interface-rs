#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn choose_buffer() {
    use crate::{ChooseBuffer, ChooseBufferBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-buffer", "-N", "-Z", "-r", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::Hook)
    }));

    let choose_buffer = ChooseBuffer {
        no_preview: Some(true),
        zoom: Some(true),
        reverse_sort_order: Some(true),
        format: Some("1"),
        filter: Some("2"),
        sort_order: Some("3"),
        target_pane: Some(&TargetPane::Raw("4")),
        template: Some("5"),
    };
    tmux.choose_buffer(Some(&choose_buffer)).unwrap_err();

    let choose_buffer = ChooseBufferBuilder::new()
        .no_preview()
        .zoom()
        .reverse_sort_order()
        .format("1")
        .filter("2")
        .sort_order("3")
        .target_pane(&TargetPane::Raw("4"))
        .template("5")
        .build();
    tmux.choose_buffer(Some(&choose_buffer)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn choose_buffer() {
    use crate::{ChooseBuffer, ChooseBufferBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-buffer", "-N", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::Hook)
    }));

    let choose_buffer = ChooseBuffer {
        no_preview: Some(true),
        format: Some("1"),
        filter: Some("2"),
        sort_order: Some("3"),
        target_pane: Some(&TargetPane::Raw("4")),
        template: Some("5"),
    };
    tmux.choose_buffer(Some(&choose_buffer)).unwrap_err();

    let choose_buffer = ChooseBufferBuilder::new()
        .no_preview()
        .format("1")
        .filter("2")
        .sort_order("3")
        .target_pane(&TargetPane::Raw("4"))
        .template("5")
        .build();
    tmux.choose_buffer(Some(&choose_buffer)).unwrap_err();
}
