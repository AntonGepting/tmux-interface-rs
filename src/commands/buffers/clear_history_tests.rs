#[test]
fn clear_history() {
    use crate::{Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux clear-history [-t target-pane]
        // (alias: clearhist)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("clear-history");
        #[cfg(feature = "use_cmd_alias")]
        s.push("clearhist");
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_pane = TargetPane::Raw("1").to_string();
    tmux.clear_history(Some(&target_pane)).unwrap_err();
}
