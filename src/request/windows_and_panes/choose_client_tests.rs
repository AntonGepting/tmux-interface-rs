#[test]
fn choose_client() {
    #[cfg(feature = "tmux_2_6")]
    use crate::TargetPane;
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    use crate::TargetWindow;
    use crate::{ChooseClient, ChooseClientBuilder, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        // ```
        //
        // tmux ^2.7:
        // ```text
        // tmux choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        // ```
        //
        // tmux ^2.6:
        // ```text
        // tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux choose-client [-F format] [-t target-window] [template]
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux choose-client  [-t target-window] [template]
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("choose-client");
        #[cfg(feature = "tmux_2_6")]
        s.push("-N");
        #[cfg(feature = "tmux_3_1")]
        s.push("-r");
        #[cfg(feature = "tmux_3_1")]
        s.push("-Z");
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-F", "1"]);
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-f", "2"]);
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-O", "3"]);
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-t", "4"]);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
        s.extend_from_slice(&["-t", "4"]);
        #[cfg(feature = "tmux_1_0")]
        s.push("5");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let choose_client = ChooseClient {
        #[cfg(feature = "tmux_2_6")]
        without_preview: Some(true),
        #[cfg(feature = "tmux_3_1")]
        reverse_sort_order: Some(true),
        #[cfg(feature = "tmux_3_1")]
        zoom: Some(true),
        #[cfg(feature = "tmux_1_7")]
        format: Some("1"),
        #[cfg(feature = "tmux_2_6")]
        filter: Some("2"),
        #[cfg(feature = "tmux_2_6")]
        sort_order: Some("3"),
        #[cfg(feature = "tmux_2_6")]
        target_pane: Some(&TargetPane::Raw("4")),
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
        target_window: Some(&TargetWindow::Raw("2")),
        #[cfg(feature = "tmux_1_0")]
        template: Some("5"),
    };
    tmux.choose_client(Some(&choose_client)).unwrap_err();

    let mut builder = ChooseClientBuilder::new();
    #[cfg(feature = "tmux_2_6")]
    builder.without_preview();
    #[cfg(feature = "tmux_3_1")]
    builder.reverse_sort_order();
    #[cfg(feature = "tmux_3_1")]
    builder.zoom();
    #[cfg(feature = "tmux_1_7")]
    builder.format("1");
    #[cfg(feature = "tmux_2_6")]
    builder.filter("2");
    #[cfg(feature = "tmux_2_6")]
    builder.sort_order("3");
    #[cfg(feature = "tmux_2_6")]
    builder.target_pane(&TargetPane::Raw("4"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    builder.target_window(&TargetWindow::Raw("4"));
    #[cfg(feature = "tmux_1_0")]
    builder.template("5");
    let choose_client = builder.build();
    tmux.choose_client(Some(&choose_client)).unwrap_err();
}
