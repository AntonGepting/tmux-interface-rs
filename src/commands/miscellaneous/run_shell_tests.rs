#[test]
fn run_shell() {
    use crate::{Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux run-shell [-b] [-t target-pane] shell-command
        // (alias: run)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("run-shell");
        #[cfg(feature = "use_cmd_alias")]
        s.push("run");
        s.push("-b");
        s.extend_from_slice(&["-t", "1"]);
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_pane = TargetPane::Raw("1").to_string();
    tmux.run_shell(Some(true), Some(&target_pane), "2")
        .unwrap_err();
}
