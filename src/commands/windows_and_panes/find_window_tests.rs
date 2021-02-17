#[test]
fn find_window() {
    use crate::{Error, FindWindow, FindWindowBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // # Manual
        //
        // tmux ^3.0:
        // ```text
        // tmux find-window [-rCNTZ] [-t target-pane] match-string
        // (alias: findw)
        //
        // tmux ^2.6:
        // ```text
        // tmux find-window [-CNT] [-t target-pane] match-string
        // (alias: findw)
        //
        // tmux ^1.7:
        // ```text
        // tmux find-window [-CNT] [-F format] [-t target-pane] match-string
        // (alias: findw)
        //
        // tmux ^0.8:
        // ```text
        // tmux find-window [-t target-pane] match-string
        // (alias: findw)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("find-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("findw");
        #[cfg(feature = "tmux_3_0")]
        s.push("-r");
        #[cfg(feature = "tmux_1_7")]
        s.push("-C");
        #[cfg(feature = "tmux_1_7")]
        s.push("-N");
        #[cfg(feature = "tmux_1_7")]
        s.push("-T");
        #[cfg(feature = "tmux_3_0")]
        s.push("-Z");
        s.extend_from_slice(&["-t", "1"]);
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_pane = TargetPane::Raw("1").to_string();
    let find_window = FindWindow {
        #[cfg(feature = "tmux_3_0")]
        regex: Some(true),
        #[cfg(feature = "tmux_1_7")]
        only_visible: Some(true),
        #[cfg(feature = "tmux_1_7")]
        only_name: Some(true),
        #[cfg(feature = "tmux_1_7")]
        only_title: Some(true),
        #[cfg(feature = "tmux_3_0")]
        zoom: Some(true),
        target_pane: Some(&target_pane),
    };
    tmux.find_window(Some(&find_window), "2").unwrap_err();

    let mut builder = FindWindowBuilder::new();
    #[cfg(feature = "tmux_3_0")]
    builder.regex();
    #[cfg(feature = "tmux_1_7")]
    builder.only_visible();
    #[cfg(feature = "tmux_1_7")]
    builder.only_name();
    #[cfg(feature = "tmux_1_7")]
    builder.only_title();
    #[cfg(feature = "tmux_3_0")]
    builder.zoom();
    builder.target_pane(&target_pane);
    let find_window = builder.build();
    tmux.find_window(Some(&find_window), "2").unwrap_err();
}
