#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn choose_client() {
    use crate::{ChooseClient, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-client", "-N", "-r", "-Z", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::new("hook"))
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
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn choose_client() {
    use crate::{ChooseClient, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-client", "-N", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::new("hook"))
    }));
    let choose_client = ChooseClient {
        without_preview: Some(true),
        format: Some("1"),
        filter: Some("2"),
        sort_order: Some("3"),
        target_pane: Some(&TargetPane::Raw("4")),
        template: Some("5"),
    };
    tmux.choose_client(Some(&choose_client)).unwrap_err();
}
