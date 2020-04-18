// TODO: size and percentage both test
#[test]
fn join_pane() {
    use crate::{Error, JoinPane, JoinPaneBuilder, PaneSize, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
        // (alias: joinp)
        // tmux ^3.1:
        // ```text
        // tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
        // (alias: joinp)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
        // (alias: joinp)
        // ```
        //
        // tmux ^1.2:
        // ```text
        // tmux join-pane [-dhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
        // (alias: joinp)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("join-pane");
        #[cfg(feature = "tmux_2_6")]
        s.push("-b");
        #[cfg(feature = "tmux_1_2")]
        s.push("-d");
        #[cfg(feature = "tmux_2_6")]
        s.push("-f");
        #[cfg(feature = "tmux_1_2")]
        s.push("-h");
        #[cfg(feature = "tmux_1_2")]
        s.push("-v");
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_1")))]
        s.extend_from_slice(&["-l", "1%"]);
        #[cfg(feature = "tmux_3_1")]
        s.extend_from_slice(&["-p", "1"]);
        #[cfg(feature = "tmux_1_2")]
        s.extend_from_slice(&["-s", "2"]);
        #[cfg(feature = "tmux_1_2")]
        s.extend_from_slice(&["-t", "3"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let join_pane = JoinPane {
        #[cfg(feature = "tmux_2_6")]
        left_above: Some(true),
        #[cfg(feature = "tmux_1_2")]
        detached: Some(true),
        #[cfg(feature = "tmux_2_6")]
        full_size: Some(true),
        #[cfg(feature = "tmux_1_2")]
        horizontal: Some(true),
        #[cfg(feature = "tmux_1_2")]
        vertical: Some(true),
        #[cfg(feature = "tmux_1_2")]
        size: Some(&PaneSize::Percentage(1)),
        #[cfg(feature = "tmux_1_2")]
        src_pane: Some(&TargetPane::Raw("2")),
        #[cfg(feature = "tmux_1_2")]
        dst_pane: Some(&TargetPane::Raw("3")),
    };
    tmux.join_pane(Some(&join_pane)).unwrap_err();

    let mut builder = JoinPaneBuilder::new();
    #[cfg(feature = "tmux_1_2")]
    builder.left_above();
    #[cfg(feature = "tmux_1_2")]
    builder.detached();
    #[cfg(feature = "tmux_1_2")]
    builder.full_size();
    #[cfg(feature = "tmux_1_2")]
    builder.horizontal();
    #[cfg(feature = "tmux_1_2")]
    builder.vertical();
    #[cfg(feature = "tmux_1_2")]
    builder.size(&PaneSize::Percentage(1));
    #[cfg(feature = "tmux_1_2")]
    builder.src_pane(&TargetPane::Raw("2"));
    #[cfg(feature = "tmux_1_2")]
    builder.dst_pane(&TargetPane::Raw("3"));
    builder.build();
    tmux.join_pane(Some(&join_pane)).unwrap_err();
}
