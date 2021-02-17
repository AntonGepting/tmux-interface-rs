#[test]
fn swap_pane() {
    use crate::{Error, SwapPane, SwapPaneBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
        // (alias: swapp)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
        // (alias: swapp)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
        // (alias: swapp)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("swap-pane");
        #[cfg(feature = "use_cmd_alias")]
        s.push("swapp");
        #[cfg(feature = "tmux_0_8")]
        s.push("-d");
        #[cfg(feature = "tmux_0_8")]
        s.push("-D");
        #[cfg(feature = "tmux_0_8")]
        s.push("-U");
        #[cfg(feature = "tmux_3_1")]
        s.push("-Z");
        #[cfg(feature = "tmux_1_0")]
        s.extend_from_slice(&["-s", "1"]);
        #[cfg(feature = "tmux_1_0")]
        s.extend_from_slice(&["-t", "2"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let src_pane = TargetPane::Raw("1").to_string();
    let dst_pane = TargetPane::Raw("2").to_string();

    let swap_pane = SwapPane {
        #[cfg(feature = "tmux_0_8")]
        detached: Some(true),
        #[cfg(feature = "tmux_0_8")]
        previous_pane: Some(true),
        #[cfg(feature = "tmux_0_8")]
        next_pane: Some(true),
        #[cfg(feature = "tmux_3_1")]
        keep_zoomed: Some(true),
        #[cfg(feature = "tmux_1_0")]
        src_pane: Some(&src_pane),
        #[cfg(feature = "tmux_1_0")]
        dst_pane: Some(&dst_pane),
    };
    tmux.swap_pane(Some(&swap_pane)).unwrap_err();

    let mut builder = SwapPaneBuilder::new();
    #[cfg(feature = "tmux_0_8")]
    builder.detached();
    #[cfg(feature = "tmux_0_8")]
    builder.previous_pane();
    #[cfg(feature = "tmux_0_8")]
    builder.next_pane();
    #[cfg(feature = "tmux_3_1")]
    builder.keep_zoomed();
    #[cfg(feature = "tmux_1_0")]
    builder.src_pane(&src_pane);
    #[cfg(feature = "tmux_1_0")]
    builder.dst_pane(&dst_pane);
    let swap_pane = builder.build();
    tmux.swap_pane(Some(&swap_pane)).unwrap_err();
}
