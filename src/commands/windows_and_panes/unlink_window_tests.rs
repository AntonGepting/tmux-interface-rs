#[test]
fn unlink_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.0:
        // ```text
        // tmux unlink-window [-k] [-t target-window]
        // (alias: unlinkw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux unlink-window [-t target-window]
        // (alias: unlinkw)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("unlink-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("unlinkw");
        #[cfg(feature = "tmux_0_8")]
        s.push("-k");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_window = TargetWindow::Raw("1").to_string();
    tmux.unlink_window(Some(true), Some(&target_window))
        .unwrap_err();
}
