#[test]
fn refresh_client() {
    use crate::{Error, RefreshClient, RefreshClientBuilder, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux 3.0:
        // ```text
        // tmux refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client] [adjustment]
        // (alias: refresh)
        // ```
        //
        // tmux 2.9a:
        // ```text
        // tmux refresh-client [-cDlLRSU] [-C width,height] [-F flags] [-t target-client] [adjustment]
        // (alias: refresh)
        // ```
        //
        // tmux 2.4:
        // ```text
        // tmux refresh-client [-C width,height] [-S] [-t target-client]
        // (alias: refresh)
        // ```
        //
        // tmux 1.6:
        // ```text
        // tmux refresh-client [-S] [-t target-client]
        // (alias: refresh)
        // ```
        //
        // tmux 0.8:
        // ```text
        // tmux refresh-client [-t target-client]
        // (alias: refresh)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("refresh-client");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-c");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-D");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-l");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-L");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-R");
        #[cfg(feature = "tmux_1_6")]
        s.push("-S");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-U");
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-C", "1x2"]);
        #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_0")))]
        s.extend_from_slice(&["-C", "1,2"]);
        #[cfg(feature = "tmux_2_9a")]
        s.extend_from_slice(&["-F", "3"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "4"]);
        #[cfg(feature = "tmux_2_9a")]
        s.push("5");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let refresh_client = RefreshClient {
        #[cfg(feature = "tmux_2_9a")]
        tracking_cursor: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        down: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        request_clipboard: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        left: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        right: Some(true),
        #[cfg(feature = "tmux_1_6")]
        status_line: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        up: Some(true),
        #[cfg(feature = "tmux_2_4")]
        size: Some((1, 2)),
        #[cfg(feature = "tmux_2_9a")]
        flags: Some("3"),
        #[cfg(feature = "tmux_0_8")]
        target_client: Some("4"),
        #[cfg(feature = "tmux_2_9a")]
        adjustment: Some(5),
    };
    tmux.refresh_client(Some(&refresh_client)).unwrap_err();

    let mut builder = RefreshClientBuilder::new();
    #[cfg(feature = "tmux_2_9a")]
    builder.tracking_cursor();
    #[cfg(feature = "tmux_2_9a")]
    builder.down();
    #[cfg(feature = "tmux_2_9a")]
    builder.request_clipboard();
    #[cfg(feature = "tmux_2_9a")]
    builder.left();
    #[cfg(feature = "tmux_2_9a")]
    builder.right();
    #[cfg(feature = "tmux_1_6")]
    builder.status_line();
    #[cfg(feature = "tmux_2_9a")]
    builder.up();
    #[cfg(feature = "tmux_2_4")]
    builder.size((1, 2));
    #[cfg(feature = "tmux_2_9a")]
    builder.flags("3");
    #[cfg(feature = "tmux_0_8")]
    builder.target_client("4");
    #[cfg(feature = "tmux_2_9a")]
    builder.adjustment(5);
    let refresh_client = builder.build();
    tmux.refresh_client(Some(&refresh_client)).unwrap_err();
}
