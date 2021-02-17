#[test]
fn previous_window() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^0.9:
        // ```text
        // tmux previous-window [-a] [-t target-session]
        // (alias: prev)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux previous-window [-t target-session]
        // (alias: prev)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("previous-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("prev");
        #[cfg(feature = "tmux_0_9")]
        s.push("-a");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.previous_window(Some(true), Some("1")).unwrap_err();
}
