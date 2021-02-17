#[test]
fn display_panes() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.9:
        // ```text
        // tmux display-panes [-b] [-d duration] [-t target-client] [template]
        // (alias: displayp)
        // ```
        //
        // tmux ^2.6:
        // ```text
        // tmux display-panes [-d duration] [-t target-client] [template]
        // (alias: displayp)
        // ```
        //
        // tmux ^2.3:
        // ```text
        // tmux display-panes [-t target-client] [template]
        // (alias: displayp)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux display-panes [-t target-client]
        // (alias: displayp)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("display-panes");
        #[cfg(feature = "use_cmd_alias")]
        s.push("displayp");
        #[cfg(feature = "tmux_2_9")]
        s.push("-b");
        #[cfg(feature = "tmux_2_6")]
        s.extend_from_slice(&["-d", "1"]);
        s.extend_from_slice(&["-t", "2"]);
        s.push("3");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let _ = tmux.display_panes(Some(true), Some("1"), Some("2"), Some("3"));
}
