#[test]
fn unbind_key() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux unbind-key [-an] [-T key-table] key
        // (alias: unbind)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("unbind-key");
        #[cfg(feature = "use_cmd_alias")]
        s.push("unbind");
        s.push("-a");
        s.push("-n");
        s.extend_from_slice(&["-T", "1"]);
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.unbind_key(Some(true), Some(true), Some("1"), "2")
        .unwrap_err();
}
