#[test]
fn last_pane() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux last-pane [-deZ] [-t target-window]
        // (alias: lastp)
        // ```
        //
        // tmux ^2.0:
        // ```text
        // tmux last-pane [-de] [-t target-window]
        // (alias: lastp)
        // ```
        //
        // tmux ^1.4:
        // ```text
        // tmux last-pane [-t target-window]
        // (alias: lastp)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("last-pane");
        #[cfg(feature = "use_cmd_alias")]
        s.push("lastp");
        #[cfg(feature = "tmux_2_0")]
        s.push("-d");
        #[cfg(feature = "tmux_2_0")]
        s.push("-e");
        #[cfg(feature = "tmux_3_1")]
        s.push("-Z");
        #[cfg(feature = "tmux_1_4")]
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_window = TargetWindow::Raw("1").to_string();
    tmux.last_pane(Some(true), Some(true), Some(true), Some(&target_window))
        .unwrap_err();
}
