#[test]
fn resize_pane() {
    use crate::{Error, ResizePane, ResizePaneBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.1:
        // ```text
        // tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
        // (alias: resizep)
        // ```
        //
        // tmux ^1.8:
        // ```text
        // tmux resize-pane [-DLRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
        // (alias: resizep)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux resize-pane [-DLRU] [-t target-pane] [adjustment]
        // (alias: resizep)
        // ```
        //
        // tmux ^0.9:
        // ```text
        // tmux resize-pane [-DU] [-p pane-index] [-t target-pane] [adjustment]
        // (alias: resizep)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["resize-pane", "-D", "-L", "-M", "-R", "-U", "-Z", "-t", "1", "-x", "2", "-y", "3", "4"]"#
        );
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("resize-pane");
        #[cfg(feature = "tmux_0_9")]
        s.push("-D");
        #[cfg(feature = "tmux_1_8")]
        s.push("-L");
        #[cfg(feature = "tmux_2_1")]
        s.push("-M");
        #[cfg(feature = "tmux_1_8")]
        s.push("-R");
        #[cfg(feature = "tmux_0_9")]
        s.push("-U");
        #[cfg(feature = "tmux_1_8")]
        s.push("-Z");
        #[cfg(feature = "tmux_0_9")]
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(feature = "tmux_1_8")]
        s.extend_from_slice(&["-x", "2"]);
        #[cfg(feature = "tmux_1_8")]
        s.extend_from_slice(&["-y", "3"]);
        #[cfg(feature = "tmux_0_9")]
        s.push("4");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let resize_pane = ResizePane {
        #[cfg(feature = "tmux_0_9")]
        down: Some(true),
        #[cfg(feature = "tmux_1_8")]
        left: Some(true),
        #[cfg(feature = "tmux_2_1")]
        mouse: Some(true),
        #[cfg(feature = "tmux_1_8")]
        right: Some(true),
        #[cfg(feature = "tmux_0_9")]
        up: Some(true),
        #[cfg(feature = "tmux_1_8")]
        zoom: Some(true),
        #[cfg(feature = "tmux_0_9")]
        target_pane: Some(&TargetPane::Raw("1")),
        #[cfg(feature = "tmux_1_8")]
        width: Some(2),
        #[cfg(feature = "tmux_1_8")]
        height: Some(3),
        #[cfg(feature = "tmux_0_9")]
        adjustment: Some("4"),
    };
    tmux.resize_pane(Some(&resize_pane)).unwrap_err();

    let mut builder = ResizePaneBuilder::new();
    #[cfg(feature = "tmux_0_9")]
    builder.down();
    #[cfg(feature = "tmux_1_8")]
    builder.left();
    #[cfg(feature = "tmux_2_1")]
    builder.mouse();
    #[cfg(feature = "tmux_1_8")]
    builder.right();
    #[cfg(feature = "tmux_0_9")]
    builder.up();
    #[cfg(feature = "tmux_1_8")]
    builder.zoom();
    #[cfg(feature = "tmux_0_9")]
    builder.target_pane(&TargetPane::Raw("1"));
    #[cfg(feature = "tmux_1_8")]
    builder.width(2);
    #[cfg(feature = "tmux_1_8")]
    builder.height(3);
    #[cfg(feature = "tmux_0_9")]
    builder.adjustment("4");
    let resize_pane = builder.build();
    tmux.resize_pane(Some(&resize_pane)).unwrap_err();
}
