#[test]
fn kill_pane() {
    use crate::{Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.1:
        // ```text
        // tmux kill-pane [-a] [-t target-pane]
        // (alias: killp)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux kill-pane [-t target-pane]
        // (alias: killp)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux kill-pane [-p pane-index] [-t target-window]
        // (alias: killp)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("kill-pane");
        #[cfg(feature = "use_cmd_alias")]
        s.push("killp");
        #[cfg(feature = "tmux_1_1")]
        s.push("-a");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_pane = TargetPane::Raw("1").to_string();
    tmux.kill_pane(Some(true), Some(&target_pane)).unwrap_err();
}
