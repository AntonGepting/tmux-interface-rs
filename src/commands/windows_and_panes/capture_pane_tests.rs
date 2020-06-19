#[test]
fn capture_pane() {
    use crate::{CapturePane, CapturePaneBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
        // (alias: capturep)
        // ```
        //
        // tmux ^2.4:
        // ```text
        // tmux capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
        // (alias: capturep)
        // ```
        //
        // tmux ^1.8:
        // ```text
        // tmux capture-pane [-aepPq] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
        // (alias: capturep)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux capture-pane [-b buffer-index] [-E end-line] [-S start-line] [-t target-pane]
        // (alias: capturep)
        // ```
        //
        // tmux ^1.2:
        // ```text
        // tmux capture-pane [-b buffer-index] [-t target-pane]
        // (alias: capturep)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("capture-pane");
        #[cfg(feature = "tmux_1_8")]
        s.push("-a");
        #[cfg(feature = "tmux_1_8")]
        s.push("-e");
        #[cfg(feature = "tmux_1_8")]
        s.push("-p");
        #[cfg(feature = "tmux_1_8")]
        s.push("-P");
        #[cfg(feature = "tmux_1_8")]
        s.push("-q");
        #[cfg(feature = "tmux_2_4")]
        s.push("-C");
        #[cfg(feature = "tmux_2_4")]
        s.push("-J");
        #[cfg(feature = "tmux_3_1")]
        s.push("-N");
        #[cfg(feature = "tmux_1_8")]
        s.extend_from_slice(&["-b", "1"]);
        #[cfg(feature = "tmux_1_5")]
        s.extend_from_slice(&["-E", "2"]);
        #[cfg(feature = "tmux_1_5")]
        s.extend_from_slice(&["-S", "3"]);
        #[cfg(feature = "tmux_1_2")]
        s.extend_from_slice(&["-t", "4"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    #[cfg(feature = "tmux_1_2")]
    let target_pane = TargetPane::Raw("4").to_string();

    let capture_pane = CapturePane {
        #[cfg(feature = "tmux_1_8")]
        alternate_screen: Some(true),
        #[cfg(feature = "tmux_1_8")]
        escape_sequences: Some(true),
        #[cfg(feature = "tmux_1_8")]
        stdout: Some(true),
        #[cfg(feature = "tmux_1_8")]
        pane: Some(true),
        #[cfg(feature = "tmux_1_8")]
        quite: Some(true),
        #[cfg(feature = "tmux_2_4")]
        escape_non_printable: Some(true),
        #[cfg(feature = "tmux_2_4")]
        join: Some(true),
        #[cfg(feature = "tmux_3_1")]
        trailing_spaces: Some(true),
        #[cfg(feature = "tmux_1_8")]
        buffer_name: Some("1"),
        #[cfg(feature = "tmux_1_5")]
        end_line: Some("2"),
        #[cfg(feature = "tmux_1_5")]
        start_line: Some("3"),
        #[cfg(feature = "tmux_1_2")]
        target_pane: Some(&target_pane),
    };
    tmux.capture_pane(Some(&capture_pane)).unwrap_err();

    let mut builder = CapturePaneBuilder::new();
    #[cfg(feature = "tmux_1_8")]
    builder.alternate_screen();
    #[cfg(feature = "tmux_1_8")]
    builder.escape_sequences();
    #[cfg(feature = "tmux_1_8")]
    builder.stdout();
    #[cfg(feature = "tmux_1_8")]
    builder.pane();
    #[cfg(feature = "tmux_1_8")]
    builder.quite();
    #[cfg(feature = "tmux_2_4")]
    builder.escape_non_printable();
    #[cfg(feature = "tmux_2_4")]
    builder.join();
    #[cfg(feature = "tmux_3_1")]
    builder.trailing_spaces();
    #[cfg(feature = "tmux_1_8")]
    builder.buffer_name("1");
    #[cfg(feature = "tmux_1_5")]
    builder.end_line("2");
    #[cfg(feature = "tmux_1_5")]
    builder.start_line("3");
    #[cfg(feature = "tmux_1_2")]
    builder.target_pane(&target_pane);
    let capture_pane = builder.build();
    tmux.capture_pane(Some(&capture_pane)).unwrap_err();
}
