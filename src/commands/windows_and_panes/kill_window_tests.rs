#[test]
fn kill_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.7:
        // ```text
        // tmux kill-window [-a] [-t target-window]
        // (alias: killw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux kill-window [-t target-window]
        // (alias: killw)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("kill-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("killw");
        #[cfg(feature = "tmux_1_7")]
        s.push("-a");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_window = TargetWindow::Raw("1").to_string();
    tmux.kill_window(Some(true), Some(&target_window))
        .unwrap_err();
}
