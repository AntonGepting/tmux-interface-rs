#[test]
fn break_pane() {
    #[cfg(any(
        all(feature = "tmux_0_8", not(feature = "tmux_1_7")),
        feature = "tmux_2_2"
    ))]
    use crate::TargetWindow;
    use crate::{BreakPane, BreakPaneBuilder, Error, TargetPane, TmuxInterface};
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    use std::marker::PhantomData;

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.4:
        // ```text
        // tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
        // (alias: breakp)
        // ```
        //
        // tmux ^2.2:
        // ```text
        // tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
        // (alias: breakp)
        // ```
        //
        // tmux ^2.1:
        // ```text
        // tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
        // (alias: breakp)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux break-pane [-dP] [-F format] [-t target-pane]
        // (alias: breakp)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux break-pane [-d] [-t target-window]
        // (alias: breakp)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux break-pane [-d] [-p pane-index] [-t target-window]
        // (alias: breakp)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("break-pane");
        #[cfg(feature = "tmux_0_8")]
        s.push("-d");
        #[cfg(feature = "tmux_1_7")]
        s.push("-P");
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-F", "1"]);
        #[cfg(feature = "tmux_2_4")]
        s.extend_from_slice(&["-n", "2"]);
        #[cfg(feature = "tmux_2_1")]
        s.extend_from_slice(&["-s", "3"]);
        #[cfg(feature = "tmux_2_2")]
        s.extend_from_slice(&["-t", "4"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let break_pane: BreakPane<TargetPane, _> = BreakPane {
        #[cfg(feature = "tmux_0_8")]
        detached: Some(true),
        #[cfg(feature = "tmux_1_7")]
        print: Some(true),
        #[cfg(feature = "tmux_1_7")]
        format: Some("1"),
        #[cfg(feature = "tmux_2_4")]
        window_name: Some("2"),
        #[cfg(feature = "tmux_2_1")]
        src_pane: Some(&TargetPane::Raw("3")),
        #[cfg(feature = "tmux_2_2")]
        dst_window: Some(&TargetWindow::Raw("4")),
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        dst_pane: Some(&TargetPane::Raw("4")),
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        target_pane: Some(&TargetPane::Raw("3")),
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
        target_window: Some(&TargetWindow::Raw("4")),
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
        _phantom: PhantomData,
        //_phantom2: PhantomData,
    };
    tmux.break_pane(Some(&break_pane)).unwrap_err();

    let mut builder: BreakPaneBuilder<TargetPane, _> = BreakPaneBuilder::new();
    #[cfg(feature = "tmux_0_8")]
    builder.detached();
    #[cfg(feature = "tmux_1_7")]
    builder.print();
    #[cfg(feature = "tmux_1_7")]
    builder.format("1");
    #[cfg(feature = "tmux_2_4")]
    builder.window_name("2");
    #[cfg(feature = "tmux_2_1")]
    builder.src_pane(&TargetPane::Raw("3"));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    builder.dst_pane(&TargetPane::Raw("4"));
    #[cfg(feature = "tmux_2_2")]
    builder.dst_window(&TargetWindow::Raw("4"));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    builder.target_pane(&TargetPane::Raw("3"));
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    builder.target_window(&TargetWindow::Raw("4"));
    let break_pane = builder.build();
    tmux.break_pane(Some(&break_pane)).unwrap_err();
}
