#[test]
fn last_window() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^0.8:
        // ```text
        // tmux last-window [-t target-session]
        // (alias: last)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("last-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("last");
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.last_window(Some("1")).unwrap_err();
}
