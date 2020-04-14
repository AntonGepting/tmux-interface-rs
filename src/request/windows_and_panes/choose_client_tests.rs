#[test]
fn choose_client() {
    use crate::{ChooseClient, ChooseClientBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-client", "-N", "-r", "-Z", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::Hook)
    }));

    let choose_client = ChooseClient {
        without_preview: Some(true),
        reverse_sort_order: Some(true),
        zoom: Some(true),
        format: Some("1"),
        filter: Some("2"),
        sort_order: Some("3"),
        target_pane: Some(&TargetPane::Raw("4")),
        template: Some("5"),
    };
    tmux.choose_client(Some(&choose_client)).unwrap_err();

    let choose_client = ChooseClientBuilder::new()
        .without_preview()
        .reverse_sort_order()
        .zoom()
        .format("1")
        .filter("2")
        .sort_order("3")
        .target_pane(&TargetPane::Raw("4"))
        .template("5")
        .build();
    tmux.choose_client(Some(&choose_client)).unwrap_err();
}
