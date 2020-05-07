#[test]
fn move_pane() {
    use crate::{Error, MovePane, MovePaneBuilder, PaneSize, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
        // (alias: movep)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
        // (alias: movep)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["move-pane", "-b", "-d", "-h", "-v", "-l", "1", "-s", "2", "-t", "3"]"#
        );
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("move-pane");
        #[cfg(feature = "tmux_1_7")]
        s.push("-b");
        #[cfg(feature = "tmux_1_7")]
        s.push("-d");
        #[cfg(feature = "tmux_1_7")]
        s.push("-h");
        #[cfg(feature = "tmux_1_7")]
        s.push("-v");
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-l", "1"]);
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-s", "2"]);
        #[cfg(feature = "tmux_1_7")]
        s.extend_from_slice(&["-t", "3"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let move_pane = MovePane {
        #[cfg(feature = "tmux_1_7")]
        left_above: Some(true),
        #[cfg(feature = "tmux_1_7")]
        detached: Some(true),
        #[cfg(feature = "tmux_1_7")]
        horizontal: Some(true),
        #[cfg(feature = "tmux_1_7")]
        vertical: Some(true),
        #[cfg(feature = "tmux_1_7")]
        size: Some(&PaneSize::Size(1)),
        #[cfg(feature = "tmux_1_7")]
        src_pane: Some(&TargetPane::Raw("2")),
        #[cfg(feature = "tmux_1_7")]
        dst_pane: Some(&TargetPane::Raw("3")),
    };
    tmux.move_pane(Some(&move_pane)).unwrap_err();

    let mut builder = MovePaneBuilder::new();
    #[cfg(feature = "tmux_1_7")]
    builder.left_above();
    #[cfg(feature = "tmux_1_7")]
    builder.detached();
    #[cfg(feature = "tmux_1_7")]
    builder.horizontal();
    #[cfg(feature = "tmux_1_7")]
    builder.vertical();
    #[cfg(feature = "tmux_1_7")]
    builder.size(&PaneSize::Size(1));
    #[cfg(feature = "tmux_1_7")]
    builder.src_pane(&TargetPane::Raw("2"));
    #[cfg(feature = "tmux_1_7")]
    builder.dst_pane(&TargetPane::Raw("3"));
    let move_pane = builder.build();
    tmux.move_pane(Some(&move_pane)).unwrap_err();
}
