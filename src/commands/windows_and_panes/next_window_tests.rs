#[test]
fn next_window() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^0.9:
        // ```text
        // tmux next-window [-a] [-t target-session]
        // (alias: next)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux next-window [-t target-session]
        // (alias: next)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("next-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("next");
        #[cfg(feature = "tmux_0_9")]
        s.push("-a");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.next_window(Some(true), Some("1")).unwrap_err();
}
