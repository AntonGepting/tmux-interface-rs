#[test]
fn send_keys() {
    use crate::{Error, SendKeys, SendKeysBuilder, TargetPaneExt, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
        // (alias: send)
        // ```
        //
        // tmux ^3.0:
        // ```text
        // tmux send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
        // (alias: send)
        // ```
        //
        // tmux ^2.4:
        // ```text
        // tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
        // (alias: send)
        // ```
        //
        // tmux ^2.1:
        // ```text
        // tmux send-keys [-lMR] [-t target-pane] key ...
        // (alias: send)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux send-keys [-lR] [-t target-pane] key ...
        // (alias: send)
        // ```
        //
        // tmux ^1.6:
        // ```text
        // tmux send-keys [-R] [-t target-pane] key ...
        // (alias: send)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux send-keys [-t target-window] key ...
        // (alias: send)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("send-keys");
        #[cfg(feature = "use_cmd_alias")]
        s.push("send");
        #[cfg(feature = "tmux_3_1")]
        s.push("-F");
        #[cfg(feature = "tmux_3_0")]
        s.push("-H");
        #[cfg(feature = "tmux_1_7")]
        s.push("-l");
        #[cfg(feature = "tmux_2_1")]
        s.push("-M");
        #[cfg(feature = "tmux_1_6")]
        s.push("-R");
        #[cfg(feature = "tmux_2_4")]
        s.push("-X");
        #[cfg(feature = "tmux_2_4")]
        s.extend_from_slice(&["-N", "1"]);
        #[cfg(feature = "tmux_1_6")]
        s.extend_from_slice(&["-t", "2"]);
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
        s.extend_from_slice(&["-t", "2"]);
        s.push("3");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_pane = TargetPaneExt::raw("2").to_string();
    let send_keys = SendKeys {
        #[cfg(feature = "tmux_3_1")]
        expand_formats: Some(true),
        #[cfg(feature = "tmux_3_0")]
        hex: Some(true),
        #[cfg(feature = "tmux_1_7")]
        disable_lookup: Some(true),
        #[cfg(feature = "tmux_2_1")]
        mouse_event: Some(true),
        #[cfg(feature = "tmux_1_6")]
        copy_mode: Some(true),
        #[cfg(feature = "tmux_2_4")]
        reset: Some(true),
        #[cfg(feature = "tmux_2_4")]
        repeat_count: Some(1),
        #[cfg(feature = "tmux_1_6")]
        target_pane: Some(&target_pane),
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
        target_window: Some(&target_pane),
    };
    tmux.send_keys(Some(&send_keys), &vec!["3"]).unwrap_err();
    //tmux.send_keys(None, &vec!["3"]);

    let mut builder = SendKeysBuilder::new();
    #[cfg(feature = "tmux_3_1")]
    builder.expand_formats();
    #[cfg(feature = "tmux_3_0")]
    builder.hex();
    #[cfg(feature = "tmux_1_7")]
    builder.disable_lookup();
    #[cfg(feature = "tmux_2_1")]
    builder.mouse_event();
    #[cfg(feature = "tmux_1_6")]
    builder.copy_mode();
    #[cfg(feature = "tmux_2_4")]
    builder.reset();
    #[cfg(feature = "tmux_2_4")]
    builder.repeat_count(1);
    #[cfg(feature = "tmux_1_6")]
    builder.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    builder.target_window(&target_pane);
    let send_keys = builder.build();
    tmux.send_keys(Some(&send_keys), &vec!["3"]).unwrap_err();
}
