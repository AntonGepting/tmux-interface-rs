#[test]
fn show_messages() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux show-messages [-JT] [-t target-client]
        // (alias: showmsgs)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("show-messages");
        #[cfg(feature = "use_cmd_alias")]
        s.push("showmsgs");
        s.push("-J");
        s.push("-T");
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.show_messages(Some(true), Some(true), Some("1"))
        .unwrap_err();
}
