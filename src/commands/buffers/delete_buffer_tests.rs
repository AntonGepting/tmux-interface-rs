#[test]
fn delete_buffer() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux delete-buffer [-b buffer-name]
        // (alias: deleteb)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("delete-buffer");
        #[cfg(feature = "use_cmd_alias")]
        s.push("deleteb");
        s.extend_from_slice(&["-b", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.delete_buffer(Some("1")).unwrap_err();
}
