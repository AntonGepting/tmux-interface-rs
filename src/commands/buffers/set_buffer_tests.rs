#[test]
fn set_buffer() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
        // (alias: setb)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("set-buffer");
        #[cfg(feature = "use_cmd_alias")]
        s.push("setb");
        s.push("-a");
        s.extend_from_slice(&["-b", "1"]);
        s.extend_from_slice(&["-n", "2"]);
        s.push("3");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.set_buffer(Some(true), Some("1"), Some("2"), "3")
        .unwrap_err();
}
