#[test]
fn wait_for() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux wait-for [-L | -S | -U] channel
        // (alias: wait)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("wait-for");
        #[cfg(feature = "use_cmd_alias")]
        s.push("wait");
        s.push("-L");
        s.push("-S");
        s.push("-U");
        s.push("1");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.wait_for(Some(true), Some(true), Some(true), "1")
        .unwrap_err();
}
