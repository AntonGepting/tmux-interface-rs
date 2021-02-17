#[test]
fn list_clients() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-clients [-F format] [-t target-session]
        // (alias: lsc)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("list-clients");
        #[cfg(feature = "use_cmd_alias")]
        s.push("lsc");
        s.extend_from_slice(&["-F", "1"]);
        s.extend_from_slice(&["-t", "2"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_session = TargetSession::Raw("2").to_string();
    tmux.list_clients(Some("1"), Some(&target_session))
        .unwrap_err();
}
