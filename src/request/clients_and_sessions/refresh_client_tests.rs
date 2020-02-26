#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn refresh_client() {
    use crate::{Error, RefreshClient, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client]
        // [adjustment]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["refresh-client", "-c", "-D", "-l", "-L", "-R", "-S", "-U", "-C", "1x2", "-F", "3", "-t", "4", "5"]"#
        );
        Err(Error::Hook)
    }));
    let refresh_client = RefreshClient {
        tracking_cursor: Some(true),
        down: Some(true),
        request_clipboard: Some(true),
        left: Some(true),
        right: Some(true),
        status_line: Some(true),
        up: Some(true),
        size: Some((1, 2)),
        flags: Some("3"),
        target_client: Some("4"),
        adjustment: Some(5),
    };
    tmux.refresh_client(Some(&refresh_client)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn refresh_client() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux refresh-client [-C width,height] [-S] [-t target-client]
        // (alias: refresh)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["refresh-client", "-C", "1,2", "-S", "-t", "3"]"#
        );
        Err(Error::Hook)
    }));
    tmux.refresh_client(Some((1, 2)), Some(true), Some("3"))
        .unwrap_err();
}
