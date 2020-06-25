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
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["display-panes", "-b", "-d", "1", "-t", "2", "3"]"#
        );
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("display-panes");
        s.push("-b");
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