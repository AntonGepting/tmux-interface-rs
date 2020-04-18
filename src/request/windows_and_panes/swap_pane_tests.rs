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
        s.push("swap-pane");
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

    let swap_pane = SwapPane {
        #[cfg(feature = "tmux_0_8")]
        detached: Some(true),
        #[cfg(feature = "tmux_0_8")]
        previous: Some(true),
        #[cfg(feature = "tmux_0_8")]
        next: Some(true),
        #[cfg(feature = "tmux_3_1")]
        keep_zoomed: Some(true),
        #[cfg(feature = "tmux_1_0")]
        src_pane: Some(&TargetPane::Raw("1")),
        #[cfg(feature = "tmux_1_0")]
        dst_pane: Some(&TargetPane::Raw("2")),
    };
    tmux.swap_pane(Some(&swap_pane)).unwrap_err();

    let mut builder = SwapPaneBuilder::new();
    #[cfg(feature = "tmux_0_8")]
    builder.detached();
    #[cfg(feature = "tmux_0_8")]
    builder.previous();
    #[cfg(feature = "tmux_0_8")]
    builder.next();
    #[cfg(feature = "tmux_3_1")]
    builder.keep_zoomed();
    #[cfg(feature = "tmux_1_0")]
    builder.src_pane(&TargetPane::Raw("1"));
    #[cfg(feature = "tmux_1_0")]
    builder.dst_pane(&TargetPane::Raw("2"));
    let swap_pane = builder.build();
    tmux.swap_pane(Some(&swap_pane)).unwrap_err();
}
