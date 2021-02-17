#[test]
fn list_sessions() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-sessions [-F format]
        // (alias: ls)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("lsit-sessions");
        #[cfg(feature = "use_cmd_alias")]
        s.push("ls");
        s.extend_from_slice(&["-F", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.list_sessions(Some("1")).unwrap_err();
}
