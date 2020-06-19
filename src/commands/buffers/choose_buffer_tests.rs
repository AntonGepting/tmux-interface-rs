#[test]
fn choose_buffer() {
    use crate::{ChooseBuffer, ChooseBufferBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // # Manual
        //
        // tmux X.X:
        // ```text
        // tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        // ```
        //
        // tmux ^3.1:
        // ```text
        // tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        // ```
        //
        // tmux ^2.7:
        // ```text
        // tmux choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        // ```
        //
        // tmux ^2.6:
        // ```text
        // tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux choose-buffer [-F format] [-t target-pane] [template]
        // ```
        //
        // tmux ^1.3:
        // ```text
        // tmux choose-buffer [-t target-pane] [template]
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("choose-buffer");
        #[cfg(feature = "tmux_2_6")]
        s.push("-N");
        #[cfg(feature = "tmux_2_7")]
        s.push("-Z");
        #[cfg(feature = "tmux_3_1")]
        s.push("-r");
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-F", "1"]);
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-f", "2"]);
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-O", "3"]);
        #[cfg(feature = "tmux_1_3")]
        s.extend_from_slice(&["-t", "4"]);
        #[cfg(feature = "tmux_1_3")]
        s.push("5");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_pane = TargetPane::Raw("4").to_string();
    let choose_buffer = ChooseBuffer {
        #[cfg(feature = "tmux_2_6")]
        no_preview: Some(true),
        #[cfg(feature = "tmux_2_7")]
        zoom: Some(true),
        #[cfg(feature = "tmux_3_1")]
        reverse_sort_order: Some(true),
        #[cfg(feature = "tmux_1_7")]
        format: Some("1"),
        #[cfg(feature = "tmux_2_6")]
        filter: Some("2"),
        #[cfg(feature = "tmux_2_6")]
        sort_order: Some("3"),
        #[cfg(feature = "tmux_1_3")]
        target_pane: Some(&target_pane),
        #[cfg(feature = "tmux_1_3")]
        template: Some("5"),
    };
    tmux.choose_buffer(Some(&choose_buffer)).unwrap_err();

    let mut builder = ChooseBufferBuilder::new();
    #[cfg(feature = "tmux_2_6")]
    builder.no_preview();
    #[cfg(feature = "tmux_2_7")]
    builder.zoom();
    #[cfg(feature = "tmux_3_1")]
    builder.reverse_sort_order();
    #[cfg(feature = "tmux_1_7")]
    builder.format("1");
    #[cfg(feature = "tmux_2_6")]
    builder.filter("2");
    #[cfg(feature = "tmux_2_6")]
    builder.sort_order("3");
    #[cfg(feature = "tmux_1_3")]
    builder.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_3")]
    builder.template("5");
    let choose_buffer = builder.build();
    tmux.choose_buffer(Some(&choose_buffer)).unwrap_err();
}
