#[test]
fn rename_session() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux rename-session [-t target-session] new-name
        // (alias: rename)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("rename-session");
        #[cfg(feature = "use_cmd_alias")]
        s.push("rename");
        s.extend_from_slice(&["-t", "1"]);
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_session = TargetSession::Raw("1").to_string();
    tmux.rename_session(Some(&target_session), "2").unwrap_err();
}
