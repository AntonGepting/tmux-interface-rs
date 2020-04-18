#[test]
fn select_pane() {
    use crate::{Error, SelectPane, SelectPaneBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
        // (alias: selectp)
        // ```
        //
        // tmux ^2.6:
        // ```text
        // tmux select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
        // (alias: selectp)
        // ```
        //
        // tmux ^2.1:
        // ```text
        // tmux select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
        // (alias: selectp)
        // ```
        //
        // tmux ^2.0:
        // ```text
        // tmux select-pane [-DdeLlRU] [-t target-pane]
        // (alias: selectp)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux select-pane [-DLlRU] [-t target-pane]
        // (alias: selectp)
        // ```
        //
        // tmux ^1.3:
        // ```text
        // tmux select-pane [-DLRU] [-t target-pane]
        // (alias: selectp)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux select-pane [-t target-pane]
        // (alias: selectp)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux select-pane [-p pane-index] [-t target-window]
        // (alias: selectp)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("select-pane");
        #[cfg(feature = "tmux_1_3")]
        s.push("-D");
        #[cfg(feature = "tmux_2_0")]
        s.push("-d");
        #[cfg(feature = "tmux_2_0")]
        s.push("-e");
        #[cfg(feature = "tmux_2_1")]
        s.push("-g");
        #[cfg(feature = "tmux_1_3")]
        s.push("-L");
        #[cfg(feature = "tmux_1_5")]
        s.push("-l");
        #[cfg(feature = "tmux_2_1")]
        s.push("-M");
        #[cfg(feature = "tmux_2_1")]
        s.push("-m");
        #[cfg(feature = "tmux_1_3")]
        s.push("-R");
        #[cfg(feature = "tmux_1_3")]
        s.push("-U");
        #[cfg(feature = "tmux_3_1")]
        s.push("-Z");
        #[cfg(feature = "tmux_2_1")]
        s.extend_from_slice(&["-P", "1"]);
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-T", "2"]);
        #[cfg(feature = "tmux_1_0")]
        s.extend_from_slice(&["-t", "3"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let select_pane = SelectPane {
        #[cfg(feature = "tmux_1_3")]
        down: Some(true),
        #[cfg(feature = "tmux_2_0")]
        disable: Some(true),
        #[cfg(feature = "tmux_2_0")]
        enable: Some(true),
        #[cfg(feature = "tmux_2_1")]
        show_style: Some(true),
        #[cfg(feature = "tmux_1_3")]
        left: Some(true),
        #[cfg(feature = "tmux_1_5")]
        last: Some(true),
        #[cfg(feature = "tmux_2_1")]
        set_marked: Some(true),
        #[cfg(feature = "tmux_2_1")]
        clear_marked: Some(true),
        #[cfg(feature = "tmux_1_3")]
        right: Some(true),
        #[cfg(feature = "tmux_1_3")]
        up: Some(true),
        #[cfg(feature = "tmux_3_1")]
        keep_zoomed: Some(true),
        #[cfg(feature = "tmux_2_1")]
        style: Some("1"),
        #[cfg(feature = "tmux_2_6")]
        title: Some("2"),
        #[cfg(feature = "tmux_1_0")]
        target_pane: Some(&TargetPane::Raw("3")),
    };
    tmux.select_pane(Some(&select_pane)).unwrap_err();

    let mut builder = SelectPaneBuilder::new();
    #[cfg(feature = "tmux_1_3")]
    builder.down();
    #[cfg(feature = "tmux_2_0")]
    builder.disable();
    #[cfg(feature = "tmux_2_0")]
    builder.enable();
    #[cfg(feature = "tmux_2_1")]
    builder.show_style();
    #[cfg(feature = "tmux_1_3")]
    builder.left();
    #[cfg(feature = "tmux_1_5")]
    builder.last();
    #[cfg(feature = "tmux_2_1")]
    builder.set_marked();
    #[cfg(feature = "tmux_2_1")]
    builder.clear_marked();
    #[cfg(feature = "tmux_1_3")]
    builder.right();
    #[cfg(feature = "tmux_1_3")]
    builder.up();
    #[cfg(feature = "tmux_3_1")]
    builder.keep_zoomed();
    #[cfg(feature = "tmux_2_1")]
    builder.style("1");
    #[cfg(feature = "tmux_2_6")]
    builder.title("2");
    #[cfg(feature = "tmux_1_0")]
    builder.target_pane(&TargetPane::Raw("3"));
    let select_pane = builder.build();
    tmux.select_pane(Some(&select_pane)).unwrap_err();
}
