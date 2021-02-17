#[test]
fn list_keys() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-keys [-N] [-T key-table]
        // (alias: lsk)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("list-keys");
        #[cfg(feature = "use_cmd_alias")]
        s.push("lsk");
        s.push("-N");
        s.extend_from_slice(&["-T", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.list_keys(Some(true), Some("1")).unwrap_err();
}
