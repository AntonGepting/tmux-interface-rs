#[test]
fn save_buffer() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux save-buffer [-a] [-b buffer-name] path
        // (alias: saveb)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("save-buffer");
        #[cfg(feature = "use_cmd_alias")]
        s.push("saveb");
        s.push("-a");
        s.extend_from_slice(&["-b", "1"]);
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.save_buffer(Some(true), Some("1"), "2").unwrap_err();
}
