#[test]
fn select_layout() {
    use crate::{Error, SelectLayot, SelectLayotBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.7:
        // ```text
        // tmux select-layout [-Enop] [-t target-pane] [layout-name]
        // (alias: selectl)
        // ```
        //
        // tmux ^2.1:
        // ```text
        // tmux select-layout [-nop] [-t target-pane] [layout-name]
        // (alias: selectl)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux select-layout [-np] [-t target-pane] [layout-name]
        // (alias: selectl)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux select-layout [-t target-pane] [layout-name]
        // (alias: selectl)
        // ```
        //
        // tmux ^0.9:
        // ```text
        // tmux select-layout [-t target-pane] layout-name
        // (alias: selectl)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["select-layout", "-E", "-n", "-o", "-p", "-t", "1", "2"]"#
        );
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("select-layout");
        #[cfg(feature = "tmux_2_7")]
        s.push("-E");
        #[cfg(feature = "tmux_1_5")]
        s.push("-n");
        #[cfg(feature = "tmux_2_1")]
        s.push("-o");
        #[cfg(feature = "tmux_1_5")]
        s.push("-p");
        #[cfg(feature = "tmux_0_9")]
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(feature = "tmux_1_0")]
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let select_layout = SelectLayot {
        #[cfg(feature = "tmux_2_7")]
        spread: Some(true),
        #[cfg(feature = "tmux_1_5")]
        next_layout: Some(true),
        #[cfg(feature = "tmux_2_1")]
        last_layout: Some(true),
        #[cfg(feature = "tmux_1_5")]
        previous_layout: Some(true),
        #[cfg(feature = "tmux_0_9")]
        target_pane: Some(&TargetPane::Raw("1")),
        #[cfg(feature = "tmux_1_0")]
        layout_name: Some("2"),
    };
    tmux.select_layout(Some(&select_layout)).unwrap_err();

    let mut builder = SelectLayotBuilder::new();
    #[cfg(feature = "tmux_2_7")]
    builder.spread();
    #[cfg(feature = "tmux_1_5")]
    builder.next_layout();
    #[cfg(feature = "tmux_2_1")]
    builder.last_layout();
    #[cfg(feature = "tmux_1_5")]
    builder.previous_layout();
    #[cfg(feature = "tmux_0_9")]
    builder.target_pane(&TargetPane::Raw("1"));
    #[cfg(feature = "tmux_1_0")]
    builder.layout_name("2");
    let select_layout = builder.build();
    tmux.select_layout(Some(&select_layout)).unwrap_err();
}
