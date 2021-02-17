#[test]
fn source_file() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux source-file [-nqv] path
        // (alias: source)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("source-file");
        #[cfg(feature = "use_cmd_alias")]
        s.push("source");
        s.push("-n");
        s.push("-q");
        s.extend_from_slice(&["-v", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.source_file(Some(true), Some(true), Some(true), "1")
        .unwrap_err();
}
