#[test]
fn list_commands() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-commands [-F format]
        // (alias: lscm)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("list-commands");
        #[cfg(feature = "use_cmd_alias")]
        s.push("lscm");
        s.extend_from_slice(&["-F", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.list_commands(Some("1")).unwrap_err();
}
