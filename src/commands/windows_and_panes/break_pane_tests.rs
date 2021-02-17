#[test]
fn break_pane() {
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    use crate::TargetWindow;
    use crate::{BreakPane, BreakPaneBuilder, Error, TargetPane, TmuxInterface};

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
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("break-pane");
        #[cfg(feature = "use_cmd_alias")]
        s.push("breakp");
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

    let src_pane = TargetPane::Raw("3").to_string();
    let dst_pane = TargetPane::Raw("4").to_string();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    let target_window = TargetWindow::Raw("4").to_string();

    let break_pane: BreakPane = BreakPane {
        #[cfg(feature = "tmux_0_8")]
        detached: Some(true),
        #[cfg(feature = "tmux_1_7")]
        print: Some(true),
        #[cfg(feature = "tmux_1_7")]
        format: Some("1"),
        #[cfg(feature = "tmux_2_4")]
        window_name: Some("2"),
        #[cfg(feature = "tmux_2_1")]
        src_pane: Some(&src_pane),
        #[cfg(feature = "tmux_2_2")]
        dst_window: Some(&dst_pane),
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        dst_pane: Some(&dst_pane),
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        target_pane: Some(&dst_pane),
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
        target_window: Some(&target_window),
    };
    tmux.break_pane(Some(&break_pane)).unwrap_err();

    let mut builder: BreakPaneBuilder = BreakPaneBuilder::new();
    #[cfg(feature = "tmux_0_8")]
    builder.detached();
    #[cfg(feature = "tmux_1_7")]
    builder.print();
    #[cfg(feature = "tmux_1_7")]
    builder.format("1");
    #[cfg(feature = "tmux_2_4")]
    builder.window_name("2");
    #[cfg(feature = "tmux_2_1")]
    builder.src_pane(&src_pane);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    builder.dst_pane(&dst_pane);
    #[cfg(feature = "tmux_2_2")]
    builder.dst_window(&dst_pane);
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    builder.target_pane(&dst_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    builder.target_window(&target_window);
    let break_pane = builder.build();
    tmux.break_pane(Some(&break_pane)).unwrap_err();
}
