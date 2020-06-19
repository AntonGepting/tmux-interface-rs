#[test]
fn paste_buffer() {
    use crate::{Error, PasteBuffer, PasteBufferBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.7:
        // ```text
        // tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
        // (alias: pasteb)
        // ```
        //
        // tmux ^1.3:
        // ```text
        // tmux paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-window]
        // (alias: pasteb)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux paste-buffer [-dr] [-b buffer-index] [-t target-window]
        // (alias: pasteb)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux paste-buffer [-d] [-b buffer-index] [-t target-window]
        // (alias: pasteb)
        // ```

        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("paste-buffer");
        #[cfg(feature = "tmux_0_8")]
        s.push("-d");
        #[cfg(feature = "tmux_1_7")]
        s.push("-p");
        #[cfg(feature = "tmux_1_0")]
        s.push("-r");
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-b", "1"]);
        #[cfg(feature = "tmux_1_3")]
        s.extend_from_slice(&["-s", "2"]);
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-t", "3"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_pane = TargetPane::Raw("3").to_string();
    let paste_buffer = PasteBuffer {
        #[cfg(feature = "tmux_0_8")]
        delete: Some(true),
        #[cfg(feature = "tmux_1_7")]
        bracket_codes: Some(true),
        #[cfg(feature = "tmux_1_0")]
        no_replacement: Some(true),
        #[cfg(feature = "tmux_1_7")]
        buffer_name: Some("1"),
        #[cfg(feature = "tmux_1_3")]
        separator: Some("2"),
        #[cfg(feature = "tmux_1_7")]
        target_pane: Some(&target_pane),
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
        target_window: Some(&target_pane),
    };
    tmux.paste_buffer(Some(&paste_buffer)).unwrap_err();

    let mut builder = PasteBufferBuilder::new();
    #[cfg(feature = "tmux_0_8")]
    builder.delete();
    #[cfg(feature = "tmux_1_7")]
    builder.bracket_codes();
    #[cfg(feature = "tmux_1_0")]
    builder.no_replacement();
    #[cfg(feature = "tmux_1_7")]
    builder.buffer_name("1");
    #[cfg(feature = "tmux_1_3")]
    builder.separator("2");
    #[cfg(feature = "tmux_1_7")]
    builder.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    builder.target_window(&target_pane);
    let paste_buffer = builder.build();
    tmux.paste_buffer(Some(&paste_buffer)).unwrap_err();
}
