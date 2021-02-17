#[test]
fn load_buffer() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux load-buffer [-b buffer-name] path
        // (alias: loadb)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("load-buffer");
        #[cfg(feature = "use_cmd_alias")]
        s.push("loadb");
        s.extend_from_slice(&["-b", "1"]);
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.load_buffer(Some("1"), "2").unwrap_err();
}
